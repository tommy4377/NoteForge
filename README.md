<div align="center">

# NoteForge

**A focused desktop editor for text, Markdown and LaTeX.**

[![CI](https://github.com/tommy4377/NoteForge/actions/workflows/ci.yml/badge.svg)](https://github.com/tommy4377/NoteForge/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/tommy4377/NoteForge?display_name=tag&sort=semver)](https://github.com/tommy4377/NoteForge/releases/latest)
[![License](https://img.shields.io/github/license/tommy4377/NoteForge)](LICENSE)
![Windows](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-0078D4?logo=windows11&logoColor=white)

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri%202-24C8DB?logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte%205-FF3E00?logo=svelte&logoColor=white)
![CodeMirror](https://img.shields.io/badge/CodeMirror%206-D30707)
![KaTeX](https://img.shields.io/badge/KaTeX-LaTeX-008080)

</div>

NoteForge is a lightweight Windows editor built with Svelte 5, CodeMirror 6, Rust and Tauri 2. It keeps the workflow simple while adding the features that plain text and Markdown editing actually benefit from: tabs, live preview, LaTeX, syntax highlighting, session restore and native file handling.

## Highlights

- **Multi-tab editing** with dirty-state tracking and session restore.
- **CodeMirror 6 editor** with history, bracket matching, indentation and language-aware syntax support.
- **Markdown preview** rendered with `marked`.
- **LaTeX rendering** through KaTeX for inline and display math.
- **Syntax-highlighted code blocks** with highlight.js.
- **Native open/save dialogs**, drag and drop, recent files and external-change detection.
- **Find & replace**, go-to-line, word wrap and zoom controls.
- **Autosave** with configurable interval.
- **Light, dark and sepia themes** plus a custom frameless titlebar.
- **Print/PDF document mode**, custom context menu and integrated shortcut help.

## Download

Download the latest portable Windows build from [GitHub Releases](https://github.com/tommy4377/NoteForge/releases/latest).

```text
NoteForge.exe
```

Requirements:

- Windows 10 or Windows 11, 64-bit
- Microsoft Edge WebView2 Runtime

## Keyboard shortcuts

| Action | Shortcut |
| --- | --- |
| New document | `Ctrl+N` |
| Open file | `Ctrl+O` |
| Save | `Ctrl+S` |
| Save as | `Ctrl+Shift+S` |
| New tab | `Ctrl+T` |
| Close tab | `Ctrl+W` |
| Find & replace | `Ctrl+H` |
| Go to line | `Ctrl+G` |
| Toggle preview | `Ctrl+Shift+M` |
| Cycle theme | `Ctrl+Shift+T` |
| Toggle word wrap | `Alt+Z` |
| Zoom in / out / reset | `Ctrl++` / `Ctrl+-` / `Ctrl+0` |

The in-app help overlay lists the available shortcuts as the application evolves.

## Supported editing formats

NoteForge detects common text and code formats including Markdown, plain text, JavaScript/TypeScript, JSON, Python, HTML and CSS. Files can retain supported encodings and line-ending metadata when opened and saved.

## Build from source

Prerequisites: Node.js, Rust stable and the Tauri 2 Windows prerequisites.

```bash
npm ci
npm run check
npm run build

cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo check --locked --manifest-path src-tauri/Cargo.toml
cargo test --locked --manifest-path src-tauri/Cargo.toml
cargo build --release --locked --manifest-path src-tauri/Cargo.toml
```

Portable executable:

```text
src-tauri/target/release/noteforge.exe
```

Run the desktop application during development with:

```bash
npm run tauri dev
```

## Repository health

- CI validates the Svelte frontend and Rust backend.
- Tags matching `v*.*.*` publish a single portable `NoteForge.exe`.
- Dependabot tracks npm, Cargo and GitHub Actions dependencies.
- See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), [SUPPORT.md](SUPPORT.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Maintainer

Maintained by [@tommy4377](https://github.com/tommy4377).

## License

Released under the [MIT License](LICENSE).
