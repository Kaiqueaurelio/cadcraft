# Decode Suite — Brand System

This document is the single source of truth for the visual identity applied to the Decode Open Source Hub and launcher.

## Brand direction

- Product family: **Decode Suite**
- Publisher: **Decode Analytics**
- Tone: precise, capable, modern, practical, developer-friendly.
- Core colors: midnight navy `#0B1220`, deep blue `#172B4D`, Decode blue `#2563EB`, bright blue `#60A5FA`, ice white `#EAF2FF`.
- UI surfaces: navy backgrounds, restrained blue borders, clear typography, accessible contrast, consistent spacing and rounded geometry.
- Brand mark: a geometric “D” built from a strong blue shape on a midnight surface. Each product icon keeps the same core mark and changes only its short product label.
- Naming rule: user-facing names use **Decode + product function**. “Craft” is not used as the Decode product family name.

## Product names

| Upstream project | Decode display name | Icon |
|---|---|---|
| PhotoCraft | Decode Photo | `assets/decode-icons/decode-photo.svg` |
| VectorCraft | Decode Vector | `assets/decode-icons/decode-vector.svg` |
| FilmCraft | Decode Film | `assets/decode-icons/decode-film.svg` |
| LightCraft | Decode Light | `assets/decode-icons/decode-light.svg` |
| PdfCraft | Decode PDF | `assets/decode-icons/decode-pdf.svg` |
| EffectCraft | Decode Effects | `assets/decode-icons/decode-effects.svg` |
| DesignCraft | Decode Design | `assets/decode-icons/decode-design.svg` |
| FigCraft | Decode UI | `assets/decode-icons/decode-ui.svg` |
| WordCraft | Decode Writer | `assets/decode-icons/decode-writer.svg` |
| SoundCraft | Decode Audio | `assets/decode-icons/decode-audio.svg` |
| DeckCraft | Decode Slides | `assets/decode-icons/decode-slides.svg` |
| CADCraft | Decode Draft | `assets/decode-icons/decode-draft.svg` |
| GridCraft | Decode Sheets | `assets/decode-icons/decode-sheets.svg` |
| Decode CAD | Decode CAD | `assets/decode-icons/decode-cad.svg` |

## Important migration boundary

The names and icons in the Decode catalogue are rebranded. This alone does **not** mean the upstream applications themselves have been forked, rebuilt, or rebranded. Until Decode-branded source forks and release artifacts exist, links to source and releases still point to upstream projects, and an installed upstream binary may retain its original name, icon, bundle identifier, and UI.

Do not label an upstream binary as a finished Decode build. The next migration stage is to create a legally compliant fork for each application, apply this design system inside each app, update package metadata and identifiers, add attribution/license notices, build and test it on supported platforms, then point the launcher to the Decode-owned releases.

## Release quality gate

For each app, require:
1. Source fork with preserved upstream license and attribution.
2. App name, icons, splash/about screen, menus, theme tokens, package metadata and identifiers updated.
3. No old product marks in the visible UI except required legal attribution.
4. Existing features retained; regression tests added for launch, open/save/export and update/install.
5. Successful CI on every claimed platform and a signed/hash-verified release artifact.
6. A migration note describing upstream changes and Decode changes.

Do not claim an app is fully rebranded until all six gates pass.
