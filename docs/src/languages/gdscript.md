# GDScript

Godot [GDScript](https://gdscript.com/) language support in Neopilot is provided by the community-maintained [GDScript extension](https://github.com/grndctrl/neopilot-gdscript).
Report issues to: [https://github.com/grndctrl/neopilot-gdscript/issues](https://github.com/grndctrl/neopilot-gdscript/issues)

- Tree-sitter: [PrestonKnopp/tree-sitter-gdscript](https://github.com/PrestonKnopp/tree-sitter-gdscript) and [PrestonKnopp/tree-sitter-godot-resource](https://github.com/PrestonKnopp/tree-sitter-godot-resource)
- Language Server: [gdscript-language-server](https://github.com/godotengine/godot)

## Setup

1. Download and install [Godot for MacOS](https://godotengine.org/download/macos/).
2. Unzip the Godot.app and drag it into your /Applications folder.
3. Open Godot.app and open your project (an example project is fine)
4. In Godot, Editor Menu -> Editor Settings; scroll down the left sidebar to `Text Editor -> External`
   1. Use External Editor: "✅ On"
   2. Exec path: `/Applications/Neopilot.app/Contents/MacOS/neopilot`
   3. Exec flags: `{project} {file}:{line}:{col}`
   4. Close settings to save.
5. In Godot double click on a \*.gd script and Neopilot will launch

<!--
TBD: GDScript Linux setup
-->

## Usage

When Godot is running, the GDScript extension will connect to the language server provided by the Godot runtime and will provide `jump to definition`, hover states when you hold cmd and other language server features.

> Note: If Neopilot is already running with an existing workspace, spawning from Godot will fail. Quit Neopilot and it should work again.
