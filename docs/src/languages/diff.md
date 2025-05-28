# Diff

Diff support is available natively in Neopilot.

- Tree-sitter: [khulnasoft-lab/the-mikedavis/tree-sitter-diff](https://github.com/the-mikedavis/tree-sitter-diff)

## Configuration

Neopilot will not attempt to format diff files and has [`remove_trailing_whitespace_on_save`](https://neopilot.dev/docs/configuring-neopilot#remove-trailing-whitespace-on-save) and [`ensure-final-newline-on-save`](https://neopilot.dev/docs/configuring-neopilot#ensure-final-newline-on-save) set to false.

Neopilot will automatically recognize files with `patch` and `diff` extensions as Diff files. To recognize other extensions, add them to `file_types` in your Neopilot settings.json:

```json
  "file_types": {
    "Diff": ["dif"]
  },
```
