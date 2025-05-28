{
  mkShell,
  makeFontsConf,

  neopilot-editor,

  rust-analyzer,
  cargo-nextest,
  cargo-hakari,
  cargo-machete,
  nixfmt-rfc-style,
  protobuf,
  nodejs_22,
}:
(mkShell.override { inherit (neopilot-editor) stdenv; }) {
  inputsFrom = [ neopilot-editor ];
  packages = [
    rust-analyzer
    cargo-nextest
    cargo-hakari
    cargo-machete
    nixfmt-rfc-style
    # TODO: package protobuf-language-server for editing neopilot.proto
    # TODO: add other tools used in our scripts

    # `build.nix` adds this to the `neopilot-editor` wrapper (see `postFixup`)
    # we'll just put it on `$PATH`:
    nodejs_22
  ];

  env =
    let
      baseEnvs =
        (neopilot-editor.overrideAttrs (attrs: {
          passthru = { inherit (attrs) env; };
        })).env; # exfil `env`; it's not in drvAttrs
    in
    (removeAttrs baseEnvs [
      "LK_CUSTOM_WEBRTC" # download the staticlib during the build as usual
      "NEOPILOT_UPDATE_EXPLANATION" # allow auto-updates
      "CARGO_PROFILE" # let you specify the profile
      "TARGET_DIR"
    ])
    // {
      # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
      # outside the nix store instead of to `$src`
      FONTCONFIG_FILE = makeFontsConf {
        fontDirectories = [
          "./assets/fonts/plex-mono"
          "./assets/fonts/plex-sans"
        ];
      };
      PROTOC = "${protobuf}/bin/protoc";
    };
}
