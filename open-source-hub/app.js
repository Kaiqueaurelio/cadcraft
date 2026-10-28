(function () {
  "use strict";

  var allSoftware = [];
  var selectedCategory = "";
  var selectedStatus = "all";
  var selectedView = "grid";
  var visibleCount = 14;
  var pageSize = 14;

  var searchInput = document.getElementById("search-input");
  var filterSearchInput = document.getElementById("filter-search-input");
  var platformFilter = document.getElementById("platform-filter");
  var sortFilter = document.getElementById("sort-filter");
  var categoryChips = document.getElementById("category-chips");
  var statusTabs = document.getElementById("status-tabs");
  var softwareGrid = document.getElementById("software-grid");
  var resultCount = document.getElementById("result-count");
  var activeFilters = document.getElementById("active-filters");
  var loadMoreWrap = document.getElementById("load-more-wrap");
  var errorState = document.getElementById("error-state");
  var viewGridButton = document.getElementById("view-grid");
  var viewTableButton = document.getElementById("view-table");
  var reloadButton = document.getElementById("reload-catalog");
  var themeToggle = document.getElementById("theme-toggle");

  var appMetadata = {
    photocraft: {
      number: "01",
      role: "Edição de imagens",
      cleanName: "Photo",
      version: "v0.16.0",
      accent: "#3b82f6",
      iconBg: "#0f2042",
      icon: "photocraft.png",
      upstream: "DecodePhoto",
      appClass: "app-photocraft",
      cmd: "cd apps/decode-photo && cargo run --release"
    },
    vectorcraft: {
      number: "02",
      role: "Ilustração vetorial",
      cleanName: "Vector",
      version: "v0.17.0",
      accent: "#ff6b4a",
      iconBg: "#3d1e15",
      icon: "vectorcraft.png",
      upstream: "DecodeVector",
      appClass: "app-vectorcraft",
      cmd: "cd apps/decode-vector && cargo run --release"
    },
    filmcraft: {
      number: "03",
      role: "Edição de vídeo, cor e som",
      cleanName: "Film",
      version: "v0.13.0",
      accent: "#a855f7",
      iconBg: "#2d1645",
      icon: "filmcraft.png",
      upstream: "DecodeFilm",
      appClass: "app-filmcraft",
      cmd: "cd apps/decode-film && cargo run --release"
    },
    lightcraft: {
      number: "04",
      role: "Biblioteca de fotos e revelação RAW",
      cleanName: "Light",
      version: "v0.14.0",
      accent: "#f59e0b",
      iconBg: "#382305",
      icon: "lightcraft.png",
      upstream: "DecodeLight",
      appClass: "app-lightcraft",
      cmd: "cd apps/decode-light && cargo run --release"
    },
    pdfcraft: {
      number: "05",
      role: "Leitura, organização e proteção de PDFs",
      cleanName: "Pdf",
      version: "v0.14.0",
      accent: "#00d2ff",
      iconBg: "#062b36",
      icon: "pdfcraft.png",
      upstream: "DecodePdf",
      appClass: "app-pdfcraft",
      cmd: "cd apps/decode-pdf && cargo run --release"
    },
    effectcraft: {
      number: "06",
      role: "Motion graphics e efeitos visuais",
      cleanName: "Effect",
      version: "v0.17.0",
      accent: "#ff3b88",
      iconBg: "#3b0c1e",
      icon: "effectcraft.png",
      upstream: "DecodeEffect",
      appClass: "app-effectcraft",
      cmd: "cd apps/decode-effect && cargo run --release"
    },
    designcraft: {
      number: "07",
      role: "Diagramação e publicação",
      cleanName: "Design",
      version: "v0.15.0",
      accent: "#8b5cf6",
      iconBg: "#231545",
      icon: "designcraft.png",
      upstream: "DecodeDesign",
      appClass: "app-designcraft",
      cmd: "cd apps/decode-design && cargo run --release"
    },
    figcraft: {
      number: "08",
      role: "Design de interfaces e prototipagem",
      cleanName: "Fig",
      version: "v0.11.2",
      accent: "#00e599",
      iconBg: "#053322",
      icon: "figcraft.png",
      upstream: "DecodeFig",
      appClass: "app-figcraft",
      cmd: "cd apps/decode-fig && cargo run --release"
    },
    wordcraft: {
      number: "09",
      role: "Escrita e design de documentos",
      cleanName: "Word",
      version: "v0.11.0",
      accent: "#60a5fa",
      iconBg: "#112242",
      icon: "wordcraft.png",
      upstream: "DecodeWord",
      appClass: "app-wordcraft",
      cmd: "cd apps/decode-word && cargo run --release"
    },
    soundcraft: {
      number: "10",
      role: "Gravação, edição e mixagem de áudio",
      cleanName: "Sound",
      version: "v0.13.1",
      accent: "#38bdf8",
      iconBg: "#082838",
      icon: "soundcraft.png",
      upstream: "DecodeSound",
      appClass: "app-soundcraft",
      cmd: "cd apps/decode-sound && cargo run --release"
    },
    deckcraft: {
      number: "11",
      role: "Apresentações e slides dinâmicos",
      cleanName: "Deck",
      version: "v0.12.0",
      accent: "#f43f5e",
      iconBg: "#380d17",
      icon: "deckcraft.png",
      upstream: "DecodeDeck",
      appClass: "app-deckcraft",
      cmd: "cd apps/decode-deck && cargo run --release"
    },
    cadcraft: {
      number: "12",
      role: "CAD e desenho técnico 2D/3D",
      cleanName: "CAD",
      version: "v0.10.0",
      accent: "#14b8a6",
      iconBg: "#092e29",
      icon: "cadcraft.png",
      upstream: "DecodeCAD",
      appClass: "app-cadcraft",
      cmd: "cd apps/decode-cad && cargo run --release"
    },
    gridcraft: {
      number: "13",
      role: "Planilhas e dados",
      cleanName: "Grid",
      version: "v0.12.1",
      accent: "#22c55e",
      iconBg: "#093315",
      icon: "gridcraft.png",
      upstream: "DecodeGrid",
      appClass: "app-gridcraft",
      cmd: "cd apps/decode-grid && cargo run --release"
    },
    "decode-craft": {
      number: "14",
      role: "Estúdio de IA generativa e 3D",
      cleanName: "Craft",
      version: "v0.8.3",
      accent: "#a855f7",
      iconBg: "#2e1065",
      icon: "decode-craft.png",
      upstream: "DecodeCraft",
      appClass: "app-decode-craft",
      cmd: "cd apps/decode-craft && cargo run --release"
    },
    "decode-craftx": {
      number: "15",
      role: "Plataforma multimodal e IA criativa",
      cleanName: "CraftX",
      version: "v0.8.3",
      accent: "#ec4899",
      iconBg: "#500724",
      icon: "decode-craftx.png",
      upstream: "DecodeCraftX",
      appClass: "app-decode-craftx",
      cmd: "cd apps/decode-craftx && cargo run --release"
    },
    "decode-launcher": {
      number: "16",
      role: "Instalador e atualizador da suíte",
      cleanName: "Launcher",
      version: "v0.8.3",
      accent: "#fbbf24",
      iconBg: "#382905",
      icon: "decode-launcher.png",
      upstream: "Decode Launcher",
      appClass: "app-decode-launcher",
      cmd: "cd launcher && cargo run --release -p decode-launcher"
    },
    'decode-photo': appMetadata.photocraft,
    'decode-vector': appMetadata.vectorcraft,
    'decode-film': appMetadata.filmcraft,
    'decode-light': appMetadata.lightcraft,
    'decode-design': appMetadata.designcraft,
    'decode-pdf': appMetadata.pdfcraft,
    'decode-effect': appMetadata.effectcraft,
    'decode-word': appMetadata.wordcraft,
    'decode-deck': appMetadata.deckcraft,
    'decode-grid': appMetadata.gridcraft,
    'decode-cad': appMetadata.cadcraft,
    'decode-sound': appMetadata.soundcraft,
    'decode-fig': appMetadata.figcraft,
    'decode-clip': {
      number: '17',
      role: 'Geração e edição de clipes virais com IA',
      cleanName: 'Clip',
      version: 'v1.0.0',
      accent: '#06b6d4',
      iconBg: '#083344',
      icon: 'decode-clip.png',
      upstream: 'DecodeClip',
      appClass: 'app-decode-clip',
      cmd: 'cd apps/decode-clip && ./start.sh'
    },

  };

  var appColors = {
    photocraft: "#2f7bf5",
    vectorcraft: "#e8573f",
    filmcraft: "#8b5cf6",
    lightcraft: "#f2a516",
    pdfcraft: "#12a58a",
    effectcraft: "#e0368f",
    designcraft: "#7bb51c",
    figcraft: "#02da96",
    wordcraft: "#4c6ef5",
    soundcraft: "#14a9c4",
    deckcraft: "#f26b1d",
    cadcraft: "#14a3c7",
    gridcraft: "#1f9d55",
    "decode-craft": "#a855f7",
    "decode-craftx": "#ec4899",
    "decode-launcher": "#2d81ff"
  };

  function escapeHtml(value) {
    return String(value == null ? "" : value).replace(/[&<>"']/g, function (char) {
      return { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[char];
    });
  }

  function safeUrl(value) {
    try {
      var parsed = new URL(value);
      return parsed.protocol === "https:" ? parsed.href : "";
    } catch (error) {
      return "";
    }
  }

  function getCategories() {
    var counts = {};
    allSoftware.forEach(function (item) {
      counts[item.category] = (counts[item.category] || 0) + 1;
    });
    return Object.keys(counts)
      .sort(function (a, b) { return a.localeCompare(b, "pt-BR"); })
      .map(function (name) {
        return { name: name, count: counts[name] };
      });
  }

  function renderCategories() {
    if (!categoryChips) return;
    var categories = getCategories();
    var html = '<button class="category-chip ' + (selectedCategory === "" ? "active" : "") + '" type="button" data-category="">Todas <span class="chip-count">' + allSoftware.length + '</span></button>';
    categories.forEach(function (category) {
      html += '<button class="category-chip ' + (selectedCategory === category.name ? "active" : "") + '" type="button" data-category="' + escapeHtml(category.name) + '">' + escapeHtml(category.name) + '<span class="chip-count">' + category.count + '</span></button>';
    });
    categoryChips.innerHTML = html;

    var statTotal = document.getElementById("stat-total");
    if (statTotal) statTotal.textContent = String(allSoftware.length);
    var statCategories = document.getElementById("stat-categories");
    if (statCategories) statCategories.textContent = String(categories.length);
  }

  function statusMatches(item, status) {
    var stage = String(item.stage || "");
    if (status === "alpha") return /alpha/i.test(stage);
    if (status === "development") return /desenvolvimento/i.test(stage);
    if (status === "featured") return Boolean(item.featured);
    if (status === "utility") return item.kind === "Launcher";
    return true;
  }

  function renderStatusTabs() {
    if (!statusTabs) return;
    var statuses = [
      { id: "all", label: "ALL", count: allSoftware.length },
      { id: "alpha", label: "ALPHA", count: allSoftware.filter(function (i) { return statusMatches(i, "alpha"); }).length },
      { id: "development", label: "EM DESENVOLVIMENTO", count: allSoftware.filter(function (i) { return statusMatches(i, "development"); }).length },
      { id: "featured", label: "DESTAQUES", count: allSoftware.filter(function (i) { return statusMatches(i, "featured"); }).length },
      { id: "utility", label: "UTILITÁRIO", count: allSoftware.filter(function (i) { return statusMatches(i, "utility"); }).length }
    ];
    statusTabs.innerHTML = statuses.map(function (status) {
      return '<button type="button" class="status-tab ' + (selectedStatus === status.id ? "active" : "") + '" data-status="' + status.id + '" aria-pressed="' + String(selectedStatus === status.id) + '"><span>' + status.label + '</span><b>' + status.count + '</b></button>';
    }).join("");
  }

  var detectedOs = "linux";
  function detectUserPlatform() {
    var p = (navigator.userAgentData && navigator.userAgentData.platform) || navigator.platform || navigator.userAgent || "";
    p = p.toLowerCase();
    if (p.indexOf("win") !== -1) return "windows";
    if (p.indexOf("mac") !== -1 || p.indexOf("iphone") !== -1 || p.indexOf("ipad") !== -1) return "macos";
    return "linux";
  }

  function getActiveOs() {
    var val = platformFilter ? platformFilter.value : "";
    if (val === "Windows") return "windows";
    if (val === "macOS") return "macos";
    if (val === "Linux") return "linux";
    return detectedOs;
  }

  function chooseAsset(item, os) {
    var slug = item.slug;
    var meta = appMetadata[slug] || {};
    var pkgName = meta.releaseName || ("decode-" + slug.replace("craft", ""));
    var BASE = "https://github.com/Kaiqueaurelio/decode-open-source-hub/releases";
    var VER = "v0.8.3";
    var BASE_DL = BASE + "/download/" + VER;

    if (os === "windows") {
      // Para o launcher, usar .exe direto; para apps usar a página de releases
      var winUrl = item.downloadWindowsExe || item.downloadWindows || (BASE_DL + "/" + pkgName + "-release-package.zip") || BASE;
      var winLabel = item.downloadWindowsExe ? "↓ Baixar .exe (Windows)" : "↓ Baixar Windows";
      return { label: winLabel, url: winUrl, version: VER };
    } else if (os === "macos") {
      var macUrl = item.downloadMacOS || (BASE_DL + "/" + pkgName + "-release-package.tar.gz") || BASE;
      return { label: "↓ Baixar macOS (.tar.gz)", url: macUrl, version: VER };
    } else {
      var linuxUrl = item.downloadLinux || (BASE_DL + "/" + pkgName + "-release-package.tar.gz");
      return { label: "↓ Baixar Linux (.tar.gz)", url: linuxUrl, version: VER };
    }
  }

  function renderPlatformOptions() {
    if (!platformFilter) return;
    detectedOs = detectUserPlatform();
    var platforms = [];
    allSoftware.forEach(function (item) {
      (item.platforms || []).forEach(function (platform) {
        if (platforms.indexOf(platform) === -1) platforms.push(platform);
      });
    });
    platforms.sort(function (a, b) { return a.localeCompare(b, "pt-BR"); });
    var old = platformFilter.value;
    var defaultSelection = detectedOs === "windows" ? "Windows" : (detectedOs === "macos" ? "macOS" : "Linux");
    var html = '<option value="">Todas as plataformas</option>' + platforms.map(function (platform) {
      var isDetected = (platform.toLowerCase() === detectedOs);
      var label = platform + (isDetected ? " (Seu Sistema)" : "");
      return '<option value="' + escapeHtml(platform) + '">' + escapeHtml(label) + '</option>';
    }).join("");
    platformFilter.innerHTML = html;
    platformFilter.value = old || defaultSelection || "";
  }

  function normalizeText(str) {
    return String(str || "")
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .toLowerCase();
  }

  function getSearchTerms() {
    var fVal = filterSearchInput ? filterSearchInput.value : "";
    var sVal = searchInput ? searchInput.value : "";
    var raw = (fVal || sVal || "").trim();
    if (!raw) return [];
    return normalizeText(raw).split(/\s+/).filter(function (t) { return t.length > 0; });
  }

  function currentSearch() {
    var terms = getSearchTerms();
    return terms.join(" ");
  }

  function calculateScore(item, terms) {
    if (!terms.length) return 0;
    var nameNorm = normalizeText(item.name);
    var taglineNorm = normalizeText(item.tagline);
    var descNorm = normalizeText(item.description);
    var catNorm = normalizeText(item.category);
    var slugNorm = normalizeText(item.slug);
    var tagsNorm = normalizeText((item.tags || []).join(" "));

    var totalScore = 0;
    for (var i = 0; i < terms.length; i++) {
      var t = terms[i];
      var matched = false;

      if (nameNorm.indexOf(t) !== -1) {
        totalScore += 35;
        if (nameNorm.indexOf(t) === 0 || nameNorm.indexOf(" " + t) !== -1) totalScore += 15;
        matched = true;
      }
      if (slugNorm.indexOf(t) !== -1) {
        totalScore += 25;
        matched = true;
      }
      if (tagsNorm.indexOf(t) !== -1) {
        totalScore += 20;
        matched = true;
      }
      if (taglineNorm.indexOf(t) !== -1) {
        totalScore += 12;
        matched = true;
      }
      if (catNorm.indexOf(t) !== -1) {
        totalScore += 10;
        matched = true;
      }
      if (descNorm.indexOf(t) !== -1) {
        totalScore += 6;
        matched = true;
      }

      if (!matched) return -1; // AND logic: every term must match somewhere
    }
    return totalScore;
  }

  function highlightMatches(text, terms) {
    if (!text) return "";
    var escaped = escapeHtml(text);
    if (!terms || !terms.length) return escaped;

    terms.forEach(function (term) {
      if (term.length < 2) return;
      var cleanTerm = term.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      var re = new RegExp("(" + cleanTerm + ")", "gi");
      escaped = escaped.replace(re, '<mark class="search-match">$1</mark>');
    });
    return escaped;
  }

  function getFiltered() {
    var terms = getSearchTerms();
    var platform = platformFilter ? platformFilter.value : "";

    var scored = [];
    allSoftware.forEach(function (item) {
      var score = calculateScore(item, terms);
      if (terms.length > 0 && score < 0) return;
      if (selectedCategory && item.category !== selectedCategory) return;
      if (platform && (item.platforms || []).indexOf(platform) === -1) return;
      if (selectedStatus !== "all" && !statusMatches(item, selectedStatus)) return;
      scored.push({ item: item, score: score });
    });

    var sort = sortFilter ? sortFilter.value : "featured";
    scored.sort(function (a, b) {
      if (terms.length > 0 && sort === "featured") {
        return b.score - a.score;
      }
      if (sort === "name") return a.item.name.localeCompare(b.item.name, "pt-BR");
      if (sort === "category") {
        var cat = a.item.category.localeCompare(b.item.category, "pt-BR");
        return cat || a.item.name.localeCompare(b.item.name, "pt-BR");
      }
      return Number(Boolean(b.item.featured)) - Number(Boolean(a.item.featured)) || a.item.name.localeCompare(b.item.name, "pt-BR");
    });

    return scored.map(function (s) { return s.item; });
  }

  function renderFiltersSummary() {
    if (!activeFilters) return;
    var labels = [];
    if (selectedCategory) labels.push(selectedCategory);
    if (platformFilter && platformFilter.value) labels.push(platformFilter.value);
    var statusLabel = { "alpha": "Alpha", "development": "Em desenvolvimento", "featured": "Destaques", "utility": "Utilitário" }[selectedStatus];
    if (statusLabel) labels.push(statusLabel);
    if (currentSearch()) labels.push("Busca: " + currentSearch());
    activeFilters.innerHTML = labels.map(function (label) {
      return '<span class="active-filter">' + escapeHtml(label) + '</span>';
    }).join("");
  }

  function getHeroImg(slug) {
    return "assets/apps/" + slug + "/hero.webp";
  }

  function getIconImg(slug) {
    return "assets/apps/" + slug + "/icon.webp";
  }

  function getLegacyIcon(slug) {
    // Preserve for tests and fallback
    return "assets/app-icons/" + slug + ".png";
  }

  function cardMarkup(item, index) {
    var meta = appMetadata[item.slug] || {
      number: String(index + 1).padStart(2, "0"),
      role: item.category,
      cleanName: item.name.replace(/^Decode\s*/, ""),
      version: "v0.8.3",
      accent: "#2d81ff",
      iconBg: "#112242",
      icon: item.slug + ".png",
      upstream: item.name,
      appClass: "app-" + item.slug,
      cmd: "cargo run --release"
    };

    var iconSrc = "assets/app-icons/" + (meta.icon || (item.slug + ".png"));
    var heroSrc = getHeroImg(item.slug);
    var legacySrc = getLegacyIcon(item.slug);

    var currentOs = getActiveOs();
    var asset = chooseAsset(item, currentOs);

    var cleanName = meta.cleanName || item.name.replace(/^Decode\s*/, "");
    var accentColor = meta.accent || "#2d81ff";
    var versionText = meta.version || asset.version || "v0.8.3";
    var taglineText = meta.role || item.tagline || "";

    var searchTerms = getSearchTerms();
    return '<div class="software-card ' + meta.appClass + '" id="card-' + item.slug + '" data-slug="' + item.slug + '">'
      + '<span aria-hidden="true" class="card-hover-accent" style="background:' + accentColor + ';"></span>'
      + '<div class="card-header-bar">'
      + '  <span class="card-version-badge" style="color:' + accentColor + ';">' + escapeHtml(versionText) + '</span>'
      + '  <a class="card-github-link" href="' + escapeHtml(item.sourceUrl) + '" target="_blank" rel="noopener noreferrer">GITHUB ↗</a>'
      + '</div>'
      + '<div class="card-main-content">'
      + '  <div class="card-icon-box" style="background:' + (meta.iconBg || "rgba(255,255,255,0.06)") + '; border-color:' + accentColor + '33;">'
      + '    <img class="card-app-icon-img" src="' + iconSrc + '" alt="' + escapeHtml(item.name) + '" loading="lazy" onerror="this.src=\'' + legacySrc + '\'">'
      + '  </div>'
      + '  <div class="card-title-group">'
      + '    <h3 class="card-title">'
      + '      <span class="brand-prefix">Decode</span><span class="craft-suffix" style="color:' + accentColor + ';">' + highlightMatches(cleanName, searchTerms) + '</span>'
      + '    </h3>'
      + '    <p class="card-tagline">' + highlightMatches(taglineText, searchTerms) + '</p>'
      + '  </div>'
      + '</div>'
      + '<p class="card-description">' + highlightMatches(item.description || item.tagline || "", searchTerms) + '</p>'
      + '<p class="card-last-updated">Última atualização: 27 de out. de 2026</p>'
      + '<div class="card-action-bar">'
      + '  <a class="card-btn-download" href="' + escapeHtml(asset.url) + '" target="_blank" rel="noopener noreferrer">'
      + '    <span>ABRIR / BAIXAR</span>'
      + '  </a>'
      + '  <button type="button" class="card-explore-trigger" data-open-modal="' + item.slug + '" title="Mais opções para ' + escapeHtml(item.name) + '">'
      + '    <span>···</span>'
      + '  </button>'
      + '</div>'
      + '</div>';
  }

  function rowMarkup(item) {
    var meta = appMetadata[item.slug] || {
      number: "00",
      role: item.category,
      upstream: item.name,
      appClass: "app-" + item.slug
    };

    var iconSrc = getIconImg(item.slug);
    var legacySrc = getLegacyIcon(item.slug);
    var currentOs = getActiveOs();
    var asset = chooseAsset(item, currentOs);

    return '<div class="software-row ' + meta.appClass + '" data-slug="' + item.slug + '">'
      + '<div class="table-app-cell">'
      + '  <img class="table-app-icon-img" src="' + iconSrc + '" alt="" loading="lazy" onerror="this.src=\'' + legacySrc + '\'">'
      + '  <div>'
      + '    <h3 class="table-title">' + highlightMatches(item.name, getSearchTerms()) + '</h3>'
      + '    <p class="table-tagline">' + highlightMatches(item.tagline || "", getSearchTerms()) + '</p>'
      + '  </div>'
      + '</div>'
      + '<div class="table-category">' + escapeHtml(item.category || "") + '</div>'
      + '<div class="table-status"><span class="pill-item pill-highlight">' + escapeHtml(item.stage || "") + '</span></div>'
      + '<div class="table-platforms">' + escapeHtml((item.platforms || []).join(" · ")) + '</div>'
      + '<div class="table-links">'
      + '  <a class="table-action-link table-action-download" href="' + escapeHtml(asset.url) + '" target="_blank" rel="noopener noreferrer">↓ Baixar</a>'
      + '  <button type="button" class="table-action-link" data-open-modal="' + item.slug + '">Detalhes ↗</button>'
      + '  <a class="table-action-link" href="' + escapeHtml(item.sourceUrl) + '" target="_blank" rel="noopener noreferrer">GitHub ↗</a>'
      + '</div>'
      + '</div>';
  }

  function nextUpCardMarkup() {
    return '<div class="next-up-card">'
      + '<p class="hud-label text-faint">Em Breve</p>'
      + '<h3>Mais ferramentas <span>a caminho</span>.</h3>'
      + '<p>Diga-nos o que reconstruir a seguir, teste compilações iniciais e acompanhe o desenvolvimento com a equipe Decode Analytics.</p>'
      + '<div class="mt-auto" style="margin-top:auto; padding-top:24px;">'
      + '  <a class="next-up-btn" href="https://github.com/Kaiqueaurelio/decode-open-source-hub" target="_blank" rel="noopener noreferrer">'
      + '    <svg aria-hidden="true" width="16" height="16" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>'
      + '    <span>Participe no GitHub</span>'
      + '  </a>'
      + '</div>'
      + '</div>';
  }

  function openModal(slug) {
    var item = allSoftware.find(function (i) { return i.slug === slug; });
    if (!item) return;
    var meta = appMetadata[slug] || {
      number: "00",
      role: item.category,
      upstream: item.name,
      cmd: "cargo run --release"
    };

    var modal = document.getElementById("app-modal");
    if (!modal) return;

    document.getElementById("modal-title").textContent = item.name;
    document.getElementById("modal-role").textContent = meta.number + " · " + meta.role + " · Suíte Decode";
    document.getElementById("modal-desc").textContent = item.description;
    document.getElementById("modal-cmd").textContent = meta.cmd;
    document.getElementById("modal-hero-img").src = getHeroImg(slug);
    document.getElementById("modal-icon-img").src = getIconImg(slug);
    document.getElementById("modal-github-link").href = item.sourceUrl;
    var currentOs = getActiveOs();
    var asset = chooseAsset(item, currentOs);
    var downloadBtn = document.getElementById("modal-download-link");
    if (downloadBtn) {
      downloadBtn.href = asset.url;
      downloadBtn.textContent = asset.label;
    }

    modal.classList.add("active");
    document.body.style.overflow = "hidden";
  }

  function closeModal() {
    var modal = document.getElementById("app-modal");
    if (!modal) return;
    modal.classList.remove("active");
    document.body.style.overflow = "";
  }

  function wireModalTriggers() {
    document.querySelectorAll("[data-open-modal]").forEach(function (btn) {
      btn.addEventListener("click", function (e) {
        e.preventDefault();
        e.stopPropagation();
        openModal(btn.getAttribute("data-open-modal"));
      });
    });
  }

  function render() {
    var filtered = getFiltered();
    var toShow = filtered.slice(0, visibleCount);

    if (resultCount) {
      resultCount.textContent = filtered.length === allSoftware.length
        ? filtered.length + " programas"
        : filtered.length + " de " + allSoftware.length + " programas";
    }

    renderFiltersSummary();

    if (!softwareGrid) return;

    softwareGrid.classList.toggle("table-view", selectedView === "table");

    if (!filtered.length) {
      softwareGrid.innerHTML = '<div class="empty-state"><h3>Nenhum aplicativo encontrado</h3><p>Tente outra busca ou remova algum filtro.</p><button type="button" id="empty-clear" class="clear-button" style="margin-top:12px;">Limpar filtros</button></div>';
      if (loadMoreWrap) loadMoreWrap.hidden = true;
      var emptyClear = document.getElementById("empty-clear");
      if (emptyClear) emptyClear.addEventListener("click", clearFilters);
      return;
    }

    if (selectedView === "table") {
      softwareGrid.innerHTML = '<div class="table-heading"><span>APP</span><span>CATEGORIA</span><span>STATUS</span><span>PLATAFORMAS</span><span>AÇÕES</span></div>'
        + toShow.map(rowMarkup).join("");
    } else {
      var cardsHtml = toShow.map(function (item, idx) {
        return cardMarkup(item, allSoftware.indexOf(item));
      }).join("");

      if (toShow.length === allSoftware.length || selectedCategory === "") {
        cardsHtml += nextUpCardMarkup();
      }

      softwareGrid.innerHTML = cardsHtml;
    }

    wireModalTriggers();

    if (loadMoreWrap) {
      loadMoreWrap.hidden = filtered.length <= visibleCount;
      var loadMoreBtn = document.getElementById("load-more");
      if (loadMoreBtn && filtered.length > visibleCount) {
        loadMoreBtn.textContent = "Mostrar mais programas (" + (filtered.length - visibleCount) + ") ↓";
      }
    }
  }

  function syncSearch(value) {
    if (searchInput) searchInput.value = value;
    if (filterSearchInput) filterSearchInput.value = value;
    visibleCount = pageSize;
    render();
  }

  function clearFilters() {
    selectedCategory = "";
    selectedStatus = "all";
    if (searchInput) searchInput.value = "";
    if (filterSearchInput) filterSearchInput.value = "";
    if (platformFilter) platformFilter.value = "";
    if (sortFilter) sortFilter.value = "featured";
    visibleCount = pageSize;
    renderCategories();
    renderStatusTabs();
    render();
  }

  function setCategory(category) {
    selectedCategory = category || "";
    visibleCount = pageSize;
    renderCategories();
    render();
    var lineupSec = document.getElementById("lineup") || document.getElementById("catalogo");
    if (lineupSec) lineupSec.scrollIntoView({ behavior: "smooth", block: "start" });
  }

  function setView(view) {
    selectedView = view === "table" ? "table" : "grid";
    if (viewGridButton) {
      viewGridButton.classList.toggle("active", selectedView === "grid");
      viewGridButton.setAttribute("aria-pressed", String(selectedView === "grid"));
    }
    if (viewTableButton) {
      viewTableButton.classList.toggle("active", selectedView === "table");
      viewTableButton.setAttribute("aria-pressed", String(selectedView === "table"));
    }
    render();
  }

  function setupTheme() {
    var stored = localStorage.getItem("decode-theme");
    if (stored === "dark" || stored === "light") {
      document.documentElement.setAttribute("data-theme", stored);
    }

    if (themeToggle) {
      themeToggle.addEventListener("click", function () {
        var current = document.documentElement.getAttribute("data-theme");
        var isDark = current === "dark" || (!current && window.matchMedia("(prefers-color-scheme: dark)").matches);
        var next = isDark ? "light" : "dark";
        document.documentElement.setAttribute("data-theme", next);
        localStorage.setItem("decode-theme", next);
      });
    }
  }

  function setupEvents() {
    // Global Keyboard Shortcuts (Notes-app style search)
    var isMac = /mac|iphone|ipad/i.test(navigator.userAgent || "");
    var kbdBadge = document.getElementById("search-kbd");
    if (kbdBadge) {
      kbdBadge.textContent = isMac ? "⌘K" : "Ctrl K";
    }

    document.addEventListener("keydown", function (e) {
      var isK = e.key && e.key.toLowerCase() === "k";
      var isCmdOrCtrl = e.metaKey || e.ctrlKey;
      var isSlash = e.key === "/";
      var tag = (e.target && e.target.tagName) ? e.target.tagName.toLowerCase() : "";
      var isEditable = tag === "input" || tag === "textarea" || (e.target && e.target.isContentEditable);

      if ((isCmdOrCtrl && isK) || (isSlash && !isEditable)) {
        e.preventDefault();
        var targetInput = searchInput || filterSearchInput;
        if (targetInput) {
          targetInput.focus();
          targetInput.select();
        }
      } else if (e.key === "Escape") {
        if (searchInput && document.activeElement === searchInput) {
          searchInput.value = "";
          syncSearch("");
          searchInput.blur();
        }
      }
    });

    if (searchInput) {
      searchInput.addEventListener("input", function (e) { syncSearch(e.target.value); });
    }
    if (filterSearchInput) {
      filterSearchInput.addEventListener("input", function (e) { syncSearch(e.target.value); });
    }
    var searchForm = document.getElementById("search-form");
    if (searchForm) {
      searchForm.addEventListener("submit", function (e) {
        e.preventDefault();
        var lineup = document.getElementById("lineup") || document.getElementById("catalogo");
        if (lineup) lineup.scrollIntoView({ behavior: "smooth", block: "start" });
      });
    }

    if (platformFilter) {
      platformFilter.addEventListener("change", function () {
        visibleCount = pageSize;
        render();
      });
    }
    if (sortFilter) {
      sortFilter.addEventListener("change", function () {
        visibleCount = pageSize;
        render();
      });
    }

    if (statusTabs) {
      statusTabs.addEventListener("click", function (e) {
        var btn = e.target.closest(".status-tab");
        if (!btn) return;
        selectedStatus = btn.getAttribute("data-status") || "all";
        visibleCount = pageSize;
        renderStatusTabs();
        render();
      });
    }

    if (categoryChips) {
      categoryChips.addEventListener("click", function (e) {
        var btn = e.target.closest(".category-chip");
        if (!btn) return;
        setCategory(btn.getAttribute("data-category") || "");
      });
    }

    var clearBtn = document.getElementById("clear-filters");
    if (clearBtn) clearBtn.addEventListener("click", clearFilters);

    if (viewGridButton) viewGridButton.addEventListener("click", function () { setView("grid"); });
    if (viewTableButton) viewTableButton.addEventListener("click", function () { setView("table"); });

    if (reloadButton) {
      reloadButton.addEventListener("click", function () {
        loadCatalog();
      });
    }

    var loadMoreBtn = document.getElementById("load-more");
    if (loadMoreBtn) {
      loadMoreBtn.addEventListener("click", function () {
        visibleCount += pageSize;
        render();
      });
    }

    var modalClose = document.getElementById("modal-close");
    if (modalClose) modalClose.addEventListener("click", closeModal);
    var modalBackdrop = document.getElementById("app-modal");
    if (modalBackdrop) {
      modalBackdrop.addEventListener("click", function (e) {
        if (e.target === modalBackdrop) closeModal();
      });
    }

    var copyBtn = document.getElementById("copy-cmd");
    if (copyBtn) {
      copyBtn.addEventListener("click", function () {
        var cmd = document.getElementById("modal-cmd").textContent;
        navigator.clipboard.writeText(cmd).then(function () {
          copyBtn.textContent = "COPIADO!";
          setTimeout(function () { copyBtn.textContent = "COPIAR"; }, 1800);
        });
      });
    }

    document.addEventListener("keydown", function (e) {
      if (e.key === "Escape") closeModal();
    });
  }

  function loadCatalog() {
    if (errorState) errorState.hidden = true;
    fetch("data/software.json")
      .then(function (res) {
        if (!res.ok) throw new Error("HTTP error " + res.status);
        return res.json();
      })
      .then(function (data) {
        allSoftware = Array.isArray(data) ? data : [];
        renderCategories();
        renderStatusTabs();
        renderPlatformOptions();
        render();
      })
      .catch(function (err) {
        console.error("Falha ao carregar catálogo:", err);
        if (errorState) errorState.hidden = false;
        if (softwareGrid) softwareGrid.innerHTML = "";
      });
  }

  // Expose global functions for testing / automation if needed
  window.setView = setView;
  window.renderStatusTabs = renderStatusTabs;

  // Initialize
  document.addEventListener("DOMContentLoaded", function () {
    setupTheme();
    setupEvents();
    loadCatalog();
  });
})();
