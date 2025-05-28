# XML

XML support is available through the [XML extension](https://github.com/sweetppro/neopilot-xml/).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-xml](https://github.com/tree-sitter-grammars/tree-sitter-xml)

## Configuration

If you have additional file extensions that are not being automatically recognineopilot as XML just add them to [file_types](../configuring-neopilot.md#file-types) in your Neopilot settings:

```json
  "file_types": {
    "XML": ["rdf", "gpx", "kml"]
  }
```
