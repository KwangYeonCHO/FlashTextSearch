# Changelog

All notable changes to **FlashText Search** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
