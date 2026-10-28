#[derive(Clone, Copy, Debug)]
pub struct DecodeApp {
    pub id: &'static str,
    pub name: &'static str,
    /// Upstream display name retained for compatibility with original release binaries.
    pub upstream_name: &'static str,
    /// Stable Decode identity for menus and future branded builds.
    pub product_id: &'static str,
    pub tagline: &'static str,
    pub category: &'static str,
    pub repo: &'static str,
    pub asset_stem: &'static str,
    pub license: &'static str,
    pub color: [u8; 3],
    pub is_decode_project: bool,
}

impl DecodeApp {
    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn website_url(&self) -> String {
        "https://decode-open-source-hub.vercel.app".to_string()
    }

    pub fn preparation_url(&self) -> String {
        "https://github.com/Kaiqueaurelio/decodecad/actions/workflows/decode-suite-downstream-build.yml".to_string()
    }

    pub fn icon_url(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/Kaiqueaurelio/decodecad/main/open-source-hub/assets/icons/{}.svg",
            self.product_id
        )
    }

    pub fn icon_svg(&self) -> &'static str {
        match self.product_id {
            "decode-photo" => include_str!("../../open-source-hub/assets/icons/decode-photo.svg"),
            "decode-light" => include_str!("../../open-source-hub/assets/icons/decode-light.svg"),
            "decode-film" => include_str!("../../open-source-hub/assets/icons/decode-film.svg"),
            "decode-design" => include_str!("../../open-source-hub/assets/icons/decode-design.svg"),
            "decode-vector" => include_str!("../../open-source-hub/assets/icons/decode-vector.svg"),
            "decode-pdf" => include_str!("../../open-source-hub/assets/icons/decode-pdf.svg"),
            "decode-effects" => include_str!("../../open-source-hub/assets/icons/decode-effects.svg"),
            "decode-writer" => include_str!("../../open-source-hub/assets/icons/decode-writer.svg"),
            "decode-slides" => include_str!("../../open-source-hub/assets/icons/decode-slides.svg"),
            "decode-sheets" => include_str!("../../open-source-hub/assets/icons/decode-sheets.svg"),
            "decode-draft" => include_str!("../../open-source-hub/assets/icons/decode-draft.svg"),
            "decode-audio" => include_str!("../../open-source-hub/assets/icons/decode-audio.svg"),
            "decode-ui" => include_str!("../../open-source-hub/assets/icons/decode-ui.svg"),
            "decode-cad" => include_str!("../../open-source-hub/assets/icons/decode-cad.svg"),
            _ => include_str!("../../open-source-hub/assets/icons/decode-hub-mark.svg"),
        }
    }

    pub fn bundle_id(&self) -> String {
        format!("com.decodeanalytics.{}", self.product_id)
    }

    pub fn upstream_bundle_id(&self) -> String {
        format!("com.decode.{}", self.id)
    }

}

/// Decode-branded entries shown in the launcher. Upstream repository and asset identifiers remain
/// intact until independent Decode-branded builds exist.
pub const APPS: &[DecodeApp] = &[
    DecodeApp { id: "photocraft", product_id: "decode-photo", name: "Decode Photo", upstream_name: "PhotoCraft", tagline: "Camadas, máscaras e edição de imagens", category: "Imagem e fotografia", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "photocraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "vectorcraft", product_id: "decode-vector", name: "Decode Vector", upstream_name: "VectorCraft", tagline: "Ilustração vetorial e desenho escalável", category: "Design e ilustração", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "vectorcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "filmcraft", product_id: "decode-film", name: "Decode Film", upstream_name: "FilmCraft", tagline: "Edição de vídeo, cor e áudio", category: "Vídeo e motion", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "filmcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "lightcraft", product_id: "decode-light", name: "Decode Light", upstream_name: "LightCraft", tagline: "Fotografia RAW e edição não destrutiva", category: "Imagem e fotografia", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "lightcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "pdfcraft", product_id: "decode-pdf", name: "Decode PDF", upstream_name: "PdfCraft", tagline: "Leitura, organização e proteção de PDFs", category: "Documentos e escritório", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "pdfcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "effectcraft", product_id: "decode-effects", name: "Decode Effects", upstream_name: "EffectCraft", tagline: "Motion graphics e efeitos visuais", category: "Vídeo e motion", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "effectcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "designcraft", product_id: "decode-design", name: "Decode Design", upstream_name: "DesignCraft", tagline: "Diagramação, layout e publicação", category: "Design e ilustração", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "designcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "figcraft", product_id: "decode-ui", name: "Decode UI", upstream_name: "FigCraft", tagline: "Design de interfaces e protótipos", category: "Design e ilustração", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "figcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "wordcraft", product_id: "decode-writer", name: "Decode Writer", upstream_name: "WordCraft", tagline: "Documentos e edição de texto", category: "Documentos e escritório", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "wordcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "soundcraft", product_id: "decode-audio", name: "Decode Audio", upstream_name: "SoundCraft", tagline: "Gravação, edição e mixagem de áudio", category: "Áudio", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "soundcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "deckcraft", product_id: "decode-slides", name: "Decode Slides", upstream_name: "DeckCraft", tagline: "Apresentações e conjuntos de slides", category: "Documentos e escritório", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "deckcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "cadcraft", product_id: "decode-draft", name: "Decode Draft", upstream_name: "CADCraft", tagline: "Desenho técnico assistido por computador", category: "CAD e engenharia", repo: "Kaiqueaurelio/decodecad", asset_stem: "cadcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "gridcraft", product_id: "decode-sheets", name: "Decode Sheets", upstream_name: "GridCraft", tagline: "Planilhas e análise de dados", category: "Documentos e escritório", repo: "Kaiqueaurelio/decode-open-source-hub", asset_stem: "gridcraft", license: "MIT OR Apache-2.0", color: [76,133,247], is_decode_project: false },
    DecodeApp { id: "decodecad", product_id: "decode-cad", name: "Decode CAD", upstream_name: "Decode CAD", tagline: "CAD 2D/3D mantido pela Decode Analytics", category: "CAD e engenharia", repo: "Kaiqueaurelio/decodecad", asset_stem: "decode-cad", license: "MIT / Apache-2.0", color: [76,133,247], is_decode_project: true },
];

pub fn categories() -> Vec<&'static str> {
    let mut out = vec!["Todos"];
    for app in APPS {
        if !out.contains(&app.category) {
            out.push(app.category);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_thirteen_rebranded_apps_and_decode_cad() {
        assert_eq!(APPS.len(), 14);
        assert_eq!(APPS.iter().filter(|a| !a.is_decode_project).count(), 13);
        assert_eq!(APPS.iter().filter(|a| a.is_decode_project).count(), 1);
        assert!(APPS.iter().all(|a| a.name.starts_with("Decode ")));
    }

    #[test]
    fn catalog_ids_are_unique() {
        for (i, app) in APPS.iter().enumerate() {
            assert!(!APPS[..i].iter().any(|other| other.id == app.id));
        }
    }
}
