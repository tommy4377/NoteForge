# Contributing to NoteForge

NoteForge is a lightweight desktop text and Markdown editor built with Svelte 5, CodeMirror 6, Rust and Tauri 2.

## Before opening a pull request

Run:

```bash
npm ci
npm run check
npm run build
cd src-tauri
cargo fmt --check
cargo check
cargo test
```

Keep changes focused and preserve existing file-handling behavior, encoding support, tab/session state and keyboard shortcuts.

Do not commit generated build output, logs, credentials, screenshots used only for debugging, AI-agent prompts or memory, local editor state, temporary reports, or machine-specific configuration.

Use focused commit messages such as `feat(editor): ...`, `fix(files): ...`, `refactor(ui): ...`, `test: ...` and `docs: ...`.
