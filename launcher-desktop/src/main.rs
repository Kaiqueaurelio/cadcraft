#![deny(unsafe_code)]

mod catalog;
mod installer;
mod model;

use std::collections::{BTreeMap, HashMap};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use catalog::{APPS, DecodeApp};
use eframe::egui;
use installer::{ExternalInstall, InstalledRecord, InstallResult, Manifest, Paths};
use model::{HostPlatform, Release};

enum Event {
    Release { id: String, result: Result<Release, String> },
    Detected { id: String, result: Option<ExternalInstall> },
    Progress { id: String, text: String },
    Finished { id: String, result: Result<InstallResult, String> },
}

struct Launcher {
    tx: Sender<Event>,
    rx: Receiver<Event>,
    paths: Paths,
    manifest: Manifest,
    releases: BTreeMap<String, Release>,
    errors: HashMap<String, String>,
    detected: HashMap<String, ExternalInstall>,
    icons: HashMap<String, egui::TextureHandle>,
    tasks: HashMap<String, String>,
    results: HashMap<String, String>,
    search: String,
    category: String,
    sort_az: bool,
    refreshing: bool,
    refresh_left: usize,
    notice: Option<(String, bool)>,
    confirm_remove: Option<String>,
}

impl Launcher {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let paths = Paths::new().unwrap_or_else(|error| {
            eprintln!("Decode Open Source Launcher: {error}");
            let base = std::env::temp_dir().join("Decode Open Source Hub");
            Paths { data: base.join("data"), cache: base.join("cache") }
        });
        let manifest = installer::load_manifest(&paths);
        let releases = installer::load_release_cache(&paths.release_cache());
        let (tx, rx) = mpsc::channel();
        let icons: HashMap<String, egui::TextureHandle> = APPS
            .iter()
            .filter_map(|app| load_icon_texture(&cc.egui_ctx, app).map(|texture| (app.id.to_string(), texture)))
            .collect();
        let mut style = (*cc.egui_ctx.style_of(egui::Theme::Dark)).clone();
        style.visuals = egui::Visuals::dark();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        cc.egui_ctx.set_style_of(egui::Theme::Dark, style);
        let mut app = Self {
            tx, rx, paths, manifest, releases, errors: HashMap::new(),
            detected: HashMap::new(), icons, tasks: HashMap::new(),
            results: HashMap::new(), search: String::new(), category: "Todos".into(),
            sort_az: false, refreshing: false, refresh_left: 0, notice: None,
            confirm_remove: None,
        };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        if self.refreshing { return; }
        self.refreshing = true;
        self.refresh_left = APPS.len();
        self.errors.clear();
        for app in APPS {
            let app = *app;
            let tx = self.tx.clone();
            thread::spawn(move || {
                let result = installer::fetch_latest(&app);
                let _ = tx.send(Event::Release { id: app.id.to_string(), result });
                let detect_tx = tx.clone();
                thread::spawn(move || {
                    let result = if app.is_decode_project {
                        installer::detect_external(&app)
                    } else {
                        None
                    };
                    let _ = detect_tx.send(Event::Detected { id: app.id.to_string(), result });
                });

            });
        }
    }

    fn poll(&mut self, ctx: &egui::Context) {
        loop {
            match self.rx.try_recv() {
                Ok(Event::Release { id, result }) => {
                    self.refresh_left = self.refresh_left.saturating_sub(1);
                    match result {
                        Ok(release) => {
                            self.errors.remove(&id);
                            self.releases.insert(id, release);
                            installer::save_release_cache(&self.paths.release_cache(), &self.releases);
                        }
                        Err(error) => { self.errors.insert(id, error); }
                    }
                    if self.refresh_left == 0 {
                        self.refreshing = false;
                        let message = if self.errors.is_empty() {
                            ("Versões e catálogo atualizados.".to_string(), false)
                        } else if self.releases.is_empty() {
                            ("Não foi possível consultar o GitHub. Os links oficiais continuam disponíveis.".to_string(), true)
                        } else {
                            ("Algumas versões não atualizaram; os dados em cache foram preservados.".to_string(), true)
                        };
                        self.notice = Some(message);
                    }
                }
                Ok(Event::Detected { id, result }) => {
                    if let Some(found) = result { self.detected.insert(id, found); }
                    else { self.detected.remove(&id); }
                }
                Ok(Event::Progress { id, text }) => { self.tasks.insert(id, text); }
                Ok(Event::Finished { id, result }) => {
                    self.tasks.remove(&id);
                    match result {
                        Ok(done) => {
                            if let Some(record) = done.record {
                                self.manifest.apps.insert(id.clone(), record);
                                if let Err(error) = self.manifest.save(&self.paths.manifest()) {
                                    self.notice = Some((format!("Instalado, mas o registro local falhou: {error}"), true));
                                }
                            }
                            self.results.insert(id.clone(), done.message.clone());
                            self.notice = Some((done.message, false));
                            if let Some(app) = APPS.iter().find(|a| a.id == id).copied() {
                                let tx = self.tx.clone();
                                thread::spawn(move || {
                                    let result = if app.is_decode_project {
                                        installer::detect_external(&app)
                                    } else {
                                        None
                                    };
                                    let _ = tx.send(Event::Detected { id: app.id.to_string(), result });
                                });
                            }
                        }
                        Err(error) => {
                            self.results.insert(id.clone(), error.clone());
                            self.notice = Some((format!("{}: {error}", display_name(&id)), true));
                        }
                    }
                }
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }
    }

    fn managed(&self, app: &DecodeApp) -> Option<&InstalledRecord> {
        self.manifest.apps.get(app.id).filter(|r| r.path.exists())
    }

    fn external(&self, app: &DecodeApp) -> Option<&ExternalInstall> {
        if !app.is_decode_project || self.managed(app).is_some() {
            None
        } else {
            self.detected.get(app.id)
        }
    }

    fn install(&mut self, app: &'static DecodeApp) {
        if !app.is_decode_project {
            self.notice = Some((
                format!("{} ainda não possui um pacote recompilado com a identidade Decode. Para evitar instalar o executável original, esta ação abre as instruções de preparação.", app.name),
                true,
            ));
            self.open_url(app.preparation_url());
            return;
        }
        if self.tasks.contains_key(app.id) { return; }
        let Some(release) = self.releases.get(app.id).cloned() else {
            self.open_url(app.releases_url());
            return;
        };
        let paths = self.paths.clone();
        let previous = self.manifest.apps.get(app.id).cloned();
        let tx = self.tx.clone();
        let id = app.id.to_string();
        self.tasks.insert(id.clone(), "Preparando a instalação…".into());
        thread::spawn(move || {
            let progress_tx = tx.clone();
            let progress_id = id.clone();
            let mut progress = move |text| {
                let _ = progress_tx.send(Event::Progress { id: progress_id.clone(), text });
            };
            let result = installer::install_or_update(app, &release, &paths, previous.as_ref(), &mut progress);
            let _ = tx.send(Event::Finished { id, result });
        });
    }

    fn launch(&mut self, app: &DecodeApp) {
        let target = self.managed(app).map(|r| r.path.to_string_lossy().to_string())
            .or_else(|| self.external(app).map(|e| e.target.clone()));
        if let Some(target) = target {
            if let Err(error) = installer::launch(&target) { self.notice = Some((error, true)); }
        } else {
            self.notice = Some((format!("{} não está instalado neste computador.", app.name), true));
        }
    }

    fn remove(&mut self, id: &str) {
        let Some(app) = APPS.iter().find(|a| a.id == id).copied() else { self.confirm_remove = None; return };
        let Some(record) = self.manifest.apps.get(id).cloned() else { self.confirm_remove = None; return };
        match installer::remove_managed(&app, &record, &self.paths) {
            Ok(()) => {
                self.manifest.apps.remove(id);
                let saved = self.manifest.save(&self.paths.manifest());
                self.notice = Some(match saved {
                    Ok(()) => (format!("{} foi removido da instalação gerenciada.", app.name), false),
                    Err(e) => (format!("O aplicativo foi removido, mas o registro local falhou: {e}"), true),
                });
            }
            Err(error) => self.notice = Some((error, true)),
        }
        self.confirm_remove = None;
    }

    fn filtered(&self) -> Vec<&'static DecodeApp> {
        let query = self.search.trim().to_lowercase();
        let mut list: Vec<&'static DecodeApp> = APPS.iter().filter(|app| {
            let category_ok = self.category == "Todos" || app.category == self.category;
            let haystack = format!("{} {} {} {} {}", app.name, app.tagline, app.category, app.repo, app.license).to_lowercase();
            category_ok && (query.is_empty() || haystack.contains(&query))
        }).collect();
        if self.sort_az { list.sort_by(|a, b| a.name.cmp(b.name)); }
        list
    }

    fn status(&self, app: &DecodeApp) -> (String, egui::Color32) {
        if let Some(record) = self.managed(app) {
            if app.is_decode_project {
                let text = self.releases.get(app.id).map(|release| {
                    if is_newer(&release.tag_name, &record.version) {
                        format!("Atualização disponível · {}", release.tag_name)
                    } else {
                        format!("Decode instalado · {}", record.version)
                    }
                }).unwrap_or_else(|| format!("Decode instalado · {}", record.version));
                let color = if text.starts_with("Atualização") { rgb([255, 196, 93]) } else { rgb([123, 225, 180]) };
                return (text, color);
            }
            return (
                format!("Decode instalado · {}", record.version),
                rgb([123, 225, 180]),
            );
        }
        if self.external(app).is_some() {
            if app.is_decode_project {
                return ("Instalado fora da central".into(), rgb([123, 225, 180]));
            }
            return ("Instalação upstream encontrada".into(), rgb([255, 196, 93]));
        }
        if !app.is_decode_project {
            if let Some(release) = self.releases.get(app.id) {
                return (
                    format!("Build Decode em preparação · base {}", release.tag_name),
                    rgb([255, 196, 93]),
                );
            }
            if self.refreshing {
                return ("Build Decode em preparação · consultando fonte".into(), rgb([255, 196, 93]));
            }
            if self.errors.contains_key(app.id) {
                return ("Build Decode em preparação".into(), rgb([255, 196, 93]));
            }
            return ("Build Decode em preparação".into(), rgb([255, 196, 93]));
        }
        if let Some(release) = self.releases.get(app.id) {
            if model::select_asset(release, app, HostPlatform::current(), None).is_some() {
                return (format!("Disponível · {}", release.tag_name), rgb([137, 183, 255]));
            }
            return ("Sem pacote compatível".into(), rgb([255, 196, 93]));
        }
        if self.refreshing { return ("Consultando versão…".into(), rgb([157, 174, 199])); }
        if self.errors.contains_key(app.id) { return ("Versão indisponível".into(), rgb([242, 143, 143])); }
        ("Aguardando dados".into(), rgb([157, 174, 199]))
    }

    fn card(&mut self, ui: &mut egui::Ui, app: &'static DecodeApp, width: f32) {
        let (status, status_color) = self.status(app);
        let managed = self.managed(app).is_some();
        let installed = app.is_decode_project && (managed || self.external(app).is_some());
        let busy = self.tasks.get(app.id).cloned();
        let fill = if app.is_decode_project { rgb([22, 38, 64]) } else { rgb([17, 27, 45]) };
        let stroke = if app.is_decode_project { rgb([83, 135, 218]) } else { rgb([42, 56, 78]) };
        egui::Frame::new().fill(fill).stroke(egui::Stroke::new(1.0, stroke))
            .corner_radius(egui::CornerRadius::same(15)).inner_margin(egui::Margin::same(14))
            .show(ui, |ui| {
                ui.set_min_width(width - 28.0);
                ui.set_max_width(width - 28.0);
                ui.set_min_height(252.0);
                ui.horizontal(|ui| {
                    if let Some(icon) = self.icons.get(app.id) {
                        ui.add(egui::Image::new((icon.id(), egui::vec2(48.0, 48.0))).fit_to_exact_size(egui::vec2(48.0, 48.0)));
                    } else {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(48.0, 48.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, egui::CornerRadius::same(12), rgb([44, 104, 211]));
                        ui.painter().rect_stroke(rect, egui::CornerRadius::same(12), egui::Stroke::new(1.0, rgb([108, 169, 255])), egui::StrokeKind::Inside);
                        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "D", egui::FontId::proportional(28.0), egui::Color32::WHITE);
                    }
                    ui.vertical(|ui| {
                        let group = if app.is_decode_project { "PROJETO DECODE" } else { "DECODE SUITE" };
                        ui.label(egui::RichText::new(group).size(9.0).strong().color(rgb([145, 190, 255])));
                        ui.label(egui::RichText::new(app.license).size(10.0).color(rgb([155, 174, 199])));
                    });
                });
                ui.add_space(9.0);
                ui.label(egui::RichText::new(app.name).size(19.0).strong().color(rgb([239, 245, 255])));
                ui.label(egui::RichText::new(app.tagline).size(12.0).color(rgb([166, 181, 202])));
                ui.add_space(5.0);
                ui.label(egui::RichText::new(app.category).size(10.0).color(rgb([128, 150, 181])));
                ui.add_space(5.0);
                ui.label(egui::RichText::new(&status).size(11.0).color(status_color));
                if let Some(progress) = busy.as_ref() {
                    ui.label(egui::RichText::new(progress).size(10.0).color(rgb([114, 210, 224])));
                } else if let Some(msg) = self.results.get(app.id) {
                    ui.label(egui::RichText::new(shorten(msg, 78)).size(10.0).color(rgb([139, 158, 184])));
                }
                ui.add_space(5.0);
                ui.separator();
                ui.horizontal(|ui| {
                    if busy.is_some() {
                        ui.add_enabled(false, egui::Button::new("Processando…").min_size(egui::vec2(width - 105.0, 30.0)));
                    } else if installed {
                        let open_label = if managed { "Abrir aplicativo" } else { "Abrir instalação original" };
                        if ui.add_sized([width - 105.0, 30.0], egui::Button::new(open_label)).clicked() { self.launch(app); }
                    } else {
                        let label = if app.is_decode_project {
                            if self.releases.contains_key(app.id) { "Instalar Decode CAD" } else { "Releases do Decode CAD" }
                        } else {
                            "Preparar build Decode"
                        };
                        if ui.add_sized([width - 105.0, 30.0], egui::Button::new(label).fill(rgb([213, 230, 255])).stroke(egui::Stroke::NONE)).clicked() {
                            if !app.is_decode_project {
                                self.open_url(app.preparation_url());
                            } else if self.releases.contains_key(app.id) {
                                self.install(app);
                            } else {
                                self.open_url(app.releases_url());
                            }
                        }
                    }
                    ui.menu_button("⋯", |ui| {
                        if ui.button("Código-fonte no GitHub ↗").clicked() { self.open_url(app.repo_url()); ui.close(); }
                        if ui.button("Releases oficiais ↗").clicked() { self.open_url(app.releases_url()); ui.close(); }
                        if ui.button("Site do projeto ↗").clicked() { self.open_url(app.website_url()); ui.close(); }
                        if managed && ui.button("Remover cópia gerenciada…").clicked() { self.confirm_remove = Some(app.id.to_string()); ui.close(); }
                        if let Some(error) = self.errors.get(app.id) {
                            ui.separator();
                            ui.label(egui::RichText::new(error).size(10.0).color(rgb([242, 143, 143])));
                        }
                    });
                });
            });
    }

    fn open_url(&mut self, url: String) {
        if !url.starts_with("https://") || url.chars().any(|c| c.is_control() || c.is_whitespace() || matches!(c, '|' | ';' | '&' | '$' | '<' | '>') || c as u32 == 96) {
            self.notice = Some(("O endereço externo foi rejeitado por segurança.".into(), true));
            return;
        }
        #[cfg(target_os = "windows")]
        let result = Command::new("rundll32").args(["url.dll,FileProtocolHandler", &url]).spawn().map(|_| ());
        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg(&url).spawn().map(|_| ());
        #[cfg(target_os = "linux")]
        let result = Command::new("xdg-open").arg(&url).spawn().map(|_| ());
        if let Err(error) = result {
            self.notice = Some((format!("Não foi possível abrir o navegador: {error}"), true));
        }
    }

    fn remove_confirmation(&mut self, ctx: &egui::Context) {
        let Some(id) = self.confirm_remove.clone() else { return };
        egui::Window::new("Remover aplicativo").collapsible(false).resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_min_width(330.0);
                ui.label(format!("Deseja remover a cópia gerenciada de {}?", display_name(&id)));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("Instalações externas não serão alteradas.").size(12.0).color(rgb([166, 181, 202])));
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancelar").clicked() { self.confirm_remove = None; }
                    if ui.button(egui::RichText::new("Confirmar remoção").color(rgb([255, 150, 150]))).clicked() { self.remove(&id); }
                });
            });
    }
}

impl eframe::App for Launcher {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll(ctx);
        ctx.request_repaint_after(Duration::from_millis(150));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Frame::new().fill(rgb([8, 13, 25])).inner_margin(egui::Margin::same(18)).show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(40.0, 40.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, egui::CornerRadius::same(10), rgb([83, 137, 233]));
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "D", egui::FontId::proportional(24.0), rgb([7, 17, 33]));
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("DECODE").size(16.0).strong().color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new("OPEN SOURCE LAUNCHER").size(9.0).strong().color(rgb([139, 167, 211])));
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Seu universo open source").size(20.0).strong().color(rgb([238, 245, 255])));
                    ui.label(egui::RichText::new("Explore, instale, atualize e abra as ferramentas em um só lugar.").size(12.0).color(rgb([155, 174, 199])));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_enabled(!self.refreshing, egui::Button::new(if self.refreshing { "Consultando…" } else { "↻ Atualizar versões" })).clicked() { self.refresh(); }
                });
            });
            ui.add_space(14.0);
            ui.horizontal_wrapped(|ui| {
                chip(ui, format!("{} ferramentas", APPS.len()), rgb([126, 179, 255]));
                chip(ui, "13 apps Decode".into(), rgb([169, 185, 208]));
                chip(ui, "1 projeto Decode".into(), rgb([111, 222, 231]));
                chip(ui, HostPlatform::current().label().to_string(), rgb([123, 225, 180]));
            });
            if let Some((notice, is_error)) = self.notice.clone() {
                ui.add_space(9.0);
                let color = if is_error { rgb([255, 186, 146]) } else { rgb([135, 223, 180]) };
                egui::Frame::new().fill(rgb([17, 29, 46])).stroke(egui::Stroke::new(1.0, color.linear_multiply(0.4)))
                    .corner_radius(egui::CornerRadius::same(8)).inner_margin(egui::Margin::same(9)).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(notice).size(12.0).color(color));
                            if ui.small_button("Fechar").clicked() { self.notice = None; }
                        });
                    });
            }
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_sized([ui.available_width() * 0.54, 36.0], egui::TextEdit::singleline(&mut self.search).hint_text("Buscar nome, função ou categoria…"));
                egui::ComboBox::from_id_salt("category-select").selected_text(&self.category).width(190.0).show_ui(ui, |ui| {
                    for category in catalog::categories() {
                        if ui.selectable_label(self.category == category, category).clicked() { self.category = category.to_string(); }
                    }
                });
                if ui.selectable_label(self.sort_az, "A–Z").on_hover_text("Ordenar por nome").clicked() { self.sort_az = !self.sort_az; }
            });
            let list = self.filtered();
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{} resultados", list.len())).size(11.0).color(rgb([139, 158, 184])));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Pacotes e versões vêm das releases oficiais").size(10.0).color(rgb([103, 125, 157])));
                });
            });
            ui.add_space(7.0);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                let columns = if ui.available_width() > 1180.0 { 4 } else if ui.available_width() > 850.0 { 3 } else if ui.available_width() > 540.0 { 2 } else { 1 };
                let gap = 12.0;
                let card_width = (ui.available_width() - gap * (columns as f32 - 1.0)) / columns as f32;
                egui::Grid::new("catalog-grid").num_columns(columns).spacing(egui::vec2(gap, gap)).show(ui, |ui| {
                    for (index, app) in list.iter().enumerate() {
                        ui.allocate_ui_with_layout(egui::vec2(card_width, 1.0), egui::Layout::top_down(egui::Align::Min), |ui| self.card(ui, app, card_width));
                        if (index + 1) % columns == 0 { ui.end_row(); }
                    }
                });
                if list.is_empty() {
                    ui.add_space(40.0);
                    ui.centered_and_justified(|ui| { ui.label("Nenhuma ferramenta encontrada. Tente outro termo ou selecione Todos."); });
                }
            });
            ui.add_space(8.0);
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Central independente da Decode Analytics. Marcas e aplicativos pertencem aos seus projetos originais.").size(10.0).color(rgb([111, 130, 158])));
                ui.hyperlink_to("Código-fonte", "https://github.com/Kaiqueaurelio/decodecad/tree/main/launcher-desktop");
                ui.hyperlink_to("Launcher open source de referência", "https://github.com/Kaiqueaurelio/decode-open-source-hub");
            });
        });
        self.remove_confirmation(&ctx);
    }
}

fn load_icon_texture(ctx: &egui::Context, app: &DecodeApp) -> Option<egui::TextureHandle> {
    let bytes = app.icon_svg().as_bytes();
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(bytes, &options).ok()?;
    let size = 128_u32;
    let width = tree.size().width();
    let height = tree.size().height();
    if width <= 0.0 || height <= 0.0 { return None; }
    let scale = (size as f32 / width.max(height)).min(1.0);
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size)?;
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    let rgba = image::RgbaImage::from_raw(size, size, pixmap.data().to_vec())?;
    let color = egui::ColorImage::from_rgba_unmultiplied([size as usize, size as usize], rgba.as_raw());
    Some(ctx.load_texture(format!("decode-icon-{}", app.product_id), color, egui::TextureOptions::LINEAR))
}

fn chip(ui: &mut egui::Ui, text: String, color: egui::Color32) {
    egui::Frame::new().fill(rgb([16, 27, 44])).stroke(egui::Stroke::new(1.0, rgb([41, 55, 77])))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin { left: 9, right: 9, top: 5, bottom: 5 })
        .show(ui, |ui| { ui.label(egui::RichText::new(text).size(11.0).strong().color(color)); });
}

fn rgb(c: [u8; 3]) -> egui::Color32 { egui::Color32::from_rgb(c[0], c[1], c[2]) }

fn display_name(id: &str) -> String {
    APPS.iter().find(|a| a.id == id).map(|a| a.name.to_string()).unwrap_or_else(|| id.to_string())
}

fn is_newer(latest: &str, installed: &str) -> bool {
    match (
        semver::Version::parse(latest.trim_start_matches('v')),
        semver::Version::parse(installed.trim_start_matches('v')),
    ) {
        (Ok(a), Ok(b)) => a > b,
        _ => false,
    }
}

fn shorten(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars { return text.to_string(); }
    format!("{}…", text.chars().take(max_chars.saturating_sub(1)).collect::<String>())
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Decode Open Source Launcher")
            .with_inner_size([1280.0, 860.0])
            .with_min_inner_size([750.0, 540.0]),
        ..Default::default()
    };
    eframe::run_native("Decode Open Source Launcher", options, Box::new(|cc| Ok(Box::new(Launcher::new(cc)))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_comparison_recognizes_updates() {
        assert!(is_newer("v0.6.0", "0.5.0"));
        assert!(!is_newer("0.5.0", "0.5.0"));
        assert!(!is_newer("0.4.0", "0.5.0"));
    }

    #[test]
    fn shorten_respects_the_requested_length() {
        assert_eq!(shorten("curto", 10), "curto");
        assert_eq!(shorten("frase grande demais", 8).chars().count(), 8);
    }
}
