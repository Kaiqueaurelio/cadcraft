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
        if self.is_decode_project {
            self.repo_url()
        } else {
            format!("https://getartcraft.com/apps/{}", self.id)
        }
    }

    pub fn bundle_id(&self) -> String {
        format!("com.decodeanalytics.{}", self.product_id)
    }

    pub fn upstream_bundle_id(&self) -> String {
        format!("ai.storyteller.{}", self.id)
    }

}

/// Decode-branded entries shown in the launcher. Upstream repository and asset identifiers remain
/// intact until independent Decode-branded builds exist.
pub const APPS: &[DecodeApp] = &[
    DecodeApp { id: "photocraft", product_id: "decode-photo", name: "Decode Photo", upstream_name: "PhotoCraft", tagline: "Camadas, máscaras e edição de imagens", category: "Imagem e fotografia", repo: "storytold/photocraft", asset_stem: "photocraft", license: "Apache-2.0", color: [47,123,245], is_decode_project: false },
    DecodeApp { id: "vectorcraft", product_id: "decode-vector", name: "Decode Vector", upstream_name: "VectorCraft", tagline: "Ilustração vetorial e desenho escalável", category: "Design e ilustração", repo: "storytold/vectorcraft", asset_stem: "vectorcraft", license: "Apache-2.0", color: [232,87,63], is_decode_project: false },
    DecodeApp { id: "filmcraft", product_id: "decode-film", name: "Decode Film", upstream_name: "FilmCraft", tagline: "Edição de vídeo, cor e áudio", category: "Vídeo e motion", repo: "storytold/filmcraft", asset_stem: "filmcraft", license: "Apache-2.0", color: [139,92,246], is_decode_project: false },
    DecodeApp { id: "lightcraft", product_id: "decode-light", name: "Decode Light", upstream_name: "LightCraft", tagline: "Fotografia RAW e edição não destrutiva", category: "Imagem e fotografia", repo: "storytold/lightcraft", asset_stem: "lightcraft", license: "Apache-2.0", color: [242,165,22], is_decode_project: false },
    DecodeApp { id: "pdfcraft", product_id: "decode-pdf", name: "Decode PDF", upstream_name: "PdfCraft", tagline: "Leitura, organização e proteção de PDFs", category: "Documentos e escritório", repo: "storytold/pdfcraft", asset_stem: "pdfcraft", license: "Apache-2.0", color: [18,165,138], is_decode_project: false },
    DecodeApp { id: "effectcraft", product_id: "decode-effects", name: "Decode Effects", upstream_name: "EffectCraft", tagline: "Motion graphics e efeitos visuais", category: "Vídeo e motion", repo: "storytold/effectcraft", asset_stem: "effectcraft", license: "Apache-2.0", color: [224,54,143], is_decode_project: false },
    DecodeApp { id: "designcraft", product_id: "decode-design", name: "Decode Design", upstream_name: "DesignCraft", tagline: "Diagramação, layout e publicação", category: "Design e ilustração", repo: "storytold/designcraft", asset_stem: "designcraft", license: "Apache-2.0", color: [123,181,28], is_decode_project: false },
    DecodeApp { id: "figcraft", product_id: "decode-ui", name: "Decode UI", upstream_name: "FigCraft", tagline: "Design de interfaces e protótipos", category: "Design e ilustração", repo: "storytold/figcraft", asset_stem: "figcraft", license: "Apache-2.0", color: [2,218,150], is_decode_project: false },
    DecodeApp { id: "wordcraft", product_id: "decode-writer", name: "Decode Writer", upstream_name: "WordCraft", tagline: "Documentos e edição de texto", category: "Documentos e escritório", repo: "storytold/wordcraft", asset_stem: "wordcraft", license: "Apache-2.0", color: [63,95,224], is_decode_project: false },
    DecodeApp { id: "soundcraft", product_id: "decode-audio", name: "Decode Audio", upstream_name: "SoundCraft", tagline: "Gravação, edição e mixagem de áudio", category: "Áudio", repo: "storytold/soundcraft", asset_stem: "soundcraft", license: "Apache-2.0", color: [31,165,201], is_decode_project: false },
    DecodeApp { id: "deckcraft", product_id: "decode-slides", name: "Decode Slides", upstream_name: "DeckCraft", tagline: "Apresentações e conjuntos de slides", category: "Documentos e escritório", repo: "storytold/deckcraft", asset_stem: "deckcraft", license: "Apache-2.0", color: [240,122,30], is_decode_project: false },
    DecodeApp { id: "cadcraft", product_id: "decode-draft", name: "Decode Draft", upstream_name: "CADCraft", tagline: "Desenho técnico assistido por computador", category: "CAD e engenharia", repo: "storytold/cadcraft", asset_stem: "cadcraft", license: "Apache-2.0", color: [62,166,232], is_decode_project: false },
    DecodeApp { id: "gridcraft", product_id: "decode-sheets", name: "Decode Sheets", upstream_name: "GridCraft", tagline: "Planilhas e análise de dados", category: "Documentos e escritório", repo: "storytold/gridcraft", asset_stem: "gridcraft", license: "Apache-2.0", color: [31,157,85], is_decode_project: false },
    DecodeApp { id: "decodecad", product_id: "decode-cad", name: "Decode CAD", upstream_name: "Decode CAD", tagline: "CAD 2D/3D mantido pela Decode Analytics", category: "CAD e engenharia", repo: "Kaiqueaurelio/decodecad", asset_stem: "decode-cad", license: "MIT / Apache-2.0", color: [90,147,255], is_decode_project: true },
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
