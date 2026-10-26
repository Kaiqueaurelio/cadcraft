#[derive(Clone, Copy, Debug)]
pub struct CraftApp {
    pub id: &'static str,
    pub name: &'static str,
    pub tagline: &'static str,
    pub category: &'static str,
    pub repo: &'static str,
    pub asset_stem: &'static str,
    pub license: &'static str,
    pub color: [u8; 3],
    pub is_decode_project: bool,
}

impl CraftApp {
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
        format!("ai.storyteller.{}", self.id)
    }

    pub fn icon_url(&self) -> String {
        if self.is_decode_project {
            "https://raw.githubusercontent.com/Kaiqueaurelio/decodecad/main/assets/app-icon/hicolor/256x256/apps/ai.storyteller.cadcraft.png".to_string()
        } else {
            format!("https://raw.githubusercontent.com/{}/main/assets/app-icon/hicolor/128x128/apps/ai.storyteller.{}.png", self.repo, self.id)
        }
    }
}

/// The 13 public Craft App Suite projects plus Decode CAD, listed separately as a Decode Analytics project.
pub const APPS: &[CraftApp] = &[
    CraftApp { id: "photocraft", name: "PhotoCraft", tagline: "Camadas, máscaras e edição de imagens", category: "Imagem e fotografia", repo: "storytold/photocraft", asset_stem: "photocraft", license: "Apache-2.0", color: [47,123,245], is_decode_project: false },
    CraftApp { id: "vectorcraft", name: "VectorCraft", tagline: "Ilustração vetorial e desenho escalável", category: "Design e ilustração", repo: "storytold/vectorcraft", asset_stem: "vectorcraft", license: "Apache-2.0", color: [232,87,63], is_decode_project: false },
    CraftApp { id: "filmcraft", name: "FilmCraft", tagline: "Edição de vídeo, cor e áudio", category: "Vídeo e motion", repo: "storytold/filmcraft", asset_stem: "filmcraft", license: "Apache-2.0", color: [139,92,246], is_decode_project: false },
    CraftApp { id: "lightcraft", name: "LightCraft", tagline: "Fotografia RAW e edição não destrutiva", category: "Imagem e fotografia", repo: "storytold/lightcraft", asset_stem: "lightcraft", license: "Apache-2.0", color: [242,165,22], is_decode_project: false },
    CraftApp { id: "pdfcraft", name: "PdfCraft", tagline: "Leitura, organização e proteção de PDFs", category: "Documentos e escritório", repo: "storytold/pdfcraft", asset_stem: "pdfcraft", license: "Apache-2.0", color: [18,165,138], is_decode_project: false },
    CraftApp { id: "effectcraft", name: "EffectCraft", tagline: "Motion graphics e efeitos visuais", category: "Vídeo e motion", repo: "storytold/effectcraft", asset_stem: "effectcraft", license: "Apache-2.0", color: [224,54,143], is_decode_project: false },
    CraftApp { id: "designcraft", name: "DesignCraft", tagline: "Diagramação, layout e publicação", category: "Design e ilustração", repo: "storytold/designcraft", asset_stem: "designcraft", license: "Apache-2.0", color: [123,181,28], is_decode_project: false },
    CraftApp { id: "figcraft", name: "FigCraft", tagline: "Design de interfaces e protótipos", category: "Design e ilustração", repo: "storytold/figcraft", asset_stem: "figcraft", license: "Apache-2.0", color: [2,218,150], is_decode_project: false },
    CraftApp { id: "wordcraft", name: "WordCraft", tagline: "Documentos e edição de texto", category: "Documentos e escritório", repo: "storytold/wordcraft", asset_stem: "wordcraft", license: "Apache-2.0", color: [63,95,224], is_decode_project: false },
    CraftApp { id: "soundcraft", name: "SoundCraft", tagline: "Gravação, edição e mixagem de áudio", category: "Áudio", repo: "storytold/soundcraft", asset_stem: "soundcraft", license: "Apache-2.0", color: [31,165,201], is_decode_project: false },
    CraftApp { id: "deckcraft", name: "DeckCraft", tagline: "Apresentações e conjuntos de slides", category: "Documentos e escritório", repo: "storytold/deckcraft", asset_stem: "deckcraft", license: "Apache-2.0", color: [240,122,30], is_decode_project: false },
    CraftApp { id: "cadcraft", name: "CADCraft", tagline: "Desenho técnico assistido por computador", category: "CAD e engenharia", repo: "storytold/cadcraft", asset_stem: "cadcraft", license: "Apache-2.0", color: [62,166,232], is_decode_project: false },
    CraftApp { id: "gridcraft", name: "GridCraft", tagline: "Planilhas e análise de dados", category: "Documentos e escritório", repo: "storytold/gridcraft", asset_stem: "gridcraft", license: "Apache-2.0", color: [31,157,85], is_decode_project: false },
    CraftApp { id: "decodecad", name: "Decode CAD", tagline: "CAD 2D/3D mantido pela Decode Analytics", category: "CAD e engenharia", repo: "Kaiqueaurelio/decodecad", asset_stem: "decode-cad", license: "MIT / Apache-2.0", color: [90,147,255], is_decode_project: true },
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
    fn catalog_contains_thirteen_craft_apps_and_decode_project() {
        assert_eq!(APPS.len(), 14);
        assert_eq!(APPS.iter().filter(|a| !a.is_decode_project).count(), 13);
        assert_eq!(APPS.iter().filter(|a| a.is_decode_project).count(), 1);
    }

    #[test]
    fn catalog_ids_are_unique() {
        for (i, app) in APPS.iter().enumerate() {
            assert!(!APPS[..i].iter().any(|other| other.id == app.id));
        }
    }
}
