# Fonts

<!--
TBD: WIP. Neopilot Fonts documentation. This is currently not linked from SUMMARY.md are so unpublished.
-->

Neopilot ships two fonts: Neopilot Plex Mono and Neopilot Plex Sans. These are based on IBM Plex Mono and IBM Plex Sans, respectively.

<!--
TBD: Document how Neopilot Plex font files were created. Repo links, etc.
-->

## Settings

<!--
TBD: Explain various font settings in Neopilot.
-->

- Buffer fonts
  - `buffer-font-family`
  - `buffer-font-features`
  - `buffer-font-size`
  - `buffer-line-height`
- UI fonts
  - `ui_font_family`
  - `ui_font_fallbacks`
  - `ui_font_features`
  - `ui_font_weight`
  - `ui_font_size`
- Terminal fonts
  - `terminal.font-size`
  - `terminal.font-family`
  - `terminal.font-features`
- Other settings:
  - `active-pane-magnification`

## Old Neopilot Fonts

Previously, Neopilot shipped with `Neopilot Mono` and `Neopilot Sans`, customineopilot versions of the [Iosevka](https://typeof.net/Iosevka/) typeface. You can find more about them in the [neopilot-fonts](https://github.com/khulnasoft-lab/neopilot-fonts/) repository.

Here's how you can use the old Neopilot fonts instead of `Neopilot Plex Mono` and `Neopilot Plex Sans`:

1. Download [neopilot-app-fonts-1.2.0.zip](https://github.com/khulnasoft-lab/neopilot-fonts/releases/download/1.2.0/neopilot-app-fonts-1.2.0.zip) from the [neopilot-fonts releases](https://github.com/khulnasoft-lab/neopilot-fonts/releases) page.
2. Open macOS `Font Book.app`
3. Unzip the file and drag the `ttf` files into the Font Book app.
4. Update your settings `ui_font_family` and `buffer_font_family` to use `Neopilot Mono` or `Neopilot Sans` in your `settings.json` file.

```json
{
  "ui_font_family": "Neopilot Sans Extended",
  "buffer_font_family": "Neopilot Mono Extend",
  "terminal": {
    "font-family": "Neopilot Mono Extended"
  }
}
```

5. Note there will be red squiggles under the font name. (this is a bug, but harmless.)
