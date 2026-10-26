#!/usr/bin/env python3
"""Prepare local Decode-branded source workspaces for the 13 open-source products.

Examples:
  python3 scripts/prepare_decode_suite.py --workspace ~/decode-suite
  python3 scripts/prepare_decode_suite.py --workspace ~/decode-suite --build
  python3 scripts/prepare_decode_suite.py --workspace ~/decode-suite --only decode-photo --build

This creates local downstream clones, runs the shared visual-identity migration, and optionally
runs Cargo tests/builds. It does not create or overwrite upstream repositories, does not push code,
and does not publish release binaries.
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

def run(command: list[str], cwd: Path | None = None) -> None:
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=cwd, check=True)

def load_manifest(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))

def ensure_clone(app: dict[str, Any], workspace: Path) -> Path:
    target = workspace / app["product_id"]
    if target.exists():
        if not (target / ".git").is_dir():
            raise RuntimeError(f"{target} already exists but is not a Git checkout. Move it aside; nothing was overwritten.")
        remote = subprocess.run(
            ["git", "-C", str(target), "remote", "get-url", "origin"],
            capture_output=True, text=True,
        )
        upstream = subprocess.run(
            ["git", "-C", str(target), "remote", "get-url", "upstream"],
            capture_output=True, text=True,
        )
        urls = [x.stdout.strip() for x in (remote, upstream) if x.returncode == 0]
        expected = f"https://github.com/storytold/{app['upstream_slug']}.git"
        expected_alt = f"https://github.com/storytold/{app['upstream_slug']}"
        if not any(url.rstrip("/") in {expected.rstrip("/"), expected_alt} for url in urls):
            raise RuntimeError(f"{target} does not appear to be a checkout of {app['upstream_slug']}; refusing to modify it.")
        print(f"Using existing checkout: {target}", flush=True)
        return target
    url = f"https://github.com/storytold/{app['upstream_slug']}.git"
    run(["git", "clone", "--recurse-submodules", "--depth=1", url, str(target)])
    return target

def build_product(app: dict[str, Any], repo: Path) -> dict[str, Any]:
    manifest = repo / "apps" / app["upstream_slug"] / "Cargo.toml"
    if not manifest.is_file():
        return {"product": app["display_name"], "status": "no native Cargo manifest", "path": str(repo)}
    commands = [
        ("tests failed", ["cargo", "test", "--manifest-path", str(manifest), "--locked"]),
        ("release build failed", ["cargo", "build", "--release", "--manifest-path", str(manifest), "--locked"]),
    ]
    for failure_status, command in commands:
        try:
            run(command)
        except subprocess.CalledProcessError as exc:
            return {
                "product": app["display_name"],
                "status": failure_status,
                "path": str(repo),
                "exit_code": exc.returncode,
                "command": command,
            }
    return {"product": app["display_name"], "status": "tests and release build passed", "path": str(repo)}

def main() -> int:
    root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", required=True, type=Path, help="Directory for the thirteen independent downstream clones.")
    parser.add_argument("--only", help="Only prepare one product_id from decode-suite-manifest.json.")
    parser.add_argument("--build", action="store_true", help="Run Cargo tests and release build after branding. This can take considerable time.")
    parser.add_argument("--manifest", type=Path, default=root / "open-source-hub" / "decode-suite-manifest.json")
    args = parser.parse_args()
    workspace = args.workspace.expanduser().resolve()
    manifest = load_manifest(args.manifest.resolve())
    products = manifest["products"]
    if args.only:
        products = [p for p in products if p["product_id"] == args.only]
        if not products:
            parser.error("Unknown product_id: " + args.only)
    workspace.mkdir(parents=True, exist_ok=True)
    results = []
    for app in products:
        repo = ensure_clone(app, workspace)
        result: dict[str, Any] = {"product": app["display_name"], "checkout": str(repo)}
        # The source rebrand tool starts a dedicated branch and never pushes.
        results.append(result)
    brand_script = root / "scripts" / "rebrand_decode_suite.py"
    brand_command = [sys.executable, str(brand_script), "--workspace", str(workspace), "--manifest", str(args.manifest.resolve())]
    if args.only:
        brand_command.extend(["--only", args.only])
    run(brand_command)
    if args.build:
        for app in products:
            repo = workspace / app["product_id"]
            result = build_product(app, repo)
            results.append(result)
    print(json.dumps({"workspace": str(workspace), "build_requested": args.build, "results": results}, ensure_ascii=False, indent=2))
    if args.build:
        failed_statuses = {"tests failed", "release build failed"}
        failures = [item for item in results if item.get("status") in failed_statuses]
        print(f"Build summary: {len(products) - len(failures)} products passed or had no native target; {len(failures)} failed.", flush=True)
        return 1 if failures else 0
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
