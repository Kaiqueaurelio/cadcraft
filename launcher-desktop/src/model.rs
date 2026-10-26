use serde::{Deserialize, Serialize};

use crate::catalog::DecodeApp;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Release {
    #[serde(default)]
    pub tag_name: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseAsset {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub browser_download_url: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub digest: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageKind {
    Msi,
    PortableZip,
    Dmg,
    AppImage,
    TarGz,
}

impl PackageKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Msi => "Instalador do Windows (.msi)",
            Self::PortableZip => "Pacote portátil (.zip)",
            Self::Dmg => "Imagem de instalação do macOS (.dmg)",
            Self::AppImage => "Aplicativo Linux (.AppImage)",
            Self::TarGz => "Pacote Linux (.tar.gz)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Architecture {
    X64,
    X86,
    Arm64,
    Riscv64,
    Unsupported,
}

#[derive(Clone, Copy, Debug)]
pub struct HostPlatform {
    pub os: OperatingSystem,
    pub arch: Architecture,
}

impl HostPlatform {
    pub fn current() -> Self {
        let os = match std::env::consts::OS {
            "windows" => OperatingSystem::Windows,
            "macos" => OperatingSystem::MacOs,
            "linux" => OperatingSystem::Linux,
            _ => OperatingSystem::Unsupported,
        };
        let arch = match std::env::consts::ARCH {
            "x86_64" => Architecture::X64,
            "x86" => Architecture::X86,
            "aarch64" => Architecture::Arm64,
            "riscv64" => Architecture::Riscv64,
            _ => Architecture::Unsupported,
        };
        Self { os, arch }
    }

    pub fn label(self) -> &'static str {
        match (self.os, self.arch) {
            (OperatingSystem::Windows, Architecture::X64) => "Windows x64",
            (OperatingSystem::Windows, Architecture::X86) => "Windows x86",
            (OperatingSystem::Windows, Architecture::Arm64) => "Windows ARM64",
            (OperatingSystem::MacOs, Architecture::X64) => "macOS Intel",
            (OperatingSystem::MacOs, Architecture::Arm64) => "macOS Apple Silicon",
            (OperatingSystem::Linux, Architecture::X64) => "Linux x86_64",
            (OperatingSystem::Linux, Architecture::Arm64) => "Linux ARM64",
            (OperatingSystem::Linux, Architecture::Riscv64) => "Linux RISC-V",
            _ => "Sistema não suportado",
        }
    }
}

pub fn safe_file_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 240
        && name != "."
        && name != ".."
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '+'))
        && !name.starts_with('.')
}

pub fn safe_download_url(repo: &str, tag: &str, asset: &ReleaseAsset) -> bool {
    if !safe_file_name(&asset.name) || tag.is_empty() || tag.len() > 64 || tag.contains(['/', '\\', '?', '#']) {
        return false;
    }
    let expected = format!("https://github.com/{repo}/releases/download/{tag}/{}", asset.name);
    asset.browser_download_url == expected
}

fn version_matches(release: &Release, app: &DecodeApp, asset_name: &str, suffix: &str) -> bool {
    let Some(after_stem) = asset_name.strip_prefix(&format!("{}-", app.asset_stem)) else {
        return false;
    };
    let Some(version_text) = after_stem.strip_suffix(suffix) else {
        return false;
    };
    let release_version = release.tag_name.trim_start_matches('v');
    let file_version = version_text.trim_start_matches('v');
    if let (Ok(a), Ok(b)) = (semver::Version::parse(release_version), semver::Version::parse(file_version)) {
        a == b
    } else {
        release_version == file_version
    }
}

fn find_kind<'a>(
    release: &'a Release,
    app: &DecodeApp,
    kind: PackageKind,
    suffixes: &[&str],
) -> Option<&'a ReleaseAsset> {
    release.assets.iter().find(|asset| {
        safe_download_url(app.repo, &release.tag_name, asset)
            && suffixes.iter().any(|suffix| version_matches(release, app, &asset.name, suffix))
            && match kind {
                PackageKind::Msi => asset.name.to_ascii_lowercase().ends_with(".msi"),
                PackageKind::PortableZip => asset.name.to_ascii_lowercase().ends_with(".zip"),
                PackageKind::Dmg => asset.name.to_ascii_lowercase().ends_with(".dmg"),
                PackageKind::AppImage => asset.name.to_ascii_lowercase().ends_with(".appimage"),
                PackageKind::TarGz => asset.name.to_ascii_lowercase().ends_with(".tar.gz"),
            }
    })
}

pub fn select_asset<'a>(
    release: &'a Release,
    app: &DecodeApp,
    platform: HostPlatform,
    preferred: Option<PackageKind>,
) -> Option<(&'a ReleaseAsset, PackageKind)> {
    match platform.os {
        OperatingSystem::Windows => {
            let tokens: &[&str] = match platform.arch {
                Architecture::Arm64 => &["windows-arm64", "windows-x64", "windows-x86"],
                Architecture::X86 => &["windows-x86"],
                _ => &["windows-x64", "windows-x86"],
            };
            let mut candidates = Vec::new();
            if let Some(kind) = preferred { candidates.push(kind); }
            for kind in [PackageKind::Msi, PackageKind::PortableZip] {
                if !candidates.contains(&kind) { candidates.push(kind); }
            }
            for kind in candidates {
                let suffixes: Vec<String> = tokens.iter().flat_map(|t| match kind {
                    PackageKind::Msi => vec![format!("-{t}.msi")],
                    PackageKind::PortableZip => vec![format!("-{t}-portable.zip"), format!("-{t}.zip")],
                    _ => Vec::new(),
                }).collect();
                let refs: Vec<&str> = suffixes.iter().map(String::as_str).collect();
                if let Some(asset) = find_kind(release, app, kind, &refs) { return Some((asset, kind)); }
            }
        }
        OperatingSystem::MacOs => {
            let tokens: &[&str] = match platform.arch {
                Architecture::Arm64 => &["macos-universal", "macos-aarch64", "macos-arm64", "macos-x86_64"],
                _ => &["macos-universal", "macos-x86_64"],
            };
            if preferred.is_none_or(|p| p == PackageKind::Dmg) {
                for token in tokens {
                    let suffix = format!("-{token}.dmg");
                    if let Some(asset) = find_kind(release, app, PackageKind::Dmg, &[&suffix]) { return Some((asset, PackageKind::Dmg)); }
                }
            }
        }
        OperatingSystem::Linux => {
            let token = match platform.arch {
                Architecture::X64 => "linux-x86_64",
                Architecture::Arm64 => "linux-aarch64",
                Architecture::Riscv64 => "linux-riscv64",
                _ => return None,
            };
            let kinds = if let Some(p) = preferred {
                vec![p, PackageKind::AppImage, PackageKind::TarGz]
            } else {
                vec![PackageKind::AppImage, PackageKind::TarGz]
            };
            for kind in kinds {
                if kind == PackageKind::AppImage {
                    let suffix = format!("-{token}.AppImage");
                    if let Some(asset) = find_kind(release, app, kind, &[&suffix]) { return Some((asset, kind)); }
                } else if kind == PackageKind::TarGz {
                    let suffix = format!("-{token}.tar.gz");
                    if let Some(asset) = find_kind(release, app, kind, &[&suffix]) { return Some((asset, kind)); }
                }
            }
        }
        OperatingSystem::Unsupported => {}
    }
    None
}

pub fn parse_release(body: &str, repo: &str) -> Result<Release, String> {
    if body.len() > 4 * 1024 * 1024 {
        return Err("A resposta do GitHub excedeu o limite permitido.".into());
    }
    let release: Release = serde_json::from_str(body).map_err(|e| format!("Resposta do GitHub inválida: {e}"))?;
    if release.tag_name.is_empty() || release.tag_name.len() > 64 || release.tag_name.contains(['/', '\\']) {
        return Err("O GitHub retornou uma versão inválida.".into());
    }
    if release.html_url != format!("https://github.com/{repo}/releases/tag/{}", release.tag_name)
        && release.html_url != format!("https://github.com/{repo}/releases")
        && !release.html_url.is_empty()
    {
        return Err("A página da release não pertence ao repositório esperado.".into());
    }
    if release.prerelease {
        // The endpoint /releases/latest normally excludes pre-releases; retain the flag for display.
    }
    Ok(release)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::APPS;

    fn release(tag: &str, repo: &str, names: &[&str]) -> Release {
        Release {
            tag_name: tag.to_string(),
            name: tag.to_string(),
            html_url: format!("https://github.com/{repo}/releases/tag/{tag}"),
            assets: names.iter().map(|name| ReleaseAsset {
                name: (*name).to_string(),
                browser_download_url: format!("https://github.com/{repo}/releases/download/{tag}/{name}"),
                size: 123,
                digest: None,
            }).collect(),
            ..Release::default()
        }
    }

    #[test]
    fn linux_prefers_appimage_and_matching_architecture() {
        let app = APPS.iter().find(|a| a.id == "photocraft").expect("test app exists");
        let r = release("v0.5.0", app.repo, &[
            "photocraft-0.5.0-linux-aarch64.AppImage",
            "photocraft-0.5.0-linux-x86_64.tar.gz",
            "photocraft-0.5.0-linux-x86_64.AppImage",
        ]);
        let found = select_asset(&r, app, HostPlatform { os: OperatingSystem::Linux, arch: Architecture::X64 }, None).expect("matching asset");
        assert_eq!(found.0.name, "photocraft-0.5.0-linux-x86_64.AppImage");
        assert_eq!(found.1, PackageKind::AppImage);
    }

    #[test]
    fn server_or_cli_assets_are_not_mistaken_for_the_app() {
        let app = APPS.iter().find(|a| a.id == "figcraft").expect("test app exists");
        let r = release("0.5.1", app.repo, &["figcraft-server-0.5.1-linux-x86_64.tar.gz", "figcraft-web-0.5.1.zip"]);
        assert!(select_asset(&r, app, HostPlatform { os: OperatingSystem::Linux, arch: Architecture::X64 }, None).is_none());
    }

    #[test]
    fn decode_cad_uses_the_hyphenated_release_asset_stem() {
        let app = APPS.iter().find(|a| a.id == "decodecad").expect("test app exists");
        let r = release("v0.12.1", app.repo, &["decode-cad-0.12.1-linux-x86_64.AppImage"]);
        assert!(select_asset(&r, app, HostPlatform { os: OperatingSystem::Linux, arch: Architecture::X64 }, None).is_some());
    }

    #[test]
    fn rejects_url_from_another_host_or_repository() {
        let app = APPS.iter().find(|a| a.id == "photocraft").expect("test app exists");
        let mut asset = ReleaseAsset {
            name: "photocraft-0.5.0-linux-x86_64.AppImage".into(),
            browser_download_url: "https://evil.example/malware".into(),
            size: 10,
            digest: None,
        };
        assert!(!safe_download_url(app.repo, "0.5.0", &asset));
        asset.browser_download_url = format!("https://github.com/{}/releases/download/v0.5.0/{}", app.repo, asset.name);
        assert!(safe_download_url(app.repo, "v0.5.0", &asset));
    }
}
