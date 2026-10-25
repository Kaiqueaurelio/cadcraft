//! Decode CAD desktop app.
//!
//! Usage: `decode-cad [--control <port>] [--sample] [files…]`
//!
//! `--control <port>` (or `DECODE_CAD_CONTROL_PORT`) starts a localhost JSON-lines control server:
//! `{"id":1,"method":"cmdline.input","params":{"text":"circle 0,0 5"}}` → `{"id":1,"ok":true,…}`.
//! See `cadcraft_ui_egui::control` for the methods.
//!
//! `DECODE_CAD_VSYNC=1` presents with vsync on Linux/BSD, where the default is the low-latency
//! present mode (`DECODE_CAD_VSYNC=0` selects that elsewhere); see `cadcraft_ui_egui::gpu::surface_config`.
//!
//! `log` records go to standard error and `<settings dir>/logs/cadcraft.log` (`RUST_LOG` sets the
//! levels); see [`logging`] and [`config_dir`].
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod control_server;
mod desktop_theme;
#[cfg(any(target_os = "windows", test))]
mod graphics;
mod logging;
#[cfg(target_os = "macos")]
mod native_menu;
mod spacemouse;
mod system_animations;
mod workspace;

use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services, i18n};

struct App {
    cad: CadApp,
    spacemouse: spacemouse::SpaceMouse,
    /// The desktop's light/dark choice where winit reports none (Linux, the BSDs).
    desktop_theme: desktop_theme::DesktopTheme,
    /// Whether the OS shows animations, for the System choice of View ▸ Accessibility ▸ Animation Effects.
    system_animations: system_animations::SystemAnimations,
    workspace: workspace::Workspace,
    workspace_error: Option<String>,
    /// Brief branded first frame while the already-initialized workspace settles.
    splash_started: std::time::Instant,
    #[cfg(target_os = "macos")]
    native_menu: Option<native_menu::NativeMenu>,
}

impl App {
    fn persist_workspace(&mut self, final_attempt: bool) {
        let result = if final_attempt { self.workspace.save_on_exit(&self.cad.ui) } else { self.workspace.save_if_changed(&self.cad.ui) };
        if let Err(error) = result {
            if self.workspace_error.as_ref() != Some(&error) {
                eprintln!("Decode CAD workspace: {error}");
                self.cad.set_status(format!("Workspace could not be saved: {error}"));
                self.workspace_error = Some(error);
            }
        } else {
            self.workspace_error = None;
        }
    }
}

/// View ▸ Theme ▸ System resolves against `raw.system_theme`, which winit leaves empty on Linux
/// and the BSDs: the desktop's answer, where it gives one, stands in for it.
fn follow_desktop(raw: &mut egui::RawInput, desktop: Option<egui::Theme>) {
    if desktop.is_some() {
        raw.system_theme = desktop;
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "macos")]
        {
            if self.native_menu.is_none() && std::env::var_os("DECODE_CAD_NO_NATIVE_MENU").is_none() && std::env::var_os("CADCRAFT_NO_NATIVE_MENU").is_none() {
                self.native_menu = Some(native_menu::NativeMenu::install(&mut self.cad));
            }
            if let Some(m) = &mut self.native_menu {
                m.poll(&mut self.cad, ctx);
            }
        }
        self.cad.system_animations = self.system_animations.current();
        self.cad.logic(ctx);
        self.spacemouse.poll(&mut self.cad, ctx);
        self.persist_workspace(false);
        if std::mem::take(&mut self.cad.quit_requested) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        follow_desktop(raw, self.desktop_theme.current());
        self.cad.raw_input_hook(raw);
        cadcraft_ui_egui::cmdline::capture_tab(ctx, raw);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let elapsed = self.splash_started.elapsed();
        if elapsed < std::time::Duration::from_millis(1600) {
            draw_startup_splash(ui, elapsed.as_secs_f32());
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
            return;
        }
        self.cad.ui(ui);
        self.persist_workspace(false);
    }
    /// A normal exit: the workspace is saved, and this run's autosave files are not needed (a
    /// crash leaves them for recovery).
    fn on_exit(&mut self) {
        self.persist_workspace(true);
        self.cad.shutdown_autosave();
    }
    /// Preferences (the interface theme) survive restarts; window and egui state are not kept.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string(cadcraft_ui_egui::options::PROFILE_KEY, self.cad.profile_json());
        storage.set_string(cadcraft_ui_egui::PREFS_KEY, self.cad.prefs_json());
        storage.set_string(cadcraft_ui_egui::autosave::PREFS_KEY, cadcraft_ui_egui::autosave::prefs_json());
    }
    fn persist_egui_memory(&self) -> bool {
        false
    }
}

/// File format hooks for the engine (it stays I/O-agnostic).
pub fn install_io() {
    cadcraft_engine::cmd::file::set_io(cadcraft_engine::cmd::file::IoHooks {
        read: |b, name| cadcraft_io::read(b, name).map_err(|e| e.to_string()),
        write: |d, name| cadcraft_io::write(d, name).map_err(|e| e.to_string()),
        plot: Some(|d, space, opts| cadcraft_io::plotter::plot(d, space, opts).map_err(|e| e.to_string())),
    });
    cadcraft_engine::cmd::file::set_write_as(|d, name, o| {
        cadcraft_io::write_with(d, name, &cadcraft_io::SaveOptions { version: o.version, binary: o.binary }).map_err(|e| e.to_string())
    });
    // AutoLISP's entget, entmake and entmod see entities as DXF groups.
    cadcraft_engine::cmd::file::set_entity_codec(cadcraft_engine::cmd::file::EntityCodec {
        groups: cadcraft_io::entity_groups,
        entity: cadcraft_io::entity_from_groups,
    });
}

fn services() -> Services {
    Services {
        pick_open: Some(Box::new(|| {
            rfd::FileDialog::new()
                .add_filter(i18n::t("Drawings (DWG, DXF)"), &["dwg", "dxf", "dwt"])
                .add_filter(i18n::t("SVG drawings (SVG, SVGZ)"), &["svg", "svgz"])
                .add_filter(i18n::t("All files"), &["*"])
                .pick_file()
                .map(|p| p.to_string_lossy().to_string())
        })),
        pick_save: Some(Box::new(|name: &str| {
            filters_for_save(name)
                .iter()
                .fold(rfd::FileDialog::new().set_file_name(name), |d, (label, ext)| d.add_filter(i18n::t(label), &[*ext]))
                .save_file()
                .map(|p| p.to_string_lossy().to_string())
        })),
        // APPLOAD: AutoLISP routines only, starting where the last one was loaded from.
        pick_lisp: Some(Box::new(|dir: &str| {
            let dialog = rfd::FileDialog::new().add_filter(i18n::t("AutoLISP (LSP)"), &["lsp"]).add_filter(i18n::t("All files"), &["*"]);
            let dialog = if dir.is_empty() { dialog } else { dialog.set_directory(dir) };
            dialog.pick_file().map(|p| p.to_string_lossy().to_string())
        })),
        no_file_system: false,
    }
}

/// File types for the Save dialog, the suggested name's type first: Windows adds the first
/// filter's extension, and macOS accepts only listed extensions, replacing any other with the
/// first one. DWG must be listed for a `.dwg` name to survive, and a drawing opened from DWG
/// then defaults to DWG; other names keep DXF first.
fn filters_for_save(name: &str) -> [(&'static str, &'static str); 6] {
    let mut filters =
        [("Drawing (DXF)", "dxf"), ("Drawing (DWG)", "dwg"), ("Drawing Template (DWT)", "dwt"), ("SVG", "svg"), ("PNG", "png"), ("PDF", "pdf")];
    let lower = name.to_ascii_lowercase();
    filters.sort_by_key(|(_, ext)| !lower.ends_with(&format!(".{ext}")));
    filters
}

/// The historical platform-specific folder is intentionally retained so existing preferences,
/// workspace state and plotter settings load instead of silently starting with fresh settings.
fn config_dir() -> Option<std::path::PathBuf> {
    config_dir_from(|k| std::env::var_os(k))
}

/// [`config_dir`] with the environment passed in (`var` looks a variable up). An empty variable
/// counts as unset, so a blank value never means the working directory.
fn config_dir_from(var: impl Fn(&str) -> Option<std::ffi::OsString>) -> Option<std::path::PathBuf> {
    let set = |k: &str| var(k).filter(|v| !v.is_empty()).map(std::path::PathBuf::from);
    if let Some(dir) = set("DECODE_CAD_CONFIG_DIR").or_else(|| set("CADCRAFT_CONFIG_DIR")) {
        return Some(dir);
    }
    if cfg!(target_os = "macos") {
        set("HOME").map(|h| h.join("Library/Application Support/CADCraft"))
    } else if cfg!(windows) {
        set("APPDATA").map(|a| a.join("CADCraft"))
    } else {
        // The XDG spec ignores a relative XDG_CONFIG_HOME.
        set("XDG_CONFIG_HOME").filter(|p| p.is_absolute()).or_else(|| set("HOME").map(|h| h.join(".config"))).map(|c| c.join("cadcraft"))
    }
}

const APP_ID: &str = "ai.storyteller.cadcraft";

/// Stencil bits of the window's render pass (see `NativeOptions::stencil_buffer`).
const STENCIL_BITS: u8 = 8;

/// Create the app icon from Decode CAD's own vector geometry; do not load inherited artwork.
fn app_icon() -> Option<egui::IconData> {
    let size = 64usize;
    let mut rgba = vec![0u8; size * size * 4];
    let navy = [14, 24, 42, 255];
    let border = [43, 65, 94, 255];
    let blue = [56, 137, 255, 255];
    let cyan = [43, 220, 220, 255];
    let white = [244, 248, 255, 255];
    let orange = [255, 166, 74, 255];

    for y in 0..size {
        for x in 0..size {
            let dx = if x < 10 { 10 - x } else if x >= size - 10 { x - (size - 11) } else { 0 };
            let dy = if y < 10 { 10 - y } else if y >= size - 10 { y - (size - 11) } else { 0 };
            if dx == 0 || dy == 0 || (dx * dx + dy * dy) <= 100 {
                icon_pixel(&mut rgba, size, x as i32, y as i32, navy);
            }
        }
    }
    for x in 10..54 {
        icon_pixel(&mut rgba, size, x, 9, border);
        icon_pixel(&mut rgba, size, x, 54, border);
    }
    for y in 10..54 {
        icon_pixel(&mut rgba, size, 9, y, border);
        icon_pixel(&mut rgba, size, 54, y, border);
    }
    let point = |x: f32, y: f32| ((32.0 + x * 58.0).round() as i32, (32.0 + y * 58.0).round() as i32);
    let cube = [(-0.30,-0.14), (0.0,-0.31), (0.30,-0.14), (0.30,0.17), (0.0,0.34), (-0.30,0.17), (-0.30,-0.14)];
    for seg in cube.windows(2) {
        let a = point(seg[0].0, seg[0].1);
        let b = point(seg[1].0, seg[1].1);
        icon_line(&mut rgba, size, a, b, blue, 2);
    }
    for (a, b) in [
        (point(0.0,-0.31), point(0.0,0.0)),
        (point(-0.30,0.17), point(0.0,0.0)),
        (point(0.30,0.17), point(0.0,0.0)),
    ] {
        icon_line(&mut rgba, size, a, b, cyan, 2);
    }
    let dpoints = [
        point(-0.105,-0.155), point(-0.105,0.155), point(-0.01,0.155),
        point(0.075,0.105), point(0.105,0.0), point(0.075,-0.105),
        point(-0.01,-0.155), point(-0.105,-0.155),
    ];
    for seg in dpoints.windows(2) {
        icon_line(&mut rgba, size, seg[0], seg[1], white, 2);
    }
    icon_line(&mut rgba, size, point(0.15,-0.22), point(0.23,-0.265), orange, 2);
    let (ox, oy) = point(0.23,-0.265);
    icon_disk(&mut rgba, size, ox, oy, 2, orange);
    Some(egui::IconData { rgba, width: size as u32, height: size as u32 })
}

fn icon_pixel(rgba: &mut [u8], width: usize, x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= width as i32 || y >= width as i32 {
        return;
    }
    let i = (y as usize * width + x as usize) * 4;
    rgba[i..i + 4].copy_from_slice(&color);
}

fn icon_disk(rgba: &mut [u8], width: usize, cx: i32, cy: i32, radius: i32, color: [u8; 4]) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                icon_pixel(rgba, width, cx + x, cy + y, color);
            }
        }
    }
}

fn icon_line(rgba: &mut [u8], width: usize, a: (i32, i32), b: (i32, i32), color: [u8; 4], thickness: i32) {
    let steps = (b.0 - a.0).abs().max((b.1 - a.1).abs()).max(1);
    for i in 0..=steps {
        let x = a.0 + (b.0 - a.0) * i / steps;
        let y = a.1 + (b.1 - a.1) * i / steps;
        icon_disk(rgba, width, x, y, thickness, color);
    }
}

/// The first visible desktop frame uses the same Decode CAD mark as the web loading screen.
fn draw_startup_splash(ui: &mut egui::Ui, elapsed: f32) {
    use egui::{Align2, Color32, FontId, Rect, Stroke, StrokeKind, pos2, vec2};
    let rect = ui.max_rect();
    let painter = ui.painter().clone();
    painter.rect_filled(rect, 0.0, Color32::from_rgb(12, 20, 35));

    let mark_size = 116.0;
    let mark = Rect::from_center_size(pos2(rect.center().x, rect.center().y - 100.0), vec2(mark_size, mark_size));
    painter.rect_filled(mark, 22.0, Color32::from_rgb(14, 24, 42));
    painter.rect_stroke(mark, 22.0, Stroke::new(1.5, Color32::from_rgb(43, 65, 94)), StrokeKind::Inside);
    let point = |x: f32, y: f32| pos2(mark.center().x + mark_size * x, mark.center().y + mark_size * y);
    let blue = Stroke::new(3.2, Color32::from_rgb(56, 137, 255));
    let cyan = Stroke::new(2.6, Color32::from_rgb(43, 220, 220));
    let cube = [(-0.30,-0.14), (0.0,-0.31), (0.30,-0.14), (0.30,0.17), (0.0,0.34), (-0.30,0.17), (-0.30,-0.14)];
    for seg in cube.windows(2) {
        painter.line_segment([point(seg[0].0, seg[0].1), point(seg[1].0, seg[1].1)], blue);
    }
    for (a, b) in [
        (point(0.0,-0.31), point(0.0,0.0)),
        (point(-0.30,0.17), point(0.0,0.0)),
        (point(0.30,0.17), point(0.0,0.0)),
    ] {
        painter.line_segment([a, b], cyan);
    }
    let white = Stroke::new(3.0, Color32::from_rgb(244, 248, 255));
    let dpoints = [
        point(-0.105,-0.155), point(-0.105,0.155), point(-0.01,0.155),
        point(0.075,0.105), point(0.105,0.0), point(0.075,-0.105),
        point(-0.01,-0.155), point(-0.105,-0.155),
    ];
    for seg in dpoints.windows(2) {
        painter.line_segment([seg[0], seg[1]], white);
    }
    painter.line_segment([point(0.15,-0.22), point(0.23,-0.265)], Stroke::new(3.0, Color32::from_rgb(255, 166, 74)));
    painter.circle_filled(point(0.23,-0.265), 3.3, Color32::from_rgb(255, 166, 74));

    painter.text(pos2(rect.center().x, rect.center().y + 6.0), Align2::CENTER_CENTER, "Decode CAD", FontId::proportional(32.0), Color32::from_rgb(244, 248, 255));
    painter.text(pos2(rect.center().x, rect.center().y + 39.0), Align2::CENTER_CENTER, "DECODE ANALYTICS", FontId::proportional(12.0), Color32::from_rgb(123, 155, 197));
    let bar = Rect::from_center_size(pos2(rect.center().x, rect.center().y + 93.0), vec2(210.0, 4.0));
    painter.rect_filled(bar, 2.0, Color32::from_rgb(43, 55, 73));
    let progress = (elapsed / 1.6).clamp(0.08, 1.0);
    let fill = Rect::from_min_max(bar.min, pos2(bar.left() + bar.width() * progress, bar.bottom()));
    painter.rect_filled(fill, 2.0, Color32::from_rgb(56, 137, 255));
    painter.text(pos2(rect.center().x, rect.center().y + 119.0), Align2::CENTER_CENTER, "Preparing workspace…", FontId::proportional(12.0), Color32::from_rgb(157, 173, 196));
}

fn version_string() -> String {
    let sha = option_env!("DECODE_CAD_BUILD_SHA").or(option_env!("CADCRAFT_BUILD_SHA")).unwrap_or("dev");
    format!("Decode CAD {} ({sha})", env!("CARGO_PKG_VERSION"))
}

/// The control port from `--control <port>` or `DECODE_CAD_CONTROL_PORT` (`source`); a value that is
/// not a port is logged and ignored instead of silently starting no control channel.
fn control_port_from(source: &str, value: Option<String>) -> Option<u16> {
    let value = value?;
    let port = value.trim().parse().ok();
    if port.is_none() {
        log::warn!("{source}: {value:?} is not a port number; the control channel is off");
    }
    port
}

/// `--control` followed by `value`: a bare `--control` (last argument) keeps the port already
/// read from `DECODE_CAD_CONTROL_PORT` (`current`) instead of turning the control channel off.
fn control_flag(current: Option<u16>, value: Option<String>) -> Option<u16> {
    if value.is_some() {
        return control_port_from("--control", value);
    }
    match current {
        Some(port) => log::warn!("--control: no port given; using DECODE_CAD_CONTROL_PORT ({port})"),
        None => log::warn!("--control: no port given; the control channel is off"),
    }
    current
}

fn main() -> eframe::Result {
    // First, so every start-up record is captured; see `logging`.
    let logger = logging::install();
    // lossy, so a value that is not Unicode is reported like any other bad port instead of ignored
    let mut control_port =
        control_port_from("DECODE_CAD_CONTROL_PORT", std::env::var_os("DECODE_CAD_CONTROL_PORT").or_else(|| std::env::var_os("CADCRAFT_CONTROL_PORT")).map(|p| p.to_string_lossy().into_owned()));
    let mut files = Vec::new();
    let mut sample = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--control" => control_port = control_flag(control_port, args.next()),
            "--sample" => sample = true,
            "--version" | "-V" => {
                println!("{}", version_string());
                return Ok(());
            }
            _ => files.push(a),
        }
    }
    // The log file lives under the settings directory; opened after the arguments, so `--version`
    // leaves no file behind. Records logged until now are written to it first.
    if let Some(logger) = logger {
        match config_dir().map(|d| logger.attach_dir(&d.join(logging::LOG_DIR))) {
            Some(Ok(path)) => log::info!("{} ({} {}), log file {}", version_string(), std::env::consts::OS, std::env::consts::ARCH, path.display()),
            Some(Err(e)) => log::warn!("no log file: {e}"),
            None => {
                logger.stderr_only();
                log::warn!("no log file: no settings directory (set DECODE_CAD_CONFIG_DIR)");
            }
        }
    }
    install_io();
    let surface = cadcraft_ui_egui::gpu::surface_config(std::env::var("DECODE_CAD_VSYNC").or_else(|_| std::env::var("CADCRAFT_VSYNC")).ok().as_deref(), cfg!(all(unix, not(target_os = "macos"))));
    let mut options = eframe::NativeOptions {
        // The crosshair is drawn by the app: present without a queue on Linux/BSD, where X11
        // swapchains hold two frames back under vsync (`DECODE_CAD_VSYNC=1` restores vsync).
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration::default().with_surface_config(surface),
        viewport: egui::ViewportBuilder::default()
            .with_title("Decode CAD")
            .with_inner_size([1600.0, 1000.0])
            .with_min_inner_size([900.0, 560.0])
            .with_drag_and_drop(true)
            .with_fullsize_content_view(true)
            .with_titlebar_shown(false)
            .with_title_shown(false)
            .with_app_id(APP_ID),
        persist_window: false,
        // A stencil buffer clips the contents of clipped layout viewports on the GPU.
        stencil_buffer: STENCIL_BITS,
        ..Default::default()
    };
    if let Some(icon) = app_icon() {
        options.viewport = options.viewport.with_icon(icon);
    }
    // Before eframe creates the wgpu instance: default Windows to DirectX 12 with FXC (see graphics.rs).
    #[cfg(target_os = "windows")]
    graphics::configure(&mut options, eframe::wgpu::Backends::from_env(), eframe::wgpu::Dx12Compiler::from_env());
    // Command previews (COPY, MOVE...) of large selections are tessellated by egui, whose index
    // buffer outgrew wgpu's default 256 MiB cap; the validation error is fatal. Ask for what the
    // adapter actually supports.
    if let eframe::egui_wgpu::WgpuSetup::CreateNew(create) = &mut options.wgpu_options.wgpu_setup {
        let base = std::sync::Arc::clone(&create.device_descriptor);
        create.device_descriptor = std::sync::Arc::new(move |adapter| {
            let mut d = base(adapter);
            d.required_limits.max_buffer_size = d.required_limits.max_buffer_size.max(adapter.limits().max_buffer_size);
            d
        });
    }
    // winit has no file drag-and-drop on Wayland (only on X11), so dropping files from the file
    // manager showed a "no" cursor. Run through XWayland when it's there; DECODE_CAD_WAYLAND=1
    // keeps the native Wayland backend.
    #[cfg(all(unix, not(target_os = "macos")))]
    if std::env::var_os("DISPLAY").is_some() && std::env::var_os("DECODE_CAD_WAYLAND").is_none() && std::env::var_os("CADCRAFT_WAYLAND").is_none() {
        options.event_loop_builder = Some(Box::new(|b| {
            use winit::platform::x11::EventLoopBuilderExtX11;
            b.with_x11();
        }));
    }
    eframe::run_native(
        "Decode CAD",
        options,
        Box::new(move |cc| {
            let mut app = CadApp::new(Session::empty(), services());
            let (workspace, restored, workspace_error) = workspace::Workspace::load();
            if let Some(ui) = restored {
                app.ui = ui;
            }
            if let Some(error) = &workspace_error {
                eprintln!("Decode CAD workspace: {error}");
                app.set_status(error);
            }
            app.system_languages = std::env::var("DECODE_CAD_LOCALE").or_else(|_| std::env::var("CADCRAFT_LOCALE")).map(|s| vec![s]).unwrap_or_else(|_| sys_locale::get_locales().collect());
            // The user profile (Options): drafting settings, selection sizes, drawing-area colours.
            if let Some(profile) = cc.storage.and_then(|s| s.get_string(cadcraft_ui_egui::options::PROFILE_KEY)) {
                app.load_profile(&profile);
            }
            if let Some(prefs) = cc.storage.and_then(|s| s.get_string(cadcraft_ui_egui::PREFS_KEY)) {
                app.load_prefs(&prefs);
            }
            if let Some(prefs) = cc.storage.and_then(|s| s.get_string(cadcraft_ui_egui::autosave::PREFS_KEY)) {
                cadcraft_ui_egui::autosave::load_prefs(&prefs);
            }
            // Autosave files (SAVEFILEPATH) default to `<settings dir>/autosave`.
            if let Some(dir) = config_dir() {
                app.enable_autosave(&dir.join("autosave"));
                // Startup LISP (acad.lsp, acaddoc.lsp) is read from `<settings>/support`; the
                // folder is made so it is there to put them in.
                let support = dir.join("support");
                if let Err(e) = std::fs::create_dir_all(&support) {
                    log::warn!("no LISP support folder {}: {e}", support.display());
                }
                app.session.set_lisp_support_dir(Some(support));
            }
            app.integrated_titlebar = cfg!(target_os = "macos");
            if surface.present_mode == eframe::wgpu::PresentMode::AutoNoVsync {
                app.frame_cap = Some(cadcraft_ui_egui::gpu::FrameCap::new(240));
            }
            if let Some(rs) = &cc.wgpu_render_state {
                app.set_wgpu(rs, eframe::egui_wgpu::depth_format_from_bits(0, STENCIL_BITS));
            }
            if let Some(port) = control_port {
                let rx = control_server::start(port, cc.egui_ctx.clone());
                app = app.with_control(rx);
            }
            if sample {
                let _ = app.run("ui.sample", serde_json::json!({}));
            }
            for f in &files {
                app.open_path(f);
            }
            if app.session.docs.is_empty() {
                // In the units the profile asks for (MEASUREINIT, restored above).
                let metric = app.session.settings.measureinit;
                app.session.new_drawing(metric);
            }
            let ctx = cc.egui_ctx.clone();
            let desktop_theme = desktop_theme::DesktopTheme::start(move || ctx.request_repaint());
            let ctx = cc.egui_ctx.clone();
            let system_animations = system_animations::SystemAnimations::start(move || ctx.request_repaint());
            Ok(Box::new(App {
                desktop_theme,
                system_animations,
                cad: app,
                spacemouse: spacemouse::SpaceMouse::start(cc.egui_ctx.clone()),
                workspace,
                workspace_error,
                splash_started: std::time::Instant::now(),
                #[cfg(target_os = "macos")]
                native_menu: None,
            }))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::{config_dir_from, control_flag, control_port_from, filters_for_save, follow_desktop};
    use std::ffi::OsString;
    use std::path::PathBuf;

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<OsString> + 'a {
        move |k| vars.iter().find(|(n, _)| *n == k).map(|(_, v)| OsString::from(v))
    }

    #[test]
    fn save_dialog_offers_dwg_with_the_drawing_format_first() {
        let first = |name: &str| filters_for_save(name)[0].1;
        assert_eq!(first("Waco Winnelson Floorplan.dwg"), "dwg");
        assert_eq!(first("FLOORPLAN.DWG"), "dwg");
        // New drawings and DXF files keep DXF first.
        assert_eq!(first("Drawing1.dxf"), "dxf");
        assert_eq!(first("Drawing1"), "dxf");
        assert_eq!(first("plan.pdf"), "pdf");
        assert_eq!(first("office.dwt"), "dwt");
        for name in ["a.dwg", "b.dxf", "c"] {
            let exts: Vec<_> = filters_for_save(name).iter().map(|(_, e)| *e).collect();
            assert!(exts.contains(&"dwg") && exts.contains(&"dxf"), "{name}");
        }
    }

    #[test]
    fn control_port_values_that_are_not_ports_are_ignored() {
        assert_eq!(control_port_from("--control", Some("7979".into())), Some(7979));
        assert_eq!(control_port_from("--control", Some(" 7979 ".into())), Some(7979));
        assert_eq!(control_port_from("--control", None), None);
        for bad in ["", "abc", "-1", "65536", "99999999999999999999", "\u{0}"] {
            assert_eq!(control_port_from("--control", Some(bad.into())), None, "{bad:?}");
        }
    }

    #[test]
    fn control_without_a_port_keeps_the_environment_port() {
        // `DECODE_CAD_CONTROL_PORT=47321 cadcraft --control` (#844)
        assert_eq!(control_flag(Some(47321), None), Some(47321));
        assert_eq!(control_flag(None, None), None);
        // A port given on the command line still wins over the variable.
        assert_eq!(control_flag(Some(47321), Some("7979".into())), Some(7979));
        assert_eq!(control_flag(Some(47321), Some("abc".into())), None);
    }

    #[test]
    fn the_override_variable_wins_and_empty_values_are_unset() {
        let vars = [("DECODE_CAD_CONFIG_DIR", "/tmp/cc"), ("HOME", "/home/u"), ("XDG_CONFIG_HOME", "/x"), ("APPDATA", "C:\\a")];
        assert_eq!(config_dir_from(env(&vars)), Some(PathBuf::from("/tmp/cc")));
        let empty = [("DECODE_CAD_CONFIG_DIR", ""), ("HOME", ""), ("XDG_CONFIG_HOME", ""), ("APPDATA", "")];
        assert_eq!(config_dir_from(env(&empty)), None, "no usable variable means no directory, not the working directory");
        assert_eq!(config_dir_from(env(&[])), None);
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    #[test]
    fn linux_and_bsd_follow_xdg_config_home() {
        assert_eq!(config_dir_from(env(&[("XDG_CONFIG_HOME", "/x"), ("HOME", "/home/u")])), Some(PathBuf::from("/x/cadcraft")));
        assert_eq!(config_dir_from(env(&[("HOME", "/home/u")])), Some(PathBuf::from("/home/u/.config/cadcraft")));
        // The XDG spec says a relative XDG_CONFIG_HOME is ignored.
        assert_eq!(config_dir_from(env(&[("XDG_CONFIG_HOME", "rel"), ("HOME", "/home/u")])), Some(PathBuf::from("/home/u/.config/cadcraft")));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_uses_application_support() {
        assert_eq!(config_dir_from(env(&[("HOME", "/Users/u")])), Some(PathBuf::from("/Users/u/Library/Application Support/CADCraft")));
    }

    #[cfg(windows)]
    #[test]
    fn windows_uses_appdata() {
        assert_eq!(
            config_dir_from(env(&[("APPDATA", "C:\\Users\\u\\AppData\\Roaming")])),
            Some(PathBuf::from("C:\\Users\\u\\AppData\\Roaming").join("CADCraft"))
        );
    }

    /// View ▸ Theme ▸ System on Linux: winit reports no theme, so the desktop's answer stands in;
    /// without one (or where winit answers and the desktop doesn't) the input is left alone.
    #[test]
    fn the_desktop_answers_for_the_system_theme() {
        let mut raw = egui::RawInput::default();
        follow_desktop(&mut raw, Some(egui::Theme::Light));
        assert_eq!(raw.system_theme, Some(egui::Theme::Light));
        let mut raw = egui::RawInput { system_theme: Some(egui::Theme::Dark), ..Default::default() };
        follow_desktop(&mut raw, None);
        assert_eq!(raw.system_theme, Some(egui::Theme::Dark));
        let mut raw = egui::RawInput::default();
        follow_desktop(&mut raw, None);
        assert_eq!(raw.system_theme, None);
    }
}
