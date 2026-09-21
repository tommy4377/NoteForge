# NoteForge v0.2.0

Native file workflow and editor reliability

## Highlights

- Native drag and drop file opening
- Session restore for previously open tabs
- PDF export workflow
- Go to line and workspace zoom controls
- Synchronized editor/preview scrolling and syntax highlighting

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

Copyright © 2026 Tommaso Verardi.
