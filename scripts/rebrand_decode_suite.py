#!/usr/bin/env python3
"""Apply the downstream Decode identity to local clones of the 13 open-source apps.

Only local clones are edited. Upstream GitHub repositories are never written to.
Original LICENSE, NOTICE and third-party attribution files are preserved.
"""
from __future__ import annotations

import argparse
import io
import json
import re
import struct
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any

TEXT_SUFFIXES = {
    ".rs", ".toml", ".json", ".xml", ".plist", ".html", ".htm", ".js", ".mjs",
    ".cjs", ".ts", ".tsx", ".css", ".scss", ".desktop", ".webmanifest", ".yml",
    ".yaml", ".sh", ".svg",
}
SKIP_PARTS = {
    ".git", "target", "node_modules", "dist", "build", "vendor", "third_party",
    "third-party", ".next", ".turbo", ".cache",
}
LIGHT_BLUE = (0x25, 0x57, 0xC6)
DARK_BLUE = (0x7C, 0xA8, 0xFF)
HOVER_BLUE = (0x4C, 0x85, 0xF7)
INK_BLUE = (0x1B, 0x40, 0x99)
RUST_STRING_RE = re.compile(r'"(?:\\.|[^"\\])*"', re.DOTALL)
WORD = r"(?<![A-Za-z0-9_]){old}(?![A-Za-z0-9_])"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def safe_word_replace(text: str, old: str, new: str) -> str:
    return re.sub(WORD.format(old=re.escape(old)), lambda _m: new, text)


def replace_rust_user_strings(text: str, old: str, new: str) -> str:
    def replace_literal(match: re.Match[str]) -> str:
        return safe_word_replace(match.group(0), old, new)
    return RUST_STRING_RE.sub(replace_literal, text)


def replace_bundle_id(text: str, upstream_slug: str, product_id: str) -> str:
    # Rename both product IDs and matching icon-file references. update_icons() renames the
    # corresponding files, so desktop Icon= keys and embedded asset paths remain in sync.
    pattern = r"ai\.storyteller\." + re.escape(upstream_slug) + r"(?![A-Za-z0-9_-])"
    return re.sub(pattern, "com.decodeanalytics." + product_id, text)


def parse_image_size(path: Path) -> tuple[int, int]:
    try:
        from PIL import Image
        with Image.open(path) as image:
            if image.width > 0 and image.height > 0:
                return image.width, image.height
    except Exception:
        pass
    match = re.search(r"(\d{2,4})x(\d{2,4})", str(path))
    if match:
        return int(match.group(1)), int(match.group(2))
    match = re.search(r"-(\d{2,4})\.(?:png|ico|icns)$", path.name, re.I)
    if match:
        return int(match.group(1)), int(match.group(1))
    return 256, 256


def svg_png(svg_bytes: bytes, width: int, height: int) -> bytes:
    try:
        import cairosvg
    except ImportError as exc:
        raise RuntimeError("Install branding dependencies: python3 -m pip install cairosvg Pillow") from exc
    return cairosvg.svg2png(bytestring=svg_bytes, output_width=width, output_height=height)


def write_ico(path: Path, svg_bytes: bytes) -> None:
    try:
        from PIL import Image
    except ImportError as exc:
        raise RuntimeError("Install Pillow: python3 -m pip install Pillow") from exc
    with Image.open(io.BytesIO(svg_png(svg_bytes, 256, 256))) as image:
        out = io.BytesIO()
        image.convert("RGBA").save(
            out, format="ICO",
            sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)],
        )
        path.write_bytes(out.getvalue())


def write_icns(path: Path, svg_bytes: bytes) -> None:
    # ICNS is a container of PNG chunks; this works when the sources are prepared on Linux.
    entries = [
        ("icp4", 16), ("icp5", 32), ("icp6", 64), ("ic07", 128),
        ("ic08", 256), ("ic09", 512), ("ic10", 1024),
    ]
    chunks: list[bytes] = []
    for code, size in entries:
        payload = svg_png(svg_bytes, size, size)
        chunks.append(code.encode("ascii") + struct.pack(">I", len(payload) + 8) + payload)
    body = b"".join(chunks)
    path.write_bytes(b"icns" + struct.pack(">I", len(body) + 8) + body)


def replace_theme_colour_calls(line: str) -> str:
    rgb_re = re.compile(r"Color32::from_rgb\([^)]*\)")
    hex_re = re.compile(r"\bhex\(0x[0-9A-Fa-f]{6}\)")
    matches_rgb = list(rgb_re.finditer(line))
    matches_hex = list(hex_re.finditer(line))
    if "pick(" in line and len(matches_rgb) >= 2:
        for match, color in reversed(list(zip(matches_rgb[:2], [LIGHT_BLUE, DARK_BLUE]))):
            value = "Color32::from_rgb(" + ", ".join("0x%02X" % x for x in color) + ")"
            line = line[:match.start()] + value + line[match.end():]
        return line
    if "pick(" in line and len(matches_hex) >= 2:
        for match, color in reversed(list(zip(matches_hex[:2], [LIGHT_BLUE, DARK_BLUE]))):
            value = "hex(0x%02X%02X%02X)" % color
            line = line[:match.start()] + value + line[match.end():]
        return line
    if matches_rgb:
        color = HOVER_BLUE if ("hover" in line.lower() or "focus" in line.lower()) else LIGHT_BLUE
        match = matches_rgb[0]
        value = "Color32::from_rgb(" + ", ".join("0x%02X" % x for x in color) + ")"
        return line[:match.start()] + value + line[match.end():]
    if matches_hex:
        color = HOVER_BLUE if ("hover" in line.lower() or "focus" in line.lower()) else LIGHT_BLUE
        match = matches_hex[0]
        value = "hex(0x%02X%02X%02X)" % color
        return line[:match.start()] + value + line[match.end():]
    return line


def rebrand_theme_file(path: Path) -> bool:
    if not path.is_file():
        return False
    original = path.read_text(encoding="utf-8", errors="replace")
    lines = original.splitlines(keepends=True)
    updated: list[str] = []
    high_contrast = False
    changed = False
    for line in lines:
        if "HighContrast" in line or "High Contrast" in line:
            high_contrast = True
        elif re.search(r"\b(?:Dark|Light|MediumDark|MediumLight|Darker|Brightness::\w+)\s*=>", line):
            high_contrast = False
        candidate = line
        if re.search(r"\b(?:APP_COLOR|APP_INK|BRAND|ACCENT)\s*:\s*Color32::from_rgb", candidate):
            match = re.search(r"Color32::from_rgb\([^)]*\)", candidate)
            if match:
                color = INK_BLUE if "APP_INK" in candidate else LIGHT_BLUE
                value = "Color32::from_rgb(" + ", ".join("0x%02X" % x for x in color) + ")"
                candidate = candidate[:match.start()] + value + candidate[match.end():]
        elif not high_contrast and re.search(
            r"\b(?:accent|accent_hover|accent_dark|accent_strong|focus|icon_accent|selection|selected_row|row_selected)\s*:",
            candidate,
        ):
            candidate = replace_theme_colour_calls(candidate)
        elif not high_contrast and re.search(r"\b(?:APP_COLOR|APP_INK|BRAND|ACCENT)\s*=", candidate) and "from_rgb" in candidate:
            candidate = replace_theme_colour_calls(candidate)
        if candidate != line:
            changed = True
        updated.append(candidate)
    if changed:
        path.write_text("".join(updated), encoding="utf-8")
    return changed


def update_toml_key(text: str, section: str, key: str, value: str) -> str:
    lines = text.splitlines()
    header = "[" + section + "]"
    start = next((i for i, line in enumerate(lines) if line.strip() == header), None)
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    assignment = key + ' = "' + escaped + '"'
    if start is None:
        lines.extend(["", header, assignment])
        return "\n".join(lines) + "\n"
    end = len(lines)
    for i in range(start + 1, len(lines)):
        if lines[i].lstrip().startswith("["):
            end = i
            break
    pattern = re.compile(r"^\s*" + re.escape(key) + r"\s*=")
    for i in range(start + 1, end):
        if pattern.search(lines[i]):
            lines[i] = assignment
            return "\n".join(lines) + "\n"
    lines.insert(start + 1, assignment)
    return "\n".join(lines) + "\n"


def add_downstream_metadata(app: dict[str, Any], repo: Path) -> int:
    changed = 0
    cargo = repo / "apps" / app["upstream_slug"] / "Cargo.toml"
    if cargo.is_file():
        text = cargo.read_text(encoding="utf-8")
        text = update_toml_key(text, "package.metadata.bundle", "name", app["display_name"])
        text = update_toml_key(text, "package.metadata.bundle", "identifier", "com.decodeanalytics." + app["product_id"])
        text = update_toml_key(text, "package.metadata.decode", "display-name", app["display_name"])
        text = update_toml_key(text, "package.metadata.decode", "upstream-repository", "storytold/" + app["upstream_slug"])
        cargo.write_text(text, encoding="utf-8")
        changed += 1
    root_cargo = repo / "Cargo.toml"
    if root_cargo.is_file():
        text = root_cargo.read_text(encoding="utf-8")
        text = update_toml_key(text, "workspace.metadata.decode", "display-name", app["display_name"])
        text = update_toml_key(text, "workspace.metadata.decode", "upstream-repository", "storytold/" + app["upstream_slug"])
        root_cargo.write_text(text, encoding="utf-8")
        changed += 1
    return changed


def visible_files(repo: Path, slug: str) -> list[Path]:
    roots = [
        repo / "apps" / slug,
        repo / ("apps/" + slug + "-web"),
        repo / "crates",
        repo / "packaging",
    ]
    candidates: set[Path] = set()
    for root in roots:
        if not root.exists():
            continue
        for path in root.rglob("*"):
            if not path.is_file() or path.suffix.lower() not in TEXT_SUFFIXES:
                continue
            parts = path.relative_to(repo).parts
            if any(part in SKIP_PARTS for part in parts):
                continue
            if any(part.lower() in {"tests", "fixtures", "goldens", "testdata", "test-data"} for part in parts):
                continue
            if "assets" in parts and "icons" in parts:
                continue
            if path.name in {"LICENSE", "LICENSE-APACHE", "LICENSE-MIT", "NOTICE"} or path.name.endswith(".attribution"):
                continue
            try:
                if path.stat().st_size > 2_000_000:
                    continue
            except OSError:
                continue
            candidates.add(path)
    for name in ["Cargo.toml", "README.md"]:
        path = repo / name
        if path.is_file():
            candidates.add(path)
    return sorted(candidates)


def patch_text_files(app: dict[str, Any], repo: Path) -> int:
    old, new = app["upstream_name"], app["display_name"]
    slug, product_id = app["upstream_slug"], app["product_id"]
    changed = 0
    for path in visible_files(repo, slug):
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        before = text
        if path.suffix.lower() == ".rs":
            text = replace_rust_user_strings(text, old, new)
        elif path.suffix.lower() == ".md":
            if path == repo / "README.md":
                lines = text.splitlines()
                if lines and lines[0].startswith("# "):
                    lines[0] = "# " + new
                    text = "\n".join(lines) + "\n"
        else:
            text = safe_word_replace(text, old, new)
        text = replace_bundle_id(text, slug, product_id)
        if text != before:
            path.write_text(text, encoding="utf-8")
            changed += 1
    changed += add_downstream_metadata(app, repo)
    return changed


def update_icons(app: dict[str, Any], repo: Path, icon_path: Path) -> int:
    svg_bytes = icon_path.read_bytes()
    try:
        ET.fromstring(svg_bytes)
    except ET.ParseError as exc:
        raise RuntimeError("Invalid Decode SVG icon: " + str(icon_path)) from exc
    count = 0
    roots = [repo / "assets" / "app-icon", repo / "apps" / (app["upstream_slug"] + "-web") / "web"]
    old_stem = "ai.storyteller." + app["upstream_slug"]
    new_stem = "com.decodeanalytics." + app["product_id"]
    # Rename the resource files and attribution sidecars as a single downstream operation.
    for root in roots:
        if not root.exists():
            continue
        for path in sorted(root.rglob("*"), key=lambda item: len(item.parts), reverse=True):
            if not path.is_file() or old_stem not in path.name:
                continue
            target = path.with_name(path.name.replace(old_stem, new_stem))
            if target.exists() and target != path:
                raise RuntimeError("Refusing to overwrite duplicate branded asset: " + str(target))
            if target != path:
                path.rename(target)
    for root in roots:
        if not root.exists():
            continue
        for path in root.rglob("*"):
            if not path.is_file() or path.suffix.lower() not in {".svg", ".png", ".ico", ".icns"}:
                continue
            if any(part in SKIP_PARTS for part in path.parts):
                continue
            if "-document." in path.name.lower():
                continue
            suffix = path.suffix.lower()
            if suffix == ".svg":
                path.write_bytes(svg_bytes)
            elif suffix == ".png":
                width, height = parse_image_size(path)
                path.write_bytes(svg_png(svg_bytes, width, height))
            elif suffix == ".ico":
                write_ico(path, svg_bytes)
            elif suffix == ".icns":
                write_icns(path, svg_bytes)
            count += 1
    # Preserve original attribution records and clarify that the displayed brand asset was
    # replaced downstream. Never delete or rewrite the existing third-party attribution.
    attribution_note = (
        "\n\nDecode Analytics downstream rebrand note: the visual app icon in this "
        "downstream build was replaced by an original Decode Studio asset. The original "
        "attribution above is retained for the upstream source/history and any third-party "
        "materials that remain in the project. See DECODE_BRANDING.md."
    )
    attribution_root = repo / "assets" / "app-icon"
    sidecars = attribution_root.rglob("*.attribution") if attribution_root.exists() else []
    for sidecar in sidecars:
        try:
            old_text = sidecar.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if "Decode Analytics downstream rebrand note:" not in old_text:
            sidecar.write_text(old_text.rstrip() + attribution_note + "\n", encoding="utf-8")
    return count


def ensure_safe_git_branch(repo: Path) -> None:
    if not (repo / ".git").exists():
        return
    current = subprocess.run(["git", "-C", str(repo), "branch", "--show-current"], text=True, capture_output=True)
    if current.returncode == 0 and current.stdout.strip() != "decode-branding":
        subprocess.run(["git", "-C", str(repo), "switch", "-c", "decode-branding"], check=True, capture_output=True)
    remotes = subprocess.run(["git", "-C", str(repo), "remote"], text=True, capture_output=True)
    remote_names = remotes.stdout.split() if remotes.returncode == 0 else []
    if "origin" in remote_names and "upstream" not in remote_names:
        subprocess.run(["git", "-C", str(repo), "remote", "rename", "origin", "upstream"], check=True, capture_output=True)


def add_brand_note(app: dict[str, Any], repo: Path) -> None:
    note = (
        "# Downstream branding — " + app["display_name"] + "\n\n"
        "This is a Decode Analytics downstream rebrand of the public project "
        "[storytold/" + app["upstream_slug"] + "](https://github.com/storytold/" + app["upstream_slug"] + ").\n\n"
        "The Decode display name, icon, application metadata and accent colors are downstream changes. "
        "Original notices, third-party notices, file attributions and LICENSE-APACHE / LICENSE-MIT files "
        "remain intact. This build is not an official upstream release.\n\n"
        "Product ID: com.decodeanalytics." + app["product_id"] + ".\n"
    )
    (repo / "DECODE_BRANDING.md").write_text(note, encoding="utf-8")


def apply_one(app: dict[str, Any], workspace: Path, icons_dir: Path) -> dict[str, Any]:
    repo = workspace / app["product_id"]
    if not repo.is_dir():
        return {"product": app["display_name"], "status": "missing source", "path": str(repo)}
    ensure_safe_git_branch(repo)
    licenses_before = {name: (repo / name).exists() for name in ["LICENSE-APACHE", "LICENSE-MIT", "NOTICE"]}
    theme_changed = rebrand_theme_file(repo / "crates" / "ui-egui" / "src" / "theme.rs")
    text_changed = patch_text_files(app, repo)
    icons_changed = update_icons(app, repo, icons_dir / app["icon"])
    add_brand_note(app, repo)
    licenses_after = {name: (repo / name).exists() for name in licenses_before}
    if licenses_before != licenses_after or not all(licenses_after.values()):
        raise RuntimeError("License/notice files were not preserved for " + app["upstream_slug"])
    return {
        "product": app["display_name"], "status": "rebranded", "path": str(repo),
        "text_files_changed": text_changed, "theme_changed": theme_changed,
        "icon_assets_replaced": icons_changed, "license_files_preserved": True,
    }


def self_test(manifest_path: Path, icons_dir: Path) -> int:
    manifest = load_json(manifest_path)
    products = manifest.get("products", [])
    assert len(products) == 13, "The manifest must include exactly 13 downstream applications."
    ids = [p["product_id"] for p in products]
    assert len(set(ids)) == 13, "Decode product IDs must be unique."
    assert all(p["display_name"].startswith("Decode ") for p in products), "Every public app name must start with Decode."
    for app in products:
        icon = icons_dir / app["icon"]
        assert icon.is_file(), "Missing icon: " + str(icon)
        ET.fromstring(icon.read_bytes())
        sample = 'let title = "' + app["upstream_name"] + '"; let variable = "' + app["upstream_name"] + 'Engine";'
        changed = replace_rust_user_strings(sample, app["upstream_name"], app["display_name"])
        assert '"' + app["display_name"] + '"' in changed
        assert '"' + app["upstream_name"] + 'Engine"' in changed, "Internal identifiers must not be rewritten."
        sample_id = 'const ID = "ai.storyteller.' + app["upstream_slug"] + '";'
        assert "com.decodeanalytics." + app["product_id"] in replace_bundle_id(sample_id, app["upstream_slug"], app["product_id"])
        sample_icon_path = "assets/app-icon/ai.storyteller." + app["upstream_slug"] + ".png"
        assert "assets/app-icon/com.decodeanalytics." + app["product_id"] + ".png" in replace_bundle_id(sample_icon_path, app["upstream_slug"], app["product_id"])
    theme_sample = "accent: pick(dark, Color32::from_rgb(0x10, 0x20, 0x30), Color32::from_rgb(0x40, 0x50, 0x60)),\n"
    patched = replace_theme_colour_calls(theme_sample)
    assert "0x25, 0x57, 0xC6" in patched and "0x7C, 0xA8, 0xFF" in patched
    print("Decode rebrand self-test: OK — 13 products, 13 valid icons, safe names, IDs and theme tokens.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", type=Path, help="Directory containing clones named by Decode product ID.")
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--icons", type=Path)
    parser.add_argument("--only", help="Only rebrand one product_id from the manifest.")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    manifest_path = (args.manifest or root / "open-source-hub" / "decode-suite-manifest.json").resolve()
    icons_dir = (args.icons or root / "open-source-hub" / "assets" / "icons").resolve()
    if args.self_test:
        return self_test(manifest_path, icons_dir)
    if not args.workspace:
        parser.error("--workspace is required unless --self-test is used.")
    workspace = args.workspace.resolve()
    if not workspace.is_dir():
        parser.error("Workspace directory does not exist: " + str(workspace))
    manifest = load_json(manifest_path)
    products = manifest["products"]
    if args.only:
        products = [app for app in products if app["product_id"] == args.only]
        if not products:
            parser.error("Unknown product_id: " + args.only)
    results = [apply_one(app, workspace, icons_dir) for app in products]
    print(json.dumps({"workspace": str(workspace), "results": results}, ensure_ascii=False, indent=2))
    if any(result["status"] != "rebranded" for result in results):
        print("Some downstream source clones were missing. See the results above.", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
