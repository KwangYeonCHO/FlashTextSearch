# Contributing to FlashText Search

Thank you for your interest in contributing to FlashText Search! We welcome contributions of all kinds: bug reports, feature requests, documentation improvements, translations, and code contributions.

---

## 🛠️ Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/) (latest stable toolchain)
- [Git](https://git-scm.com/)
- Visual Studio C++ Build Tools (on Windows)

### Local Development Setup

```bash
# 1. Clone the repository
git clone https://github.com/KwangYeonCHO/FlashTextSearch.git
cd FlashTextSearch

# 2. Install frontend dependencies
npm install

# 3. Start local development mode with hot reload
npm run tauri dev
```

---

## 📐 Project Architecture

- **`src-tauri/`**: Rust Backend (Tauri 2.0 Core)
  - `src/fast_text_search.rs`: Zero-copy memory-mapped search engine with Aho-Corasick SIMD matching.
  - `src/excel_search.rs`: Native Excel reader and full-sheet cell traverser based on `calamine`.
  - `src/encoding.rs`: Multi-encoding auto detector (`chardetng` + `encoding_rs`).
  - `src/document_service.rs`: Document preview loader & native Windows Explorer integration.
  - `src/updater_service.rs`: Automated in-app GitHub Release updater.
- **`src/`**: Vue 3 Frontend (Vite + TypeScript + Tailwind CSS)
  - `components/HeaderSearchControls.vue`: Top search bar, format filters, language/theme selectors, history dropdown.
  - `components/SearchResultsList.vue`: File result list, matched items with keyword highlighting.
  - `components/DocumentPreviewPane.vue`: Preview host container with navigation buttons (F3/Shift+F3).
  - `components/MonacoEditorViewer.vue`: Embedded Monaco editor with automatic line jumping and theme switching.
  - `components/ExcelSheetViewer.vue`: Native spreadsheet grid with sticky headers and cell targeting.
  - `components/UpdateModal.vue`: In-app GitHub release update modal.
  - `i18n/`: Multilingual localization (Chinese, Korean, English).
  - `theme/`: Semantic theme system (Dark Slate, Midnight Navy, Obsidian Gold, Light Crisp).

---

## 📝 Pull Request Guidelines

1. Fork the repository and create a descriptive branch: `git checkout -b feature/my-awesome-feature`.
2. Commit your changes with clear, concise messages adhering to Conventional Commits:
   - `feat(...)`: New features
   - `fix(...)`: Bug fixes
   - `docs(...)`: Documentation updates
   - `perf(...)`: Performance optimizations
   - `refactor(...)`: Code refactoring
3. Test your changes locally:
   - `npm run build`
   - `npm run tauri build -- --no-bundle`
4. Open a Pull Request on GitHub against the `master` branch.

---

## 📄 Code of Conduct

We are committed to providing a welcoming, inclusive, and harassment-free environment for all contributors. Please treat everyone with respect and empathy.
