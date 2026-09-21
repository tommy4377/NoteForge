# NoteForge

A lightweight desktop text and Markdown editor built with Svelte 5, CodeMirror 6, Rust and Tauri 2.

## Highlights

- Multi-tab editor with dirty-state tracking and session restore
- Markdown, LaTeX and syntax-highlighted preview
- Open/save, recent files, autosave and external-change detection
- Find/replace, go-to-line, word wrap and editor/workspace zoom
- Light, dark and sepia themes with a custom desktop titlebar
- Print/PDF document mode plus custom context menu and shortcut help

## Requirements

- Windows 10 or Windows 11, 64-bit
- Microsoft Edge WebView2 Runtime

## Development

Prerequisites: Node.js 22+, Rust stable and the Tauri 2 Windows prerequisites.

```bash
npm ci
npm run check
npm run build
cd src-tauri
cargo fmt --check
cargo check
cargo test
```

Run the desktop application during development with:

```bash
npm run tauri dev
```

Build the portable executable without an installer with:

```bash
npm run build
cargo build --release --manifest-path src-tauri/Cargo.toml
```

Output:

```text
src-tauri/target/release/noteforge.exe
```

## Repository

- CI validates the Svelte frontend and Rust backend.
- Tags matching `v*.*.*` publish a single portable Windows asset named `NoteForge.exe`.
- Dependabot tracks npm, Cargo and GitHub Actions dependencies.
- See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md) and [SUPPORT.md](SUPPORT.md).

## License

NoteForge is released under the [MIT License](LICENSE).

Maintained by @tommy4377.
