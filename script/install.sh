#!/usr/bin/env sh
set -eu

# Downloads the latest tarball from https://neopilot.dev/releases and unpacks it
# into ~/.local/. If you'd prefer to do this manually, instructions are at
# https://neopilot.dev/docs/linux.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${NEOPILOT_CHANNEL:-stable}"
    temp="$(mktemp -d "/tmp/neopilot-XXXXXX")"

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$platform-$arch" in
        macos-arm64* | linux-arm64* | linux-armhf | linux-aarch64)
            arch="aarch64"
            ;;
        macos-x86* | linux-x86* | linux-i686*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported platform or architecture"
            exit 1
            ;;
    esac

    if command -v curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif command -v wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    "$platform" "$@"

    if [ "$(command -v neopilot)" = "$HOME/.local/bin/neopilot" ]; then
        echo "Neopilot has been installed. Run with 'neopilot'"
    else
        echo "To run Neopilot from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run Neopilot now, '~/.local/bin/neopilot'"
    fi
}

linux() {
    if [ -n "${NEOPILOT_BUNDLE_PATH:-}" ]; then
        cp "$NEOPILOT_BUNDLE_PATH" "$temp/neopilot-linux-$arch.tar.gz"
    else
        echo "Downloading Neopilot"
        curl "https://neopilot.dev/api/releases/$channel/latest/neopilot-linux-$arch.tar.gz" > "$temp/neopilot-linux-$arch.tar.gz"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="dev.neopilot.Neopilot"
        ;;
      nightly)
        appid="dev.neopilot.Neopilot-Nightly"
        ;;
      preview)
        appid="dev.neopilot.Neopilot-Preview"
        ;;
      dev)
        appid="dev.neopilot.Neopilot-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.neopilot.Neopilot"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/neopilot$suffix.app"
    mkdir -p "$HOME/.local/neopilot$suffix.app"
    tar -xzf "$temp/neopilot-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/neopilot$suffix.app/bin/neopilot" ]; then
        ln -sf "$HOME/.local/neopilot$suffix.app/bin/neopilot" "$HOME/.local/bin/neopilot"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/neopilot$suffix.app/bin/cli" "$HOME/.local/bin/neopilot"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    cp "$HOME/.local/neopilot$suffix.app/share/applications/neopilot$suffix.desktop" "${desktop_file_path}"
    sed -i "s|Icon=neopilot|Icon=$HOME/.local/neopilot$suffix.app/share/icons/hicolor/512x512/apps/neopilot.png|g" "${desktop_file_path}"
    sed -i "s|Exec=neopilot|Exec=$HOME/.local/neopilot$suffix.app/bin/neopilot|g" "${desktop_file_path}"
}

macos() {
    echo "Downloading Neopilot"
    curl "https://neopilot.dev/api/releases/$channel/latest/Neopilot-$arch.dmg" > "$temp/Neopilot-$arch.dmg"
    hdiutil attach -quiet "$temp/Neopilot-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [ -d "/Applications/$app" ]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"

    mkdir -p "$HOME/.local/bin"
    # Link the binary
    ln -sf "/Applications/$app/Contents/MacOS/cli" "$HOME/.local/bin/neopilot"
}

main "$@"
