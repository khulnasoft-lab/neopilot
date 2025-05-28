#!/usr/bin/env sh
set -eu

# Uninstalls Neopilot that was installed using the install.sh script

check_remaining_installations() {
    platform="$(uname -s)"
    if [ "$platform" = "Darwin" ]; then
        # Check for any Neopilot variants in /Applications
        remaining=$(ls -d /Applications/Neopilot*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    else
        # Check for any Neopilot variants in ~/.local
        remaining=$(ls -d "$HOME/.local/neopilot"*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    fi
}

prompt_remove_preferences() {
    printf "Do you want to keep your Neopilot preferences? [Y/n] "
    read -r response
    case "$response" in
        [nN]|[nN][oO])
            rm -rf "$HOME/.config/neopilot"
            echo "Preferences removed."
            ;;
        *)
            echo "Preferences kept."
            ;;
    esac
}

main() {
    platform="$(uname -s)"
    channel="${NEOPILOT_CHANNEL:-stable}"

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    "$platform"

    echo "Neopilot has been uninstalled"
}

linux() {
    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    db_suffix="stable"
    case "$channel" in
      stable)
        appid="dev.neopilot.Neopilot"
        db_suffix="stable"
        ;;
      nightly)
        appid="dev.neopilot.Neopilot-Nightly"
        db_suffix="nightly"
        ;;
      preview)
        appid="dev.neopilot.Neopilot-Preview"
        db_suffix="preview"
        ;;
      dev)
        appid="dev.neopilot.Neopilot-Dev"
        db_suffix="dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.neopilot.Neopilot"
        db_suffix="stable"
        ;;
    esac

    # Remove the app directory
    rm -rf "$HOME/.local/neopilot$suffix.app"

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/neopilot"

    # Remove the .desktop file
    rm -f "$HOME/.local/share/applications/${appid}.desktop"

    # Remove the database directory for this channel
    rm -rf "$HOME/.local/share/neopilot/db/0-$db_suffix"

    # Remove socket file
    rm -f "$HOME/.local/share/neopilot/neopilot-$db_suffix.sock"

    # Remove the entire Neopilot directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/.local/share/neopilot"
        prompt_remove_preferences
    fi

    rm -rf $HOME/.neopilot_server
}

macos() {
    app="Neopilot.app"
    db_suffix="stable"
    app_id="dev.neopilot.Neopilot"
    case "$channel" in
      nightly)
        app="Neopilot Nightly.app"
        db_suffix="nightly"
        app_id="dev.neopilot.Neopilot-Nightly"
        ;;
      preview)
        app="Neopilot Preview.app"
        db_suffix="preview"
        app_id="dev.neopilot.Neopilot-Preview"
        ;;
      dev)
        app="Neopilot Dev.app"
        db_suffix="dev"
        app_id="dev.neopilot.Neopilot-Dev"
        ;;
    esac

    # Remove the app bundle
    if [ -d "/Applications/$app" ]; then
        rm -rf "/Applications/$app"
    fi

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/neopilot"

    # Remove the database directory for this channel
    rm -rf "$HOME/Library/Application Support/Neopilot/db/0-$db_suffix"

    # Remove app-specific files and directories
    rm -rf "$HOME/Library/Application Support/com.apple.sharedfilelist/com.apple.LSSharedFileList.ApplicationRecentDocuments/$app_id.sfl"*
    rm -rf "$HOME/Library/Caches/$app_id"
    rm -rf "$HOME/Library/HTTPStorages/$app_id"
    rm -rf "$HOME/Library/Preferences/$app_id.plist"
    rm -rf "$HOME/Library/Saved Application State/$app_id.savedState"

    # Remove the entire Neopilot directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/Library/Application Support/Neopilot"
        rm -rf "$HOME/Library/Logs/Neopilot"

        prompt_remove_preferences
    fi

    rm -rf $HOME/.neopilot_server
}

main "$@"
