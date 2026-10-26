use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tar::Archive;

use crate::catalog::DecodeApp;
use crate::model::{self, HostPlatform, OperatingSystem, PackageKind, Release, ReleaseAsset};

const MAX_DOWNLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const MAX_CHECKSUM_BYTES: usize = 2 * 1024 * 1024;
const MAX_ARCHIVE_ENTRIES: usize = 20_000;

#[derive(Clone, Debug)]
pub struct Paths {
    pub data: PathBuf,
    pub cache: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self, String> {
        let data = dirs::data_local_dir()
            .or_else(dirs::home_dir)
            .ok_or_else(|| "Não foi possível localizar a pasta de dados do usuário.".to_string())?
            .join("Decode Open Source Hub");
        let cache = dirs::cache_dir()
            .or_else(dirs::home_dir)
            .ok_or_else(|| "Não foi possível localizar a pasta de cache do usuário.".to_string())?
            .join("Decode Open Source Hub");
        fs::create_dir_all(&data).map_err(|e| format!("Não foi possível criar {}: {e}", data.display()))?;
        fs::create_dir_all(&cache).map_err(|e| format!("Não foi possível criar {}: {e}", cache.display()))?;
        Ok(Self { data, cache })
    }

    pub fn apps(&self) -> PathBuf { self.data.join("apps") }
    pub fn downloads(&self) -> PathBuf { self.cache.join("downloads") }
    pub fn release_cache(&self) -> PathBuf { self.cache.join("releases.json") }
    pub fn manifest(&self) -> PathBuf { self.data.join("installed.json") }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstalledRecord {
    pub version: String,
    pub kind: PackageKind,
    pub path: PathBuf,
    pub installed_at: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub apps: BTreeMap<String, InstalledRecord>,
}

impl Manifest {
    pub fn load(path: &Path) -> Self {
        fs::read(path).ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let parent = path.parent().ok_or_else(|| "O manifesto não tem uma pasta pai.".to_string())?;
        fs::create_dir_all(parent).map_err(|e| format!("Não foi possível criar a pasta do manifesto: {e}"))?;
        let bytes = serde_json::to_vec_pretty(self).map_err(|e| format!("Não foi possível codificar o manifesto: {e}"))?;
        let tmp = path.with_extension(format!("json.tmp.{}", std::process::id()));
        let mut file = File::create(&tmp).map_err(|e| format!("Não foi possível gravar o manifesto temporário: {e}"))?;
        file.write_all(&bytes).map_err(|e| format!("Falha ao gravar o manifesto: {e}"))?;
        file.sync_all().map_err(|e| format!("Falha ao sincronizar o manifesto: {e}"))?;
        fs::rename(&tmp, path).map_err(|e| {
            let _ = fs::remove_file(&tmp);
            format!("Não foi possível substituir o manifesto: {e}")
        })
    }
}

#[derive(Clone, Debug)]
pub struct ExternalInstall {
    /// A normal path, or flatpak:<app-id>.
    pub target: String,
    pub version: Option<String>,
}

#[derive(Clone, Debug)]
pub struct InstallResult {
    pub record: Option<InstalledRecord>,
    pub message: String,
}

pub fn fetch_latest(app: &DecodeApp) -> Result<Release, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", app.repo);
    let client = Client::builder()
        .user_agent(concat!("DecodeOpenSourceLauncher/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(25))
        .build()
        .map_err(|e| format!("Não foi possível preparar a conexão HTTPS: {e}"))?;
    let response = client.get(&url).header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .map_err(|e| format!("Não foi possível consultar {}: {e}", app.repo))?;
    if response.status().as_u16() == 404 {
        return Err("Este repositório ainda não tem uma release publicada.".into());
    }
    let status = response.status();
    let body = response.error_for_status()
        .map_err(|e| if status.as_u16() == 403 || status.as_u16() == 429 {
            "O GitHub limitou temporariamente as consultas. Tente novamente mais tarde.".to_string()
        } else { format!("O GitHub respondeu {status}: {e}") })?
        .text()
        .map_err(|e| format!("Não foi possível ler a resposta do GitHub: {e}"))?;
    model::parse_release(&body, app.repo)
}

pub fn load_release_cache(path: &Path) -> BTreeMap<String, Release> {
    fs::read(path).ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

pub fn save_release_cache(path: &Path, releases: &BTreeMap<String, Release>) {
    let parent = match path.parent() { Some(p) => p, None => return };
    if fs::create_dir_all(parent).is_err() { return; }
    let bytes = match serde_json::to_vec(releases) { Ok(v) => v, Err(_) => return };
    let tmp = path.with_extension(format!("tmp.{}", std::process::id()));
    if fs::write(&tmp, bytes).is_ok() {
        if fs::rename(&tmp, path).is_err() { let _ = fs::remove_file(tmp); }
    }
}

pub fn load_manifest(paths: &Paths) -> Manifest {
    Manifest::load(&paths.manifest())
}

pub fn install_or_update(
    app: &DecodeApp,
    release: &Release,
    paths: &Paths,
    previous: Option<&InstalledRecord>,
    report: &mut dyn FnMut(String),
) -> Result<InstallResult, String> {
    if !app.is_decode_project {
        return Err(format!(
            "{} ainda não tem um pacote Decode publicado. A instalação de releases upstream está bloqueada para evitar apresentar o executável original como se fosse um build Decode.",
            app.name
        ));
    }
    if release.prerelease {
        return Err("A release selecionada é uma versão de pré-lançamento; esta central não a instala automaticamente.".into());
    }
    let platform = HostPlatform::current();
    let preferred = previous.map(|r| r.kind);
    let (asset, kind) = model::select_asset(release, app, platform, preferred)
        .ok_or_else(|| format!("Não encontrei um pacote compatível para {} em {}. Consulte os downloads oficiais.", app.name, platform.label()))?;
    if !model::safe_download_url(app.repo, &release.tag_name, asset) {
        return Err("O endereço de download não corresponde à release oficial do aplicativo.".into());
    }
    if asset.size > MAX_DOWNLOAD_BYTES {
        return Err("O tamanho publicado do pacote excede o limite de 2 GiB.".into());
    }
    if let Some(old) = previous {
        if !is_expected_managed_record(app, old, paths) {
            return Err("O registro local não corresponde a um caminho gerenciado seguro. A instalação foi preservada.".into());
        }
        if is_running(&old.path) {
            return Err(format!("{} está em execução. Feche o programa antes de atualizá-lo.", app.name));
        }
    }
    report(format!("Verificando checksum de {}", asset.name));
    let client = Client::builder()
        .user_agent(concat!("DecodeOpenSourceLauncher/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Falha ao preparar o cliente HTTPS: {e}"))?;
    let expected_sha = published_sha256(&client, app, release, asset)?;
    report(format!("Baixando {} ({})", app.name, kind.label()));
    let downloaded = download_verified(&client, paths, asset, &expected_sha, report)?;
    report(format!("Integrando {} ao sistema", app.name));

    match (platform.os, kind) {
        (OperatingSystem::Windows, PackageKind::Msi) => {
            let status = Command::new("msiexec").arg("/i").arg(&downloaded).spawn()
                .map_err(|e| format!("Não foi possível abrir o instalador MSI do Windows: {e}"))?;
            let _ = status;
            Ok(InstallResult {
                record: None,
                message: "O assistente oficial do Windows foi aberto. Conclua a instalação e use “Atualizar status” para detectar o aplicativo.".into(),
            })
        }
        (OperatingSystem::Windows, PackageKind::PortableZip) => {
            install_portable_tree(app, release, kind, &downloaded, paths, previous, report)
        }
        (OperatingSystem::Linux, PackageKind::AppImage) | (OperatingSystem::Linux, PackageKind::TarGz) => {
            install_portable_tree(app, release, kind, &downloaded, paths, previous, report)
        }
        (OperatingSystem::MacOs, PackageKind::Dmg) => {
            install_macos_dmg(app, release, &downloaded, paths, previous, report)
        }
        _ => Err(format!("O pacote {} não pode ser instalado neste sistema.", kind.label())),
    }
}

fn published_sha256(client: &Client, app: &DecodeApp, release: &Release, asset: &ReleaseAsset) -> Result<String, String> {
    if let Some(digest) = &asset.digest {
        if let Some(value) = digest.strip_prefix("sha256:") {
            if is_hex_sha256(value) { return Ok(value.to_ascii_lowercase()); }
        }
    }
    let sums = release.assets.iter().find(|a| a.name == "SHA256SUMS.txt")
        .ok_or_else(|| format!("A release não publica um checksum verificável para {}. O pacote não será instalado.", asset.name))?;
    if !model::safe_download_url(app.repo, &release.tag_name, sums) {
        return Err("O endereço do arquivo de checksums não é confiável.".into());
    }
    let response = client.get(&sums.browser_download_url).send()
        .map_err(|e| format!("Não foi possível buscar SHA256SUMS.txt: {e}"))?
        .error_for_status().map_err(|e| format!("Falha ao buscar os checksums: {e}"))?;
    if response.content_length().is_some_and(|n| n > MAX_CHECKSUM_BYTES as u64) {
        return Err("O arquivo de checksums excede o limite permitido.".into());
    }
    let text = response.text().map_err(|e| format!("Não foi possível ler SHA256SUMS.txt: {e}"))?;
    if text.len() > MAX_CHECKSUM_BYTES {
        return Err("O arquivo de checksums excede o limite permitido.".into());
    }
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(hash) = parts.next() else { continue };
        let Some(name) = parts.next() else { continue };
        let name = name.trim_start_matches('*');
        if name == asset.name && is_hex_sha256(hash) {
            return Ok(hash.to_ascii_lowercase());
        }
    }
    Err(format!("Não encontrei um SHA-256 publicado para {}. O pacote não será instalado.", asset.name))
}

fn is_hex_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn download_verified(
    client: &Client,
    paths: &Paths,
    asset: &ReleaseAsset,
    expected_sha: &str,
    report: &mut dyn FnMut(String),
) -> Result<PathBuf, String> {
    if !model::safe_file_name(&asset.name) {
        return Err("O nome do arquivo de download é inválido.".into());
    }
    let downloads = paths.downloads();
    fs::create_dir_all(&downloads).map_err(|e| format!("Não foi possível criar a pasta de downloads: {e}"))?;
    let final_path = downloads.join(&asset.name);
    if final_path.is_file() && hash_file(&final_path).is_ok_and(|h| h.eq_ignore_ascii_case(expected_sha)) {
        report(format!("Reutilizando o arquivo verificado {}", asset.name));
        return Ok(final_path);
    }
    if final_path.exists() {
        fs::remove_file(&final_path).map_err(|e| format!("Não foi possível remover um download inválido: {e}"))?;
    }
    let partial = downloads.join(format!("{}.part-{}", asset.name, std::process::id()));
    let mut response = client.get(&asset.browser_download_url).send()
        .map_err(|e| format!("Falha ao baixar {}: {e}", asset.name))?
        .error_for_status().map_err(|e| format!("O servidor não disponibilizou o arquivo: {e}"))?;
    if response.content_length().is_some_and(|n| n > MAX_DOWNLOAD_BYTES) {
        return Err("O download excede o limite de 2 GiB.".into());
    }
    let mut file = File::create(&partial).map_err(|e| format!("Não foi possível criar o arquivo temporário: {e}"))?;
    let mut hasher = Sha256::new();
    let mut total: u64 = 0;
    let mut buffer = [0u8; 128 * 1024];
    loop {
        let count = response.read(&mut buffer).map_err(|e| {
            let _ = fs::remove_file(&partial);
            format!("A conexão foi interrompida durante o download: {e}")
        })?;
        if count == 0 { break; }
        total = total.checked_add(count as u64).ok_or_else(|| "O tamanho do download excedeu os limites.".to_string())?;
        if total > MAX_DOWNLOAD_BYTES {
            let _ = fs::remove_file(&partial);
            return Err("O download excedeu o limite de 2 GiB.".into());
        }
        hasher.update(&buffer[..count]);
        file.write_all(&buffer[..count]).map_err(|e| format!("Falha ao gravar o download: {e}"))?;
        if total % (5 * 1024 * 1024) < count as u64 {
            report(format!("Baixando {} — {} MiB", asset.name, total / (1024 * 1024)));
        }
    }
    file.sync_all().map_err(|e| format!("Falha ao sincronizar o download: {e}"))?;
    if asset.size > 0 && total != asset.size {
        let _ = fs::remove_file(&partial);
        return Err(format!("O tamanho recebido ({total} bytes) difere do tamanho publicado ({} bytes).", asset.size));
    }
    let actual = hex::encode(hasher.finalize());
    if !actual.eq_ignore_ascii_case(expected_sha) {
        let _ = fs::remove_file(&partial);
        return Err("A verificação SHA-256 falhou. O arquivo foi descartado por segurança.".into());
    }
    fs::rename(&partial, &final_path).map_err(|e| {
        let _ = fs::remove_file(&partial);
        format!("Não foi possível finalizar o download: {e}")
    })?;
    report("Checksum SHA-256 conferido.".into());
    Ok(final_path)
}

fn hash_file(path: &Path) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut hash = Sha256::new();
    let mut buf = [0u8; 128 * 1024];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        hash.update(&buf[..n]);
    }
    Ok(hex::encode(hash.finalize()))
}

fn install_portable_tree(
    app: &DecodeApp,
    release: &Release,
    kind: PackageKind,
    download: &Path,
    paths: &Paths,
    previous: Option<&InstalledRecord>,
    report: &mut dyn FnMut(String),
) -> Result<InstallResult, String> {
    let root = paths.apps().join(app.id);
    fs::create_dir_all(paths.apps()).map_err(|e| format!("Não foi possível criar a pasta de aplicativos: {e}"))?;
    if root.exists() && previous.is_none() {
        return Err(format!("A pasta {} já existe e não está registrada como instalação gerenciada. Ela não será sobrescrita.", root.display()));
    }
    let stage = paths.apps().join(format!(".stage-{}-{}", app.id, std::process::id()));
    if stage.exists() { fs::remove_dir_all(&stage).map_err(|e| format!("Não foi possível limpar a pasta temporária: {e}"))?; }
    fs::create_dir_all(&stage).map_err(|e| format!("Não foi possível preparar a pasta temporária: {e}"))?;
    let result = match kind {
        PackageKind::AppImage => {
            let target = stage.join(download.file_name().ok_or("Nome de download ausente")?);
            fs::copy(download, &target).map_err(|e| format!("Não foi possível preparar o AppImage: {e}"))?;
            make_executable(&target)?;
            Ok(target)
        }
        PackageKind::TarGz => unpack_tar_gz(download, &stage).and_then(|()| {
            find_app_executable(&stage, app).ok_or_else(|| format!("O arquivo não contém o executável esperado para {}.", app.name))
        }),
        PackageKind::PortableZip => unpack_zip(download, &stage).and_then(|()| {
            find_app_executable(&stage, app).ok_or_else(|| format!("O arquivo portátil não contém o executável esperado para {}.", app.name))
        }),
        other => Err(format!("Não é possível instalar {} como uma pasta portátil.", other.label())),
    };
    let staged_exe = match result {
        Ok(path) => path,
        Err(e) => { let _ = fs::remove_dir_all(&stage); return Err(e); }
    };
    if !staged_exe.starts_with(&stage) {
        let _ = fs::remove_dir_all(&stage);
        return Err("O executável preparado escapou da pasta temporária.".into());
    }
    report("Preparação concluída; substituindo a versão anterior.".into());
    let relative = staged_exe.strip_prefix(&stage).map_err(|_| "Caminho temporário inválido")?.to_path_buf();
    let backup = paths.apps().join(format!(".backup-{}-{}", app.id, std::process::id()));
    if backup.exists() { fs::remove_dir_all(&backup).map_err(|e| format!("Não foi possível remover backup antigo: {e}"))?; }
    if root.exists() {
        fs::rename(&root, &backup).map_err(|e| format!("Não foi possível preparar a atualização: {e}"))?;
    }
    if let Err(e) = fs::rename(&stage, &root) {
        if backup.exists() { let _ = fs::rename(&backup, &root); }
        let _ = fs::remove_dir_all(&stage);
        return Err(format!("Não foi possível colocar a nova versão no local definitivo: {e}"));
    }
    if backup.exists() { fs::remove_dir_all(&backup).map_err(|e| format!("A nova versão foi instalada, mas não foi possível limpar o backup: {e}"))?; }
    let installed_path = root.join(relative);
    if HostPlatform::current().os == OperatingSystem::Linux {
        let _ = write_desktop_entry(app, &installed_path);
    }
    Ok(InstallResult {
        record: Some(InstalledRecord {
            version: release.tag_name.clone(),
            kind,
            path: installed_path,
            installed_at: unix_now(),
        }),
        message: format!("{} {} instalado e verificado.", app.name, release.tag_name),
    })
}

fn safe_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty() && path.components().all(|c| matches!(c, Component::Normal(_)))
}

fn unpack_tar_gz(archive_path: &Path, destination: &Path) -> Result<(), String> {
    let file = File::open(archive_path).map_err(|e| format!("Não foi possível abrir o pacote: {e}"))?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let entries = archive.entries().map_err(|e| format!("Pacote TAR.GZ inválido: {e}"))?;
    let mut seen = 0usize;
    let mut expanded: u64 = 0;
    for entry_result in entries {
        seen += 1;
        if seen > MAX_ARCHIVE_ENTRIES { return Err("O pacote contém entradas demais.".into()); }
        let mut entry = entry_result.map_err(|e| format!("Não foi possível ler uma entrada do pacote: {e}"))?;
        let entry_type = entry.header().entry_type();
        if !entry_type.is_file() && !entry_type.is_dir() {
            return Err("O pacote contém links simbólicos ou tipos de arquivo não permitidos.".into());
        }
        let raw_path = entry.path().map_err(|e| format!("Caminho inválido no pacote: {e}"))?.into_owned();
        let relative = raw_path.strip_prefix("./").unwrap_or(&raw_path);
        if !safe_relative(relative) { return Err("O pacote contém um caminho inseguro.".into()); }
        let out = destination.join(relative);
        if !out.starts_with(destination) { return Err("O pacote tenta gravar fora da pasta temporária.".into()); }
        if entry_type.is_dir() {
            fs::create_dir_all(&out).map_err(|e| format!("Não foi possível criar uma pasta do pacote: {e}"))?;
            continue;
        }
        let size = entry.header().size().map_err(|e| format!("Tamanho de arquivo inválido: {e}"))?;
        expanded = expanded.checked_add(size).ok_or_else(|| "O pacote descompactado é grande demais.".to_string())?;
        if expanded > MAX_DOWNLOAD_BYTES { return Err("O conteúdo descompactado excede 2 GiB.".into()); }
        if let Some(parent) = out.parent() { fs::create_dir_all(parent).map_err(|e| format!("Não foi possível criar a pasta do aplicativo: {e}"))?; }
        let mut output = OpenOptions::new().write(true).create_new(true).open(&out)
            .map_err(|e| format!("Não foi possível criar {}: {e}", out.display()))?;
        std::io::copy(&mut entry, &mut output).map_err(|e| format!("Falha ao extrair {}: {e}", out.display()))?;
        #[cfg(unix)]
        if entry.header().mode().unwrap_or(0) & 0o111 != 0 {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&out, fs::Permissions::from_mode(0o755)).map_err(|e| format!("Não foi possível marcar o executável: {e}"))?;
        }
    }
    Ok(())
}

fn unpack_zip(archive_path: &Path, destination: &Path) -> Result<(), String> {
    let file = File::open(archive_path).map_err(|e| format!("Não foi possível abrir o ZIP: {e}"))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Pacote ZIP inválido: {e}"))?;
    if archive.len() > MAX_ARCHIVE_ENTRIES { return Err("O pacote contém entradas demais.".into()); }
    let mut expanded: u64 = 0;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|e| format!("Não foi possível ler uma entrada ZIP: {e}"))?;
        let Some(relative) = entry.enclosed_name().as_deref().map(Path::to_path_buf) else {
            return Err("O ZIP contém um caminho de arquivo inseguro.".into());
        };
        if !safe_relative(&relative) { return Err("O ZIP contém um caminho de arquivo inseguro.".into()); }
        if entry.unix_mode().is_some_and(|mode| mode & 0o170000 == 0o120000) {
            return Err("O ZIP contém links simbólicos que não serão extraídos.".into());
        }
        let out = destination.join(&relative);
        if !out.starts_with(destination) { return Err("O ZIP tenta gravar fora da pasta temporária.".into()); }
        if entry.is_dir() {
            fs::create_dir_all(&out).map_err(|e| format!("Não foi possível criar uma pasta do ZIP: {e}"))?;
            continue;
        }
        expanded = expanded.checked_add(entry.size()).ok_or_else(|| "O ZIP descompactado é grande demais.".to_string())?;
        if expanded > MAX_DOWNLOAD_BYTES { return Err("O conteúdo descompactado excede 2 GiB.".into()); }
        if let Some(parent) = out.parent() { fs::create_dir_all(parent).map_err(|e| format!("Não foi possível criar a pasta do ZIP: {e}"))?; }
        let mut output = OpenOptions::new().write(true).create_new(true).open(&out)
            .map_err(|e| format!("Não foi possível criar {}: {e}", out.display()))?;
        std::io::copy(&mut entry, &mut output).map_err(|e| format!("Falha ao extrair {}: {e}", out.display()))?;
        #[cfg(unix)]
        if entry.unix_mode().is_some_and(|mode| mode & 0o111 != 0) {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&out, fs::Permissions::from_mode(0o755)).map_err(|e| format!("Não foi possível marcar o executável: {e}"))?;
        }
    }
    Ok(())
}

fn find_app_executable(root: &Path, app: &DecodeApp) -> Option<PathBuf> {
    let candidate_names = [
        app.id.to_string(),
        app.asset_stem.to_string(),
        format!("{}.exe", app.id),
        format!("{}.exe", app.asset_stem),
        format!("{}.exe", app.name),
    ];
    let mut pending = vec![(root.to_path_buf(), 0usize)];
    let mut visited = 0usize;
    while let Some((dir, depth)) = pending.pop() {
        if depth > 5 || visited > 10_000 { continue; }
        visited += 1;
        let entries = fs::read_dir(&dir).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            let file_type = entry.file_type().ok()?;
            if file_type.is_symlink() { continue; }
            if file_type.is_dir() {
                pending.push((path, depth + 1));
            } else if file_type.is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                if candidate_names.iter().any(|candidate| name.eq_ignore_ascii_case(candidate)) {
                    #[cfg(unix)]
                    make_executable(&path).ok()?;
                    return Some(path);
                }
            }
        }
    }
    None
}

fn make_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::symlink_metadata(path).map_err(|e| format!("Não foi possível verificar o executável: {e}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err("O executável não é um arquivo regular.".into());
        }
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).map_err(|e| format!("Não foi possível tornar o aplicativo executável: {e}"))?;
    }
    #[cfg(not(unix))]
    {
        if !path.is_file() { return Err("O executável não é um arquivo regular.".into()); }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn install_macos_dmg(
    app: &DecodeApp,
    release: &Release,
    download: &Path,
    paths: &Paths,
    previous: Option<&InstalledRecord>,
    report: &mut dyn FnMut(String),
) -> Result<InstallResult, String> {
    let home = dirs::home_dir().ok_or_else(|| "Não foi possível localizar a pasta pessoal.".to_string())?;
    let apps = home.join("Applications");
    fs::create_dir_all(&apps).map_err(|e| format!("Não foi possível criar ~/Applications: {e}"))?;
    let target = apps.join(format!("{}.app", app.upstream_name));
    if target.exists() && previous.is_none() {
        return Err(format!("{} já existe e não está registrada como instalação gerenciada.", target.display()));
    }
    if let Some(old) = previous {
        if !is_expected_managed_record(app, old, paths) {
            return Err("O registro da instalação anterior não é seguro.".into());
        }
        if is_running(&old.path) {
            return Err(format!("{} está em execução. Feche o programa antes de atualizá-lo.", app.name));
        }
    }
    let mount = tempfile::tempdir().map_err(|e| format!("Não foi possível criar uma pasta de montagem: {e}"))?;
    let attach = Command::new("hdiutil").args(["attach", "-nobrowse", "-readonly", "-mountpoint"])
        .arg(mount.path()).arg(download).output().map_err(|e| format!("Não foi possível montar o DMG: {e}"))?;
    if !attach.status.success() {
        return Err(format!("O macOS não conseguiu montar o DMG: {}", String::from_utf8_lossy(&attach.stderr)));
    }
    report("DMG montado; verificando o aplicativo.".into());
    let source = mount.path().join(format!("{}.app", app.upstream_name));
    let result = if fs::symlink_metadata(&source).is_ok_and(|metadata| metadata.file_type().is_dir()) {
        install_mounted_bundle(app, release, &source, &target)
    } else {
        Err("O DMG não contém o pacote .app esperado.".into())
    };
    let _ = Command::new("hdiutil").args(["detach", "-quiet"]).arg(mount.path()).status();
    result
}

#[cfg(target_os = "macos")]
fn install_mounted_bundle(
    app: &DecodeApp,
    release: &Release,
    source: &Path,
    target: &Path,
) -> Result<InstallResult, String> {
    let parent = target.parent().ok_or_else(|| "Pasta de destino inválida.".to_string())?;
    let staging = parent.join(format!(".{}.stage-{}", app.id, std::process::id()));
    let backup = parent.join(format!(".{}.backup-{}", app.id, std::process::id()));
    if staging.exists() { fs::remove_dir_all(&staging).map_err(|e| format!("Não foi possível limpar a pasta temporária: {e}"))?; }
    let copied = Command::new("ditto").arg(source).arg(&staging).status()
        .map_err(|e| format!("Não foi possível copiar o aplicativo do DMG: {e}"))?;
    if !copied.success() {
        let _ = fs::remove_dir_all(&staging);
        return Err(format!("A cópia do aplicativo falhou: {copied}"));
    }
    if backup.exists() { fs::remove_dir_all(&backup).map_err(|e| format!("Não foi possível limpar o backup: {e}"))?; }
    if target.exists() {
        if let Err(error) = fs::rename(target, &backup) {
            let _ = fs::remove_dir_all(&staging);
            return Err(format!("Não foi possível preparar a versão anterior: {error}"));
        }
    }
    if let Err(error) = fs::rename(&staging, target) {
        if backup.exists() { let _ = fs::rename(&backup, target); }
        let _ = fs::remove_dir_all(&staging);
        return Err(format!("Não foi possível instalar o aplicativo: {error}"));
    }
    if backup.exists() { fs::remove_dir_all(&backup).map_err(|e| format!("Nova versão instalada, mas não foi possível limpar o backup: {e}"))?; }
    Ok(InstallResult {
        record: Some(InstalledRecord { version: release.tag_name.clone(), kind: PackageKind::Dmg, path: target.to_path_buf(), installed_at: unix_now() }),
        message: format!("{} {} instalado em ~/Applications.", app.name, release.tag_name),
    })
}

#[cfg(not(target_os = "macos"))]
fn install_macos_dmg(
    _app: &DecodeApp,
    _release: &Release,
    _download: &Path,
    _paths: &Paths,
    _previous: Option<&InstalledRecord>,
    _report: &mut dyn FnMut(String),
) -> Result<InstallResult, String> {
    Err("O pacote DMG só pode ser instalado no macOS.".into())
}

pub fn is_expected_managed_record(app: &DecodeApp, record: &InstalledRecord, paths: &Paths) -> bool {
    let expected_apps_root = paths.apps().join(app.id);
    if record.path.starts_with(&expected_apps_root) { return true; }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            return record.path == home.join("Applications").join(format!("{}.app", app.upstream_name));
        }
    }
    false
}

pub fn remove_managed(app: &DecodeApp, record: &InstalledRecord, paths: &Paths) -> Result<(), String> {
    if !is_expected_managed_record(app, record, paths) {
        return Err("Por segurança, só é possível remover aplicativos registrados em uma pasta gerenciada por esta central.".into());
    }
    if is_running(&record.path) {
        return Err(format!("{} está em execução. Feche o programa antes de desinstalá-lo.", app.name));
    }
    let root = paths.apps().join(app.id);
    if record.path.starts_with(&root) {
        if root.exists() { fs::remove_dir_all(&root).map_err(|e| format!("Não foi possível remover a instalação: {e}"))?; }
    } else {
        #[cfg(target_os = "macos")]
        if let Some(home) = dirs::home_dir() {
            let expected = home.join("Applications").join(format!("{}.app", app.upstream_name));
            if record.path == expected && expected.is_dir() {
                fs::remove_dir_all(expected).map_err(|e| format!("Não foi possível remover o aplicativo: {e}"))?;
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        let desktop = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("applications")
            .join(format!("{}.desktop", app.bundle_id()));
        if let Ok(content) = fs::read_to_string(&desktop) {
            if content.contains("X-Decode-Open-Source-Hub=true") {
                let _ = fs::remove_file(desktop);
            }
        }
    }
    Ok(())
}

pub fn detect_external(app: &DecodeApp) -> Option<ExternalInstall> {
    let platform = HostPlatform::current();
    match platform.os {
        OperatingSystem::Linux => {
            let mut candidates = Vec::new();
            if let Some(home) = dirs::home_dir() {
                candidates.extend([
                    home.join("Applications").join(format!("{}.AppImage", app.id)),
                    home.join("Applications").join(format!("{}.AppImage", app.name)),
                    home.join(".local/bin").join(app.id),
                ]);
            }
            candidates.extend([PathBuf::from("/usr/local/bin").join(app.id), PathBuf::from("/usr/bin").join(app.id)]);
            for path in candidates {
                if path.is_file() { return Some(ExternalInstall { target: path.to_string_lossy().to_string(), version: None }); }
            }
            let flatpak_id = app.upstream_bundle_id();
            if Command::new("flatpak").args(["info", "--user", &flatpak_id]).output().is_ok_and(|o| o.status.success())
                || Command::new("flatpak").args(["info", "--system", &flatpak_id]).output().is_ok_and(|o| o.status.success())
            {
                return Some(ExternalInstall { target: format!("flatpak:{flatpak_id}"), version: None });
            }
        }
        OperatingSystem::Windows => {
            let mut roots = Vec::new();
            if let Some(p) = std::env::var_os("ProgramFiles") {
                let base = PathBuf::from(p);
                roots.push(base.join(app.upstream_name));
                roots.push(base.join(app.name));
            }
            if let Some(p) = std::env::var_os("LOCALAPPDATA") {
                let base = PathBuf::from(p);
                roots.push(base.join("Programs").join(app.upstream_name));
                roots.push(base.join("Programs").join(app.name));
            }
            if let Some(p) = dirs::home_dir() { roots.push(p.join("Applications").join(app.upstream_name));
                roots.push(p.join("Applications").join(app.name)); }
            for root in roots {
                if root.exists() {
                    if let Some(exe) = find_app_executable(&root, app) {
                        return Some(ExternalInstall { target: exe.to_string_lossy().to_string(), version: None });
                    }
                }
            }
        }
        OperatingSystem::MacOs => {
            let bundle_name = format!("{}.app", app.upstream_name);
            let mut roots = vec![PathBuf::from("/Applications").join(&bundle_name)];
            if let Some(home) = dirs::home_dir() { roots.push(home.join("Applications").join(bundle_name)); }
            for path in roots {
                if path.is_dir() { return Some(ExternalInstall { target: path.to_string_lossy().to_string(), version: None }); }
            }
        }
        OperatingSystem::Unsupported => {}
    }
    None
}

pub fn launch(target: &str) -> Result<(), String> {
    if let Some(bundle_id) = target.strip_prefix("flatpak:") {
        Command::new("flatpak").args(["run", bundle_id]).spawn()
            .map(|_| ()).map_err(|e| format!("Não foi possível iniciar o Flatpak: {e}"))
    } else {
        let path = PathBuf::from(target);
        if !path.exists() {
            return Err(format!("O aplicativo não foi encontrado em {}. Atualize o status e tente reinstalar.", path.display()));
        }
        #[cfg(target_os = "macos")]
        if path.extension().is_some_and(|ext| ext == "app") || path.is_dir() {
            return Command::new("open").arg("-a").arg(&path).spawn()
                .map(|_| ()).map_err(|e| format!("Não foi possível abrir o aplicativo: {e}"));
        }
        #[cfg(target_os = "windows")]
        if path.is_dir() {
            let exe = path.join("launcher.exe");
            if exe.exists() {
                return Command::new(exe).spawn().map(|_| ()).map_err(|e| format!("Não foi possível iniciar o aplicativo: {e}"));
            }
        }
        Command::new(&path).spawn().map(|_| ()).map_err(|e| format!("Não foi possível iniciar {}: {e}", path.display()))
    }
}

pub fn is_running(path: &Path) -> bool {
    let Some(name) = path.file_name().map(|s| s.to_string_lossy().to_string()) else { return false };
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if !entry.file_name().to_string_lossy().bytes().all(|b| b.is_ascii_digit()) { continue; }
                if let Ok(cmdline) = fs::read(entry.path().join("cmdline")) {
                    if cmdline.windows(path.to_string_lossy().len()).any(|w| w == path.to_string_lossy().as_bytes()) {
                        return true;
                    }
                }
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = Command::new("tasklist").args(["/FO", "CSV", "/NH"]).output() {
            let text = String::from_utf8_lossy(&output.stdout).to_ascii_lowercase();
            if text.lines().any(|line| line.split('"').nth(1).is_some_and(|exe| exe == name.to_ascii_lowercase())) {
                return true;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("pgrep").args(["-x", &name]).output() {
            if output.status.success() { return true; }
        }
    }
    false
}

#[cfg(unix)]
fn make_exec_string(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "")
        .replace('\r', "")
}

#[cfg(target_os = "linux")]
fn write_desktop_entry(app: &DecodeApp, executable: &Path) -> Result<(), String> {
    let data = dirs::data_local_dir().ok_or_else(|| "Pasta de dados do usuário indisponível.".to_string())?;
    let folder = data.join("applications");
    fs::create_dir_all(&folder).map_err(|e| format!("Não foi possível criar o menu de aplicativos: {e}"))?;
    let file = folder.join(format!("{}.desktop", app.bundle_id()));
    if file.exists() {
        let old = fs::read_to_string(&file).unwrap_or_default();
        if !old.contains("X-Decode-Open-Source-Hub=true") {
            return Err("Já existe uma entrada de menu não gerenciada com este identificador.".into());
        }
    }
    let exec = make_exec_string(executable);
    let content = format!(
        "[Desktop Entry]\nType=Application\nName={}\nComment={}\nExec=\"{}\"\nTerminal=false\nCategories=Graphics;Utility;\nX-Decode-Open-Source-Hub=true\n",
        app.name.replace(['\n', '\r'], " "),
        app.tagline.replace(['\n', '\r'], " "),
        exec
    );
    fs::write(file, content).map_err(|e| format!("Não foi possível atualizar o menu de aplicativos: {e}"))
}

#[cfg(not(target_os = "linux"))]
fn write_desktop_entry(_app: &DecodeApp, _executable: &Path) -> Result<(), String> { Ok(()) }

fn unix_now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs().min(i64::MAX as u64) as i64)
}

pub fn validate_archive_name(name: &str) -> bool {
    model::safe_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_names_reject_paths_and_hidden_special_cases() {
        assert!(!validate_archive_name("../bad"));
        assert!(!validate_archive_name("folder/file"));
        assert!(!validate_archive_name(".profile"));
        assert!(validate_archive_name("app-0.1.0-linux-x86_64.AppImage"));
    }

    #[test]
    fn manifest_handles_corrupt_input_without_panicking() {
        let dir = tempfile::tempdir().expect("temporary directory");
        let path = dir.path().join("installed.json");
        fs::write(&path, b"{broken").expect("write fixture");
        assert!(Manifest::load(&path).apps.is_empty());
    }
}
