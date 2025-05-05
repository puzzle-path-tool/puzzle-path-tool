# Workspace Setup


## VS-Code

### Recommended Extensions

- [Rust Analyzer](vscode:extension/rust-lang.rust-analyzer)
- [Even Better Toml](vscode:extension/tamasfe.even-better-toml)
- [Dependi](vscode:extension/fill-labs.dependi)
- [Todo Tree](vscode:extension/Gruntfuggly.todo-tree)
- [Error Lens](vscode:extension/usernamehw.errorlens)
- [Lua (by Sumneko)](vscode:extension/sumneko.lua)
- [YAML (by Red Hat)](vscode:extension/redhat.vscode-yaml)
- [Draw.io Integration (Henning Dieterichs)](vscode:extension/hediet.vscode-drawio)
- [EditorConfig for VS Code](vscode:extension/EditorConfig.EditorConfig)

### Recommended Settings

```json
{
    "[rust]": {
        "editor.formatOnSave": true,
        "editor.formatOnPaste": true
    },
    "rust-analyzer.check.command": "clippy",
    "todo-tree.general.tags": [
        "BUG",
        "HACK",
        "FIXME",
        "TODO",
        "XXX",
        "[ ]",
        "[x]",
        "todo!"
    ],
    "errorLens.replaceLinebreaksSymbol": "⏎ ",
    "errorLens.scrollbarHackEnabled": true
}
```
