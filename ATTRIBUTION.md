# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example drawings, presets) is listed
here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if
an asset file is missing from this table.

**Policy (mandatory):** CADCraft contains **no Autodesk, Adobe or Avid iconography, images,
artwork, fonts, hatch patterns, linetypes, templates or presets.** Every asset is original work by
CADCraft contributors or third-party material under an open licence (OSI open source, public domain
/ CC0, or Creative Commons that allows redistribution). Screenshots of Autodesk software are never
committed. Font files are not added here: shared fonts live in
[storytold/craft-fonts](https://github.com/storytold/craft-fonts), an optional build input. The one
exception is the ArtCraft brand in `docs/brand/`: ArtCraft trademarks, not open source, used under
`docs/brand/LICENSE-brand.txt`.

Generated-in-code assets are original and have no file to list:
- the UI icon set (`crates/ui-egui/src/icons.rs`);
- the "Decode CAD Stroke" single-stroke drafting font (`crates/fonts/src/stroke.rs`);
- the standard linetype and hatch-pattern libraries (`crates/doc/src/library.rs`; industry-common
  names, our own dash/spacing values);
- the colour index palette, generated from its structure (`crates/color/src/lib.rs`);
- the sample drawings (`crates/engine/src/sample.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/decode-cad.svg` | Decode Analytics | Original vector mark created for the Decode CAD rebrand (2026-10-25) | MIT OR Apache-2.0 | Used by the About dialog, web loading screen, Linux packages and generated platform icons |
| `packaging/macos/dmg/background.svg` | Decode Analytics | Original Decode CAD installer background created for the rebrand (2026-10-25) | MIT OR Apache-2.0 | Source artwork rendered by `packaging/macos/dmg/generate.py` for the macOS installer |
| `examples/apartment.dxf` | CADCraft contributors | sample drawing made with cadcraft-cli commands | MIT OR Apache-2.0 | Original |
| `docs/brand/artcraft-logo-white.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-logo-white.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-logo.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-logo.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark-black.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark-black.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |

## Interface translations

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `crates/ui-egui/src/i18n/uk.tsv` | @dmatviichuk | Original Ukrainian translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/de.tsv` | the CADCraft contributors | Original German translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/es.tsv` | @PedroGuille | Original Spanish translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/pt_br.tsv` | @WladmirJunior | Original Brazilian Portuguese translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/cs.tsv` | @Losminek (AI-assisted) | Original Czech translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/ru.tsv` | @pechnikov2026-star | Original Russian translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/fr.tsv` | @pechnikov2026-star | Original French translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/zh.tsv` | @mayloglog | Original Simplified Chinese translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/sv.tsv` | Erik Holmbom (@holmbom) with Claude Code (AI-assisted) | Original Swedish translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; terminology and the running app reviewed by a native speaker |
| `crates/ui-egui/src/i18n/it.tsv` | Emanuele Bottelli (AI-assisted) | Original Italian translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/ko.tsv` | @huk082 (AI-assisted) | Original Korean translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; no font assets added |
| `crates/ui-egui/src/i18n/ar.tsv` | @hasan-sdaif | Original Arabic translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/hu.tsv` | @Skaywalker | Original Hungarian translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/tr.tsv` | @ogzhnklvz (AI-assisted) | Original Turkish translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; written from the English strings, no other catalog used as a source; the running app reviewed by a native speaker |
| `crates/ui-egui/src/i18n/el.tsv` | @Camthalion77 with ChatGPT (AI-assisted) | Original Greek translations of CADCraft English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; independent native proofreading pending |
