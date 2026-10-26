import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const html = await readFile(new URL("./index.html", import.meta.url), "utf8");
const match = html.match(/<script>([\s\S]*?)<\/script>/i);
assert.ok(match, "index.html must contain its inline script");
const script = match[1];
assert.doesNotThrow(() => new vm.Script(script, { filename: "launcher/index.html" }), "inline JavaScript must parse");

const appMatch = script.match(/const apps=(\[[\s\S]*?\]);\s*const iconBase=/);
assert.ok(appMatch, "app catalog must be present");
const apps = vm.runInNewContext("(" + appMatch[1] + ")");
const ids = apps.map((app) => app.id);
assert.equal(apps.length, 14, "catalog must have 13 Craft apps plus Decode CAD");
assert.equal(new Set(ids).size, ids.length, "app IDs must be unique");
for (const app of apps) {
  assert.ok(app.name && app.desc && app.cat && app.owner && app.repo, "incomplete card: " + app.id);
  assert.ok(/^https:\/\//.test(app.site), "site URL must use HTTPS: " + app.id);
}
for (const id of ["photocraft","lightcraft","filmcraft","designcraft","vectorcraft","pdfcraft","effectcraft","wordcraft","deckcraft","gridcraft","cadcraft","soundcraft","figcraft","decodecad"]) {
  assert.ok(ids.includes(id), "missing catalog item: " + id);
}

assert.match(html, /aria-label="Arquitetura do processador"/, "architecture selector must be accessible");
assert.match(script, /function architecture\(\)/, "architecture detection must be present");
assert.doesNotMatch(script, /loadReleases\s*\(/, "the page must not query all releases on initial load");
assert.equal((script.match(/\bfetch\(/g) || []).length, 1, "only the selected app should trigger a release API request");
assert.match(script, /function assetFor\(assets,os,arch\)/, "asset picker must consider architecture");
assert.match(script, /filmcraft-1024\.png/, "FilmCraft must use its real icon path");
assert.match(script, /effectcraft-1024\.png/, "EffectCraft must use its real icon path");
assert.match(script, /figcraft-256\.png/, "FigCraft must use its real icon path");

const assetMatch = script.match(/function assetFor\(assets,os,arch\)\{[\s\S]*?\}(?=\nfunction toast)/);
assert.ok(assetMatch, "asset selection function must be extractable for tests");
const assetFor = vm.runInNewContext(assetMatch[0] + "; assetFor");
const linuxAssets = [
  { name: "cadcraft-1-linux-aarch64.deb" },
  { name: "cadcraft-1-linux-x86_64.deb" },
  { name: "cadcraft-1-linux-x86_64.AppImage" },
];
assert.equal(assetFor(linuxAssets, "linux", "x86_64")?.name, "cadcraft-1-linux-x86_64.deb", "x86_64 Linux must not get ARM64 DEB");
assert.equal(assetFor(linuxAssets, "linux", "arm64")?.name, "cadcraft-1-linux-aarch64.deb", "ARM64 Linux must get ARM64 DEB");
assert.equal(assetFor([{ name: "tool-linux-aarch64.AppImage" }], "linux", "x86_64"), undefined, "must not suggest an explicitly incompatible architecture");
assert.equal(assetFor([{ name: "tool-x86_64.flatpak" }], "linux", "x86_64")?.name, "tool-x86_64.flatpak", "Flatpak must be supported");
assert.equal(assetFor([{ name: "tool-windows-arm64.msi" }, { name: "tool-windows-x64.msi" }], "windows", "x86_64")?.name, "tool-windows-x64.msi", "Windows installer must match architecture");
assert.equal(assetFor([{ name: "tool-universal.dmg" }], "macos", "arm64")?.name, "tool-universal.dmg", "generic/universal macOS installers must remain eligible");

console.log("Launcher checks passed: 14 catalog items, unique IDs, HTTPS links, JavaScript syntax, lazy API usage, icon fallbacks, and architecture-aware package selection.");
