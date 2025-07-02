# Workspace Setup

## Rust

Install Rustup (https://rustup.rs/)

```bash
cargo install cargo-deny
cargo install cargo-nextest
```

## VS-Code

### Recommended Settings

```json
{
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.formatOnPaste": true
  },
  "todo-tree.general.tags": [
    "BUG",
    "HACK",
    "FIXME",
    "TODO",
    "XXX",
    "[ ]",
    "[x]",
    "todo!",
    "todo("
  ],
  "errorLens.replaceLinebreaksSymbol": "⏎ ",
  "errorLens.scrollbarHackEnabled": true
}
```
