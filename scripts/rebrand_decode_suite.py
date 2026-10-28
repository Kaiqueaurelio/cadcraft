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
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any

TEXT_SUFFIXES = {
    ".rs", ".toml", ".json", ".xml", ".plist", ".html", ".htm", ".js", ".mjs",
    ".cjs", ".ts", ".tsx", ".css", ".scss", ".desktop", ".webmanifest", ".yml",
    ".yaml", ".sh", ".svg", ".in", ".ps1", ".md",
}
SKIP_PARTS = {
    ".git", "target", "node_modules", "dist", "build", "vendor", "third_party",
    "third-party", ".next", ".turbo", ".cache",
}
# These pages preserve upstream ownership / credit language and should not be relabeled as Decode.
PRESERVE_ATTRIBUTION_DOCS = {
    "docs/project/credits.md",
    "docs/project/licenses.md",
    "book/src/project/credits.md",
    "book/src/project/licenses.md",
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
        rewritten = safe_word_replace(match.group(0), old, new)
        # Keep upstream patch provenance labels inside regression tests. These labels are
        # part of the vendored source contract (for example "PdfCraft patch:" in winit),
        # not a user-facing application name; rewriting them breaks tests that verify patches.
        rewritten = re.sub(re.escape(new) + r"(?=\s+patch:)", old, rewritten)
        return rewritten
    return RUST_STRING_RE.sub(replace_literal, text)


def replace_bundle_id(text: str, upstream_slug: str, product_id: str) -> str:
    # Rename both product IDs and matching icon-file references. update_icons() renames the
    # corresponding files, so desktop Icon= keys and embedded asset paths remain in sync.
    # Product-specific suffixes such as "-cli" are valid bundle IDs and must be rebranded too.
    pattern = r"ai\.storyteller\." + re.escape(upstream_slug) + r"(?![A-Za-z0-9_])"
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


def _rgb_tuple_from_expression(expr: str) -> tuple[int, int, int] | None:
    hex_match = re.search(r"(?:hex|rgb)\(\s*0x([0-9A-Fa-f]{6})\s*\)", expr)
    if hex_match:
        value = int(hex_match.group(1), 16)
        return ((value >> 16) & 255, (value >> 8) & 255, value & 255)
    rgb_match = re.search(r"Color32::from_rgb\(\s*(0x[0-9A-Fa-f]+|\d+)\s*,\s*(0x[0-9A-Fa-f]+|\d+)\s*,\s*(0x[0-9A-Fa-f]+|\d+)", expr)
    if rgb_match:
        values = []
        for item in rgb_match.groups():
            values.append(int(item, 16) if item.lower().startswith("0x") else int(item))
        return tuple(max(0, min(255, value)) for value in values)  # type: ignore[return-value]
    rgba_match = re.search(r"Color32::from_rgba_(?:un)?premultiplied\(\s*(0x[0-9A-Fa-f]+|\d+)\s*,\s*(0x[0-9A-Fa-f]+|\d+)\s*,\s*(0x[0-9A-Fa-f]+|\d+)", expr)
    if rgba_match:
        values = []
        for item in rgba_match.groups():
            values.append(int(item, 16) if item.lower().startswith("0x") else int(item))
        return tuple(max(0, min(255, value)) for value in values)  # type: ignore[return-value]
    return None


def _replace_color_constructor(expr: str, color: tuple[int, int, int], field: str) -> str:
    rgb = "0x%02X, 0x%02X, 0x%02X" % color
    rgba_pattern = re.compile(r"Color32::from_rgba_(un)?premultiplied\(\s*([^)]*)\)")
    rgba_match = rgba_pattern.search(expr)
    if rgba_match:
        unmultiplied = rgba_match.group(1) is not None
        args = [part.strip() for part in rgba_match.group(2).split(",")]
        if len(args) >= 4:
            alpha = args[3]
            if not unmultiplied:
                try:
                    alpha_value = int(alpha, 0)
                    premultiplied = tuple(round(channel * alpha_value / 255) for channel in color)
                    rgb_value = ", ".join(str(channel) for channel in premultiplied)
                except ValueError:
                    rgb_value = ", ".join(str(channel) for channel in color)
            else:
                rgb_value = rgb
            replacement = ("Color32::from_rgba_unmultiplied(" if unmultiplied else "Color32::from_rgba_premultiplied(") + rgb_value + ", " + alpha + ")"
            return expr[:rgba_match.start()] + replacement + expr[rgba_match.end():]

    constructor = re.compile(r"Color32::from_rgb\([^)]*\)|\bhex\(0x[0-9A-Fa-f]{6}\)|\brgb\(0x[0-9A-Fa-f]{6}\)")
    match = constructor.search(expr)
    if not match:
        return expr
    original = match.group(0)
    if original.startswith("Color32::from_rgb"):
        replacement = "Color32::from_rgb(" + rgb + ")"
    elif original.startswith("hex("):
        replacement = "hex(0x%02X%02X%02X)" % color
    else:
        replacement = "rgb(0x%02X%02X%02X)" % color
    return expr[:match.start()] + replacement + expr[match.end():]


def replace_theme_colour_calls(line: str) -> str:
    """Replace a single theme-token line while preserving its Rust expression shape."""
    match = re.search(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.*?)(,\s*(?://.*)?\r?\n?$)", line)
    if not match:
        return line
    field = match.group(1)
    rhs = match.group(2)
    lowered = field.lower()
    if lowered not in {
        "accent", "accent_hover", "accent_dark", "accent_strong", "icon_accent",
        "selection", "selected_row", "row_selected", "focus", "highlight",
        "accent_soft", "accent_border", "accent_text",
    }:
        return line

    if "pick(" in rhs:
        colors: list[tuple[int, int, int]]
        if lowered == "accent_soft":
            colors = [INK_BLUE, (0xE3, 0xEC, 0xFD)]
        elif lowered == "accent_text":
            colors = [DARK_BLUE, LIGHT_BLUE]
        elif lowered in {"accent_hover", "focus"}:
            colors = [HOVER_BLUE, HOVER_BLUE]
        else:
            colors = [LIGHT_BLUE, HOVER_BLUE]
        constructor = re.compile(r"Color32::from_rgb\([^)]*\)|\bhex\(0x[0-9A-Fa-f]{6}\)|\brgb\(0x[0-9A-Fa-f]{6}\)")
        matches = list(constructor.finditer(rhs))
        if matches:
            for index in reversed(range(len(matches))):
                match_item = matches[index]
                color = colors[min(index, len(colors) - 1)]
                piece = rhs[match_item.start():match_item.end()]
                replacement = _replace_color_constructor(piece, color, lowered)
                rhs = rhs[:match_item.start()] + replacement + rhs[match_item.end():]
        # Preserve the selection alpha if the expression uses a semi-transparent constructor.
        if "from_rgba_unmultiplied" in rhs or "from_rgba_premultiplied" in rhs:
            color = HOVER_BLUE if lowered in {"accent_hover", "focus", "accent_border"} else LIGHT_BLUE
            rhs = _replace_color_constructor(rhs, color, lowered)
    else:
        existing = _rgb_tuple_from_expression(rhs)
        brightness = sum(existing) / 3 if existing else 128
        if lowered in {"accent_hover", "focus", "accent_border"}:
            color = HOVER_BLUE
        elif lowered == "accent_soft":
            if "from_rgba_unmultiplied" in rhs or "from_rgba_premultiplied" in rhs:
                color = HOVER_BLUE
            else:
                color = (0xE3, 0xEC, 0xFD) if brightness >= 170 else INK_BLUE
        elif lowered == "accent_text":
            color = LIGHT_BLUE if brightness >= 135 else (0x25, 0x57, 0xC6)
        else:
            color = LIGHT_BLUE

        if re.search(r"Color32::(?:from_rgb|from_rgba_(?:un)?premultiplied)\(", rhs) or re.search(r"\b(?:hex|rgb)\(0x[0-9A-Fa-f]{6}\)", rhs):
            rhs = _replace_color_constructor(rhs, color, lowered)

    return line[:match.start(2)] + rhs + line[match.end(2):]


def _replace_brand_constant(line: str) -> str:
    match = re.search(r"\b(APP_COLOR|APP_INK|BRAND|ACCENT|ACCENT_ON)\s*:\s*Color32\s*=", line)
    if not match:
        return line
    name = match.group(1)
    if name == "APP_INK":
        color = INK_BLUE
    elif name == "ACCENT_ON":
        color = (0x07, 0x10, 0x1E)
    elif name in {"APP_COLOR", "BRAND"}:
        color = HOVER_BLUE
    else:
        color = LIGHT_BLUE
    return _replace_color_constructor(line, color, name.lower())


def rebrand_theme_file(path: Path) -> bool:
    if not path.is_file():
        return False
    original = path.read_text(encoding="utf-8", errors="replace")
    lines = original.splitlines(keepends=True)
    updated: list[str] = []
    changed = False
    brace_depth = 0
    high_contrast_depth: int | None = None
    high_contrast_start = re.compile(r"\b(?:Brightness|ThemeKind|Theme)\s*::\s*HighContrast\s*=>|\bHighContrast\s*=>")
    normal_start = re.compile(
        r"\b(?:Brightness|ThemeKind|Theme)\s*::\s*(?:Dark|MediumDark|MediumLight|Light|Studio|StudioLight|Classic|Adwaita|AdwaitaDark|SolarizedDark|BreezeLight|BreezeDark)\s*=>"
        r"|\b(?:Dark|MediumDark|MediumLight|Light|Studio|StudioLight|Classic|Adwaita|AdwaitaDark|SolarizedDark|BreezeLight|BreezeDark)\s*=>"
    )

    for line in lines:
        code = re.sub(r'"(?:\\.|[^"\\])*"', '""', line)
        code = code.split("//", 1)[0]
        current_depth = brace_depth
        start_high = high_contrast_start.search(code) is not None
        if start_high:
            high_contrast_depth = current_depth + max(1, code.count("{") - code.count("}"))
        elif high_contrast_depth is not None and normal_start.search(code):
            high_contrast_depth = None
        in_high_contrast = start_high or (high_contrast_depth is not None and current_depth >= high_contrast_depth)
        candidate = line
        if not in_high_contrast:
            candidate = _replace_brand_constant(candidate)
            candidate = replace_theme_colour_calls(candidate)
        if candidate != line:
            changed = True
        updated.append(candidate)
        brace_depth += code.count("{") - code.count("}")
        if high_contrast_depth is not None and brace_depth < high_contrast_depth:
            high_contrast_depth = None
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
        text = update_toml_key(text, "package.metadata.decode", "upstream-repository", "Kaiqueaurelio/" + app["upstream_slug"])
        cargo.write_text(text, encoding="utf-8")
        changed += 1
    root_cargo = repo / "Cargo.toml"
    if root_cargo.is_file():
        text = root_cargo.read_text(encoding="utf-8")
        text = update_toml_key(text, "workspace.metadata.decode", "display-name", app["display_name"])
        text = update_toml_key(text, "workspace.metadata.decode", "upstream-repository", "Kaiqueaurelio/" + app["upstream_slug"])
        root_cargo.write_text(text, encoding="utf-8")
        changed += 1
    return changed


def visible_files(repo: Path, slug: str) -> list[Path]:
    roots = [
        repo / "apps" / slug,
        repo / ("apps/" + slug + "-web"),
        repo / "crates",
        repo / "packaging",
        repo / "docs",
        repo / "book",
    ]
    candidates: set[Path] = set()
    for root in roots:
        if not root.exists():
            continue
        for path in root.rglob("*"):
            is_extensionless_packaging_script = (
                path.is_file()
                and not path.suffix
                and (repo / "packaging") in path.parents
            )
            if not path.is_file() or (
                path.suffix.lower() not in TEXT_SUFFIXES and not is_extensionless_packaging_script
            ):
                continue
            parts = path.relative_to(repo).parts
            if any(part in SKIP_PARTS for part in parts):
                continue
            if path.relative_to(repo).as_posix() in PRESERVE_ATTRIBUTION_DOCS:
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


def replace_rust_comment_text(text: str, old: str, new: str) -> str:
    """Rebrand documentation comments without renaming Rust symbols or module paths."""
    updated: list[str] = []
    for line in text.splitlines(keepends=True):
        stripped = line.lstrip()
        is_comment = stripped.startswith("//") or stripped.startswith("/*") or stripped.startswith("*") or stripped.startswith("*/")
        if is_comment:
            line = safe_word_replace(line, old, new)
            line = re.sub(r"\bCraft App Suite\b", "Decode Studio Suite", line)
            line = re.sub(r"\bThe Craft look\b", "The Decode look", line)
            line = re.sub(r"\bthe Craft look\b", "the Decode look", line)
        updated.append(line)
    return "".join(updated)


def replace_markdown_package_names(text: str, upstream_slug: str, product_id: str) -> str:
    """Rebrand documented release filenames while leaving upstream repository URLs intact."""
    pattern = r"(?<![A-Za-z0-9_])" + re.escape(upstream_slug) + r"(?=-(?:<version>|[vV]?[0-9]))"
    return re.sub(pattern, product_id, text)


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
            text = replace_rust_comment_text(text, old, new)
        elif path.suffix.lower() == ".md":
            text = safe_word_replace(text, old, new)
            text = re.sub(r"\bCraft App Suite\b", "Decode Studio Suite", text)
            text = re.sub(r"\bThe Craft look\b", "The Decode look", text)
            text = re.sub(r"\bthe Craft look\b", "the Decode look", text)
            text = replace_markdown_package_names(text, slug, product_id)
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
    old_stem = "com.decode." + app["upstream_slug"]
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


def rename_bundle_id_files(app: dict[str, Any], repo: Path) -> int:
    """Rename files whose names carry the upstream bundle ID after references are patched."""
    old_stem = "com.decode." + app["upstream_slug"]
    new_stem = "com.decodeanalytics." + app["product_id"]
    renamed = 0
    # Process deepest paths first; never rename content in Git metadata, build outputs or vendor trees.
    paths = sorted(repo.rglob("*"), key=lambda item: len(item.parts), reverse=True)
    for path in paths:
        try:
            relative = path.relative_to(repo)
        except ValueError:
            continue
        if not path.is_file() or path.is_symlink():
            continue
        if any(part in SKIP_PARTS for part in relative.parts):
            continue
        if old_stem not in path.name:
            continue
        target = path.with_name(path.name.replace(old_stem, new_stem))
        if target.exists() and target != path:
            raise RuntimeError("Refusing to overwrite duplicate branded resource: " + str(target))
        path.rename(target)
        renamed += 1
    return renamed


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
        "[Kaiqueaurelio/" + app["upstream_slug"] + "](https://github.com/Kaiqueaurelio/decode-open-source-hub/tree/main/apps/" + app["upstream_slug"] + ").\n\n"
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
    bundle_files_renamed = rename_bundle_id_files(app, repo)
    add_brand_note(app, repo)
    licenses_after = {name: (repo / name).exists() for name in licenses_before}
    if licenses_before != licenses_after or not all(licenses_after.values()):
        raise RuntimeError("License/notice files were not preserved for " + app["upstream_slug"])
    return {
        "product": app["display_name"], "status": "rebranded", "path": str(repo),
        "text_files_changed": text_changed, "theme_changed": theme_changed,
        "icon_assets_replaced": icons_changed, "bundle_id_files_renamed": bundle_files_renamed,
        "license_files_preserved": True,
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
        patch_marker_sample = '"' + app["upstream_name"] + ' patch: preserves an upstream fix"'
        patch_marker_changed = replace_rust_user_strings(
            patch_marker_sample, app["upstream_name"], app["display_name"]
        )
        assert app["upstream_name"] + " patch:" in patch_marker_changed, (
            "Upstream patch provenance labels must not be rebranded inside Rust string literals."
        )
        sample_id = 'const ID = "com.decode.' + app["upstream_slug"] + '";'
        assert "com.decodeanalytics." + app["product_id"] in replace_bundle_id(sample_id, app["upstream_slug"], app["product_id"])
        sample_cli_id = 'const ID = "com.decode.' + app["upstream_slug"] + '-cli";'
        assert "com.decodeanalytics." + app["product_id"] + "-cli" in replace_bundle_id(
            sample_cli_id, app["upstream_slug"], app["product_id"]
        ), "Bundle-ID suffixes such as CLI must be rebranded."
        sample_release_name = app["upstream_slug"] + "-<version>-linux-x86_64.flatpak"
        assert app["product_id"] + "-<version>" in replace_markdown_package_names(
            sample_release_name, app["upstream_slug"], app["product_id"]
        ), "Documented release filenames must use the Decode product ID."
        sample_icon_path = "assets/app-icon/com.decode." + app["upstream_slug"] + ".png"
        assert "assets/app-icon/com.decodeanalytics." + app["product_id"] + ".png" in replace_bundle_id(sample_icon_path, app["upstream_slug"], app["product_id"])
    # Regression test: a Rust include_str! path and the resource filename must agree after rebinding.
    app = products[0]
    old_filename = "com.decode." + app["upstream_slug"] + ".desktop"
    new_filename = "com.decodeanalytics." + app["product_id"] + ".desktop"
    with tempfile.TemporaryDirectory() as temporary:
        fixture_root = Path(temporary)
        fixture_file = fixture_root / "packaging" / "linux" / old_filename
        fixture_file.parent.mkdir(parents=True)
        fixture_file.write_text("[Desktop Entry]\n", encoding="utf-8")
        source_reference = 'include_str!("../../../packaging/linux/' + old_filename + '")'
        rewritten_reference = replace_bundle_id(source_reference, app["upstream_slug"], app["product_id"])
        renamed_count = rename_bundle_id_files(app, fixture_root)
        assert renamed_count == 1, "Bundle-ID resource filename was not renamed."
        assert (fixture_file.parent / new_filename).is_file(), "Branded Linux desktop resource is missing."
        assert new_filename in rewritten_reference, "Embedded resource reference and renamed filename disagree."
    theme_sample = "accent: pick(dark, Color32::from_rgb(0x10, 0x20, 0x30), Color32::from_rgb(0x40, 0x50, 0x60)),\n"
    patched = replace_theme_colour_calls(theme_sample)
    assert "0x25, 0x57, 0xC6" in patched and "0x4C, 0x85, 0xF7" in patched
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
