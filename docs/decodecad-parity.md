# DecodeCAD functional parity with CADCraft

**Audit date:** 2026-10-24  
**DecodeCAD revision audited:** `fb3781f84e73ebb517681d35f2cf64ff9f4eb5a4`  
**CADCraft upstream revision:** `b90165666917ec3ac6c308c2ea1c55bb6ea8317d`

## Result

The DecodeCAD application code was compared with the upstream CADCraft main branch. The fork was three upstream commits behind at the start of this audit. Those changes have been ported to the distribution pull request:

| Upstream change | DecodeCAD implementation | Evidence |
|---|---|---|
| Preserve ORTHO behavior for opposite-corner input in STRETCH and related commands | Ported | `crates/engine/src/cmd/{draw2,modify,plotters}.rs`, `crates/engine/src/cmd/layout/viewports.rs`, `crates/engine/src/cmd/view/named_views.rs`, `crates/engine/src/select.rs` |
| Regression coverage for STRETCH window behavior | Ported | `crates/engine/tests/stretch_window_ortho.rs` |
| Use craft-ui shared docking panels and updated docking workflows | Ported | `apps/cadcraft/src/workspace.rs`, `crates/ui-egui/src/docking.rs`, `crates/ui-egui/tests/it/docking_workflows.rs` |

The source comparison after these ports leaves only branding/metadata and language-default changes in the compared application code. DecodeCAD keeps the CADCraft engine, command implementation, file handling, and editing paths. The UI'\''s initial language preference is English in DecodeCAD; upstream defaults to automatic locale selection. Users can still choose a supported language in settings.

## Deliberate product differences

- Product name, window title, icons, and Windows metadata identify **Decode CAD / Decode Analytics**.
- Cargo repository metadata points at the DecodeCAD repository.
- README and packaging provide DecodeCAD-branded Windows and Linux distribution entry points.
- The upstream licensing and attribution notices are retained. See `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, and `ATTRIBUTION.md`.

These are product/distribution changes and are not intended to remove CAD functionality.

## Remaining functional limitations

DecodeCAD inherits the limitations that exist in CADCraft itself. The project’s detailed evidence-backed limitation inventory is maintained in [gaps.md](gaps.md), with deeper references for [file format parity](file-format-parity.md), [geometry parity](geometry-parity.md), and [target application parity](target-app-parity.md). These documents include known gaps such as incomplete preservation of some DWG custom data, the lack of a real-file corpus validated in AutoCAD, tessellated ellipse/spline operations, and incomplete dynamic-block/xref editing.

Therefore this audit establishes source-level synchronization with the inspected CADCraft revision for the application code, plus the specific DecodeCAD product differences above. It does **not** establish equivalence with AutoCAD or claim that every behavior has been validated against real customer drawings.

## Verification

The upstream regression and docking tests were included in the port and the final pull request revision passed its checks before merge.

| Verification | Result | Evidence |
|---|---|---|
| `cargo xtask ci` quality gate | Passed | [GitHub Actions run 40150585788](https://github.com/Kaiqueaurelio/decodecad/actions/runs/40150585788) |
| Desktop release build (Linux, Windows, macOS) | Passed | [GitHub Actions run 40150585401](https://github.com/Kaiqueaurelio/decodecad/actions/runs/40150585401) |
| Windows MSI and portable ZIP; Linux DEB, AppImage and tar | Passed | [GitHub Actions run 40150585410](https://github.com/Kaiqueaurelio/decodecad/actions/runs/40150585410) |
| Nix evaluation, packaging lint, SonarCloud | Passed | [Nix](https://github.com/Kaiqueaurelio/decodecad/actions/runs/40150585754), [packaging lint](https://github.com/Kaiqueaurelio/decodecad/actions/runs/40150585664), [SonarCloud](https://github.com/Kaiqueaurelio/decodecad/actions/runs/40150585781) |

The CI installer artifacts are temporary GitHub Actions downloads. A public customer download requires a GitHub Release. These checks establish successful builds and project tests for this revision; they do not replace testing against real customer drawings or AutoCAD.

