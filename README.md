# C3 Zed Extension

## Forked form AineeJames/c3-zed

## What's new?
* Update the tree-siiter of c3
* Update the zed-api 
* Automatically download c3lsp from [perrymason/c3-lsp](https://github.com/pherrymason/c3-lsp)
* Optional custom c3lsp path via `c3lsp.json` or Zed LSP settings

## c3lsp path priority
The extension resolves the c3lsp binary path in this order:

1. `c3lsp.json` (or `cs3lsp.json`) in your project root
2. Zed LSP settings (`lsp.c3.binary.path`)
3. Bundled auto-downloaded default (`c3lsp/server/bin/release/c3lsp`)

When `c3lsp.json` or Zed settings provide a path, the extension uses that path directly.
Relative paths are resolved from the project root.
Custom paths are not chmod'ed by the extension; ensure your binary is executable.

### `c3lsp.json` example
```json
{
  "lsp": {
    "path": "/absolute/or/relative/path/to/c3lsp"
  }
}
```

### Zed settings example
```json
{
  "lsp": {
    "c3": {
      "binary": {
        "path": "/absolute/or/relative/path/to/c3lsp"
      }
    }
  }
}
```

## Installation:
1. Open Zed's command palette `Ctrl+Shift+P` and select `extensions`
2. Search for `C3` and click on `Install`

## Manual Installation:
1. `git clone https://github.com/AineeJames/c3-zed`
2. Open Zed's command palette `Ctrl+Shift+P` and select `extensions`
3. Click on `Install Dev Extension` and select the cloned directory

## Credits:
- Tree Sitter: [c3lang/tree-sitter-c3](https://github.com/c3lang/tree-sitter-c3)
- LSP: [perrymason/c3-lsp](https://github.com/pherrymason/c3-lsp)

> [!WARNING]
> This plugin is a WIP and may not work as expected.
> This plugin hasn't been fully tested on Mac and Linux
