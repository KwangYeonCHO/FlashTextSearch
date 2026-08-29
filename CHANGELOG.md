# Changelog

All notable changes to **FlashText Search** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.4.3] - 2026-08-29

### Fixed
- **Synchronized Package Versioning**: Fixed an issue where `Cargo.toml` compile-time package version was out of sync, preventing repeated update prompts.

---

## [v0.4.2] - 2026-08-29

### Fixed & Architecture
- **In-Place Atomic Hot-Swap (Zero-Script & 100% Silent Update)**: Completely eliminated external `.bat`/`.vbs` scripts and CMD popups. Utilized Windows kernel file renaming capability to perform atomic hot-swap and relaunch in pure Rust within milliseconds.

---

## [v0.4.1] - 2026-08-29

### Fixed & I18n
- **Smart Multilingual Release Notes Parser**: Solved language mismatch where update release notes showed Chinese in Korean or English mode. Added automated language block parsing (`<!-- lang:ko/zh/en -->` and Markdown headers) and instant language tab switcher in update dialog.

---

## [v0.4.0] - 2026-08-29

### Added
- **Word Document Search & Preview Support (`*.docx`, `*.doc`)**: Native parsing and deep full-text indexing for modern Word (`.docx`) OpenXML and legacy (`.doc`) OLE compound files.
- **Hancom Hangul Document Search & Preview Support (`*.hwp`, `*.hwpx`)**: Full parsing for Hancom Hangul HWP 5.0 deflate compressed streams (`.hwp`) and OpenXML format (`.hwpx`).
- **New Office & HWP Preset Filter**: Added dedicated format filter preset `Word/韩软 (*.docx, *.doc, *.hwpx, *.hwp)`.
- **Monaco Preview for Office Documents**: Extracted paragraphs and text content are seamlessly displayed in Monaco Editor with automatic line jumping.

---

## [v0.3.2] - 2026-08-29

### Fixed & UI
- **Completely Silent Background Update Execution**: Replaced console batch script with native Windows GUI WScript (`wscript.exe`), eliminating any black CMD window flash during automatic updates.
- **Title Version Badge**: Added stylish version badge next to the main title in the header and window title bar.

---

## [v0.3.1] - 2026-08-29

### Performance & Fixes
- **Asynchronous Zero-Lag Update Checking**: Replaced heavy synchronous GitHub CLI process with ultra-fast lightweight background async HTTP query (`CREATE_NO_WINDOW` and strict timeouts), eliminating any UI stutter during startup.
- **Updated Window Screenshot**: Updated official README screenshot with latest UI featuring updater controls and full layout.

---

## [v0.3.0] - 2026-08-29

### Added
- **GitHub In-App Auto Updater**: Integrated `updater_service.rs` and `UpdateModal.vue` allowing one-click update and restart directly from latest GitHub Releases.
- **Open Source Community Infrastructure**: Added `README.md` (Tri-lingual), `CONTRIBUTING.md`, `SECURITY.md`, `LICENSE` (MIT), GitHub Issue & PR templates, and GitHub Actions workflow.
- **50,000-Row Large Document Support**: Expanded preview row capacity up to 50,000 lines for large spreadsheets and code files.

### Changed
- Converted GitHub repository visibility to **Public** for community access.

---

## [v0.2.2] - 2026-08-29

### Fixed
- Fixed right-side preview truncation issue that restricted spreadsheet preview to 500 rows.
- Enhanced Excel cell coordinate jumping and smooth scrolling directly to target row & column.

---

## [v0.2.1] - 2026-08-29

### Fixed
- Fixed search history dropdown flashing/disappearing issue by replacing click-outside event bubbling with robust container mousedown boundary checks.

---

## [v0.2.0] - 2026-08-29

### Added
- **Persistent Search Memory**: Search directory paths and keywords are automatically saved and restored across sessions.
- **Search History Dropdown**: Interactive dropdown with individual record deletion (`✕`) and clear-all functionality.
- **Light Theme (晨曦素雅)**: Complete modern redesign for crisp light-mode aesthetic with Monaco editor theme synchronization (`vs` / `vs-dark`).

---

## [v0.1.0] - 2026-08-29

### Added
- Initial release of **FlashText Search**.
- Zero-copy Rust search engine (`memmap2`, `aho-corasick`, `ignore`, `rayon`).
- Full Excel spreadsheet searching (`calamine`).
- Embedded Monaco Code/Text Editor and Excel table viewer.
- Tri-lingual support (🇨🇳 中文, 🇰🇷 한국어, 🇺🇸 English) and 4 themes.
- Streaming real-time IPC progress bar and throughput reporting.
