import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const html = await readFile(new URL("./index.html", import.meta.url), "utf8");
const scriptMatch = html.match(/<script>([\s\S]*?)<\/script>/i);
assert.ok(scriptMatch, "index.html must include its script");
const script = scriptMatch[1];
assert.doesNotThrow(() => new vm.Script(script), "inline JavaScript must parse");

const catalogMatch = script.match(/var apps = (\[[\s\S]*?\]);\s*var categories=/);
assert.ok(catalogMatch, "app catalog must be present");
const apps = vm.runInNewContext("(" + catalogMatch[1] + ")");
const ids = apps.map((app) => app.id);
assert.equal(apps.length, 14, "catalog must list 13 Craft apps and Decode CAD");
assert.equal(new Set(ids).size, ids.length, "catalog IDs must be unique");
assert.deepEqual([...ids].sort(), [
  "cadcraft","decodecad","deckcraft","designcraft","effectcraft","figcraft",
  "filmcraft","gridcraft","lightcraft","pdfcraft","photocraft","soundcraft",
  "vectorcraft","wordcraft"
].sort(), "catalog must contain all expected apps");
for (const app of apps) {
  assert.ok(app.name && app.desc && app.cat && app.repo, "incomplete catalog entry: " + app.id);
  assert.ok(/^https:\/\//.test(app.site), "site must use HTTPS: " + app.id);
  assert.ok(/^https:\/\/raw\.githubusercontent\.com\//.test(app.icon), "icon must use HTTPS: " + app.id);
}
assert.match(html, /id="architectureSelect"/, "architecture selector must be present");
assert.match(html, /Arquitetura do processador/, "architecture selector must have an accessible label");
assert.match(script, /function selectedArchitecture\(\)/, "automatic architecture detection must exist");
assert.match(script, /data-download-app=/, "primary download links must opt into the download handler");
assert.match(script, /grid\.addEventListener\("click",handleDownload\)/, "download handler must be connected");
assert.match(script, /var os=platform\.value,arch=selectedArchitecture\(\)/, "system and architecture must be captured at click time");
assert.match(script, /chooseAsset\(app,os,arch\)/, "download must use the captured platform selection");
assert.match(script, /function safeHttps\(url,fallback\)/, "download URLs must be HTTPS-validated");
assert.match(script, /site:"https:\/\/getartcraft\.com\/apps\/pdfcraft"/, "PdfCraft must use its current official page");
assert.match(script, /site:"https:\/\/getartcraft\.com\/apps\/deckcraft"/, "DeckCraft must use its current official page");
assert.match(html, /option value="x86"/, "32-bit Windows option must be present");
assert.match(script, /filmcraft\/main\/assets\/app-icon\/filmcraft-1024\.png/, "FilmCraft icon path must exist in repository");
assert.match(script, /effectcraft\/main\/assets\/app-icon\/effectcraft-1024\.png/, "EffectCraft icon path must exist in repository");
assert.match(script, /figcraft\/main\/assets\/app-icon\/figcraft-256\.png/, "FigCraft icon path must exist in repository");
assert.doesNotMatch(script, /apps\.forEach\(function\(app\)\{loadRelease\(app\);\}\)/, "page must not query all releases on startup");
assert.equal((script.match(/\bfetch\(/g) || []).length, 1, "the source should have one reusable release request site");
assert.match(script, /function chooseAsset\(app,os,arch\)/, "asset selection must include architecture");

const getAssetsMatch = script.match(/function getAssets\(release\)\{[\s\S]*?\}/);
const chooseAssetMatch = script.match(/function chooseAsset\(app,os,arch\)\{[\s\S]*?\n\}/);
assert.ok(getAssetsMatch && chooseAssetMatch, "asset selection functions must be testable");
const createChooser = vm.runInNewContext(
  "(function(releases){" + getAssetsMatch[0] + "\n" + chooseAssetMatch[0] + "\nreturn chooseAsset;})"
);
function pick(assets, os, arch) {
  const choose = createChooser({sample:{assets}});
  return choose({id:"sample"}, os, arch);
}
assert.equal(pick([
  {name:"demo-aarch64.deb",browser_download_url:"https://example.org/arm.deb"},
  {name:"demo-x86_64.deb",browser_download_url:"https://example.org/x86.deb"}
], "linux", "x86_64")?.name, "demo-x86_64.deb", "x86_64 Linux must not select ARM64 packages");
assert.equal(pick([
  {name:"demo-aarch64.deb"},
  {name:"demo-x86_64.deb"}
], "linux", "arm64")?.name, "demo-aarch64.deb", "ARM64 Linux must select ARM64 packages");
assert.equal(pick([
  {name:"demo-arm64.msi"},
  {name:"demo-x64.msi"}
], "windows", "x86_64")?.name, "demo-x64.msi", "Windows architecture selection must match x64");
assert.equal(pick([{name:"demo-aarch64.AppImage"}], "linux", "x86_64"), null, "must not select explicitly incompatible packages");
assert.equal(pick([{name:"demo-x86.msi"}], "windows", "x86_64"), null, "must not select 32-bit Windows packages for x64");
assert.equal(pick([{name:"demo-x86.msi"}], "windows", "x86")?.name, "demo-x86.msi", "32-bit Windows must select 32-bit packages");
assert.equal(pick([{name:"demo-windows-x64.msi"}], "linux", "x86_64"), null, "Linux must never select a Windows installer");
assert.equal(pick([{name:"demo-freebsd-x86_64.tar.gz"}], "linux", "x86_64"), null, "Linux must never select a FreeBSD archive");
assert.equal(pick([{name:"demo-windows-x64-portable.zip"}], "macos", "x86_64"), null, "macOS must never select a Windows ZIP");
assert.equal(pick([{name:"demo-linux.deb"}], "linux", "arm64"), null, "must not guess the architecture of an unlabelled Linux package");
assert.equal(pick([{name:"demo-universal.dmg"}], "macos", "arm64")?.name, "demo-universal.dmg", "explicit universal macOS packages should remain eligible");
assert.equal(pick([{name:"demo-x86_64.flatpak"}], "linux", "x86_64")?.name, "demo-x86_64.flatpak", "Flatpak is a supported Linux package format");

console.log("Hub checks passed: catalog, unique IDs, HTTPS URLs, icon paths, syntax, lazy API calls and architecture-compatible packages.");
