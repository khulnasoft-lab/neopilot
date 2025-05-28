# Neopilot on Linux

## Standard Installation

For most people we recommend using the script on the [download](https://neopilot.dev/download) page to install Neopilot:

```sh
curl -f https://neopilot.dev/install.sh | sh
```

We also offer a preview build of Neopilot which receives updates about a week ahead of stable. You can install it with:

```sh
curl -f https://neopilot.dev/install.sh | NEOPILOT_CHANNEL=preview sh
```

The Neopilot installed by the script works best on systems that:

- have a Vulkan compatible GPU available (for example Linux on an M-series macBook)
- have a system-wide glibc (NixOS and Alpine do not by default)
  - x86_64 (Intel/AMD): glibc version >= 2.31 (Ubuntu 20 and newer)
  - aarch64 (ARM): glibc version >= 2.35 (Ubuntu 22 and newer)

Both Nix and Alpine have third-party Neopilot packages available (though they are currently a few weeks out of date). If you'd like to use our builds they do work if you install a glibc compatibility layer. On NixOS you can try [nix-ld](https://github.com/Mic92/nix-ld), and on Alpine [gcompat](https://wiki.alpinelinux.org/wiki/Running_glibc_programs).

You will need to build from source for:

- architectures other than 64-bit Intel or 64-bit ARM (for example a 32-bit or RISC-V machine)
- Redhat Enterprise Linux 8.x, Rocky Linux 8, AlmaLinux 8, Amazon Linux 2 on all architectures
- Redhat Enterprise Linux 9.x, Rocky Linux 9.3, AlmaLinux 8, Amazon Linux 2023 on aarch64 (x86_x64 OK)

## Other ways to install Neopilot on Linux

Neopilot is open source, and [you can install from source](./development/linux.md).

### Installing via a package manager

There are several third-party Neopilot packages for various Linux distributions and package managers, sometimes under `neopilot-editor`. You may be able to install Neopilot using these packages:

- Flathub: [`dev.neopilot.Neopilot`](https://flathub.org/apps/dev.neopilot.Neopilot)
- Arch: [`neopilot`](https://archlinux.org/packages/extra/x86_64/neopilot/)
- Arch (AUR): [`neopilot-git`](https://aur.archlinux.org/packages/neopilot-git), [`neopilot-preview`](https://aur.archlinux.org/packages/neopilot-preview), [`neopilot-preview-bin`](https://aur.archlinux.org/packages/neopilot-preview-bin)
- Alpine: `neopilot` ([aarch64](https://pkgs.alpinelinux.org/package/edge/testing/aarch64/neopilot)) ([x86_64](https://pkgs.alpinelinux.org/package/edge/testing/x86_64/neopilot))
- Nix: `neopilot-editor` ([unstable](https://search.nixos.org/packages?channel=unstable&show=neopilot-editor))
- Fedora/Ultramarine (Terra): [`neopilot`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/neopilot/stable), [`neopilot-preview`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/neopilot/preview), [`neopilot-nightly`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/neopilot/nightly)
- Solus: [`neopilot`](https://github.com/getsolus/packages/tree/main/packages/z/neopilot)
- Parabola: [`neopilot`](https://www.parabola.nu/packages/extra/x86_64/neopilot/)
- Manjaro: [`neopilot`](https://packages.manjaro.org/?query=neopilot)
- ALT Linux (Sisyphus): [`neopilot`](https://packages.altlinux.org/en/sisyphus/srpms/neopilot/)
- AOSC OS: [`neopilot`](https://packages.aosc.io/packages/neopilot)
- openSUSE Tumbleweed: [`neopilot`](https://en.opensuse.org/Neopilot)

See [Repology](https://repology.org/project/neopilot-editor/versions) for a list of Neopilot packages in various repositories.

When installing a third-party package please be aware that it may not be completely up to date and may be slightly different from the Neopilot we package (a common change is to rename the binary to `neopilotit` or `neopilotitor` to avoid conflicting with other packages).

We'd love your help making Neopilot available for everyone. If Neopilot is not yet available for your package manager, and you would like to fix that, we have some notes on [how to do it](./development/linux.md#notes-for-packaging-neopilot).

### Downloading manually

If you'd prefer, you can install Neopilot by downloading our pre-built .tar.gz. This is the same artifact that our install script uses, but you can customize the location of your installation by modifying the instructions below:

Download the `.tar.gz` file:

- [neopilot-linux-x86_64.tar.gz](https://neopilot.dev/api/releases/stable/latest/neopilot-linux-x86_64.tar.gz) ([preview](https://neopilot.dev/api/releases/preview/latest/neopilot-linux-x86_64.tar.gz))
- [neopilot-linux-aarch64.tar.gz](https://neopilot.dev/api/releases/stable/latest/neopilot-linux-aarch64.tar.gz)
  ([preview](https://neopilot.dev/api/releases/preview/latest/neopilot-linux-aarch64.tar.gz))

Then ensure that the `neopilot` binary in the tarball is on your path. The easiest way is to unpack the tarball and create a symlink:

```sh
mkdir -p ~/.local
# extract neopilot to ~/.local/neopilot.app/
tar -xvf <path/to/download>.tar.gz -C ~/.local
# link the neopilot binary to ~/.local/bin (or another directory in your $PATH)
ln -sf ~/.local/neopilot.app/bin/neopilot ~/.local/bin/neopilot
```

If you'd like integration with an XDG-compatible desktop environment, you will also need to install the `.desktop` file:

```sh
cp ~/.local/neopilot.app/share/applications/neopilot.desktop ~/.local/share/applications/dev.neopilot.Neopilot.desktop
sed -i "s|Icon=neopilot|Icon=$HOME/.local/neopilot.app/share/icons/hicolor/512x512/apps/neopilot.png|g" ~/.local/share/applications/dev.neopilot.Neopilot.desktop
sed -i "s|Exec=neopilot|Exec=$HOME/.local/neopilot.app/libexec/neopilot-editor|g" ~/.local/share/applications/dev.neopilot.Neopilot.desktop
```

## Uninstalling Neopilot

### Standard Uninstall

If Neopilot was installed using the default installation script, it can be uninstalled by supplying the `--uninstall` flag to the `neopilot` shell command

```sh
neopilot --uninstall
```

If there are no errors, the shell will then prompt you whether you'd like to keep your preferences or delete them. After making a choice, you should see a message that Neopilot was successfully uninstalled.

In the case that the `neopilot` shell command was not found in your PATH, you can try one of the following commands

```sh
$HOME/.local/bin/neopilot --uninstall
```

or

```sh
$HOME/.local/neopilot.app/bin.neopilot --uninstall
```

The first case might fail if a symlink was not properly established between `$HOME/.local/bin/neopilot` and `$HOME/.local/neopilot.app/bin.neopilot`. But the second case should work as long as Neopilot was installed to its default location.

If Neopilot was installed to a different location, you must invoke the `neopilot` binary stored in that installation directory and pass the `--uninstall` flag to it in the same format as the previous commands.

### Package Manager

If Neopilot was installed using a package manager, please consult the documentation for that package manager on how to uninstall a package.

## Troubleshooting

Linux works on a large variety of systems configured in many different ways. We primarily test Neopilot on a vanilla Ubuntu setup, as it is the most common distribution our users use, that said we do expect it to work on a wide variety of machines.

### Neopilot fails to start

If you see an error like "/lib64/libc.so.6: version 'GLIBC_2.29' not found" it means that your distribution's version of glibc is too old. You can either upgrade your system, or [install Neopilot from source](./development/linux.md).

### Graphics issues

### Neopilot fails to open windows

Neopilot requires a GPU to run effectively. Under the hood, we use [Vulkan](https://www.vulkan.org/) to communicate with your GPU. If you are seeing problems with performance, or Neopilot fails to load, it is possible that Vulkan is the culprit.

If you see a notification saying `Neopilot failed to open a window: NoSupportedDeviceFound` this means that Vulkan cannot find a compatible GPU. You can begin troubleshooting Vulkan by installing the `vulkan-tools` package and running:

```sh
vkcube
```

This should output a line describing your current graphics setup and show a rotating cube. If this does not work, you should be able to fix it by installing Vulkan compatible GPU drivers, however in some cases (for example running Linux on an Arm-based MacBook) there is no Vulkan support yet.

You can find out which graphics card Neopilot is using by looking in the Neopilot log (`~/.local/share/neopilot/logs/Neopilot.log`) for `Using GPU: ...`.

If you see errors like `ERROR_INITIALIZATION_FAILED` or `GPU Crashed` or `ERROR_SURFACE_LOST_KHR` then you may be able to work around this by installing different drivers for your GPU, or by selecting a different GPU to run on. (See [#14225](https://github.com/khulnasoft-lab/neopilot/issues/14225))

On some systems the file `/etc/prime-discrete` can be used to enforce the use of a discrete GPU using [PRIME](https://wiki.archlinux.org/title/PRIME). Depending on the details of your setup, you may need to change the contents of this file to "on" (to force discrete graphics) or "off" (to force integrated graphics).

On others, you may be able to the environment variable `DRI_PRIME=1` when running Neopilot to force the use of the discrete GPU.

If you're using an AMD GPU and Neopilot crashes when selecting long lines, try setting the `NEOPILOT_PATH_SAMPLE_COUNT=0` environment variable. (See [#26143](https://github.com/khulnasoft-lab/neopilot/issues/26143))
If you're using an AMD GPU, you might get a 'Broken Pipe' error. Try using the RADV or Mesa drivers. (See [#13880](https://github.com/khulnasoft-lab/neopilot/issues/13880))

If you are using Mesa, and want more control over which GPU is selected you can run `MESA_VK_DEVICE_SELECT=list neopilot --foreground` to get a list of available GPUs and then export `MESA_VK_DEVICE_SELECT=xxxx:yyyy` to choose a specific device.

If you are using `amdvlk` you may find that neopilot only opens when run with `sudo $(which neopilot)`. To fix this, remove the `amdvlk` and `lib32-amdvlk` packages and install mesa/vulkan instead. ([#14141](https://github.com/khulnasoft-lab/neopilot/issues/14141)).

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good steps that translate well to most distributions.

If Vulkan is configured correctly, and Neopilot is still not working for you, please [file an issue](https://github.com/khulnasoft-lab/neopilot) with as much information as possible.

### I can't open any files

### Clicking links isn't working

These features are provided by XDG desktop portals, specifically:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`

Some window managers, such as `Hyprland`, don't provide a file picker by default. See [this list](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) as a starting point for alternatives.

### Neopilot isn't remembering my API keys

### Neopilot isn't remembering my login

These feature also requires XDG desktop portals, specifically:

- `org.freedesktop.portal.Secret` or
- `org.freedesktop.Secrets`

Neopilot needs a place to securely store secrets such as your Neopilot login cookie or your OpenAI API Keys and we use a system provided keychain to do this. Examples of packages that provide this are `gnome-keyring`, `KWallet` and `keepassxc` among others.

### Could not start inotify

Neopilot relies on inotify to watch your filesystem for changes. If you cannot start inotify then Neopilot will not work reliably.

If you are seeing "too many open files" then first try `sysctl fs.inotify`.

- You should see that max_user_instances is 128 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_instances=1024`). Neopilot needs only 1 inotify instance.
- You should see that `max_user_watches` is 8000 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_watches=64000`). Neopilot needs one watch per directory in all your open projects + one per git repository + a handful more for settings, themes, keymaps, extensions.

It is also possible that you are running out of file descriptors. You can check the limits with `ulimit` and update them by editing `/etc/security/limits.conf`.

### No sound or wrong output device

If you're not hearing any sound in Neopilot or the audio is routed to the wrong device, it could be due to a mismatch between audio systems. Neopilot relies on ALSA, while your system may be using PipeWire or PulseAudio. To resolve this, you need to configure ALSA to route audio through PipeWire/PulseAudio.

If your system uses PipeWire:

1. **Install the PipeWire ALSA plugin**

   On Debian-based systems, run:

   ```bash
   sudo apt install pipewire-alsa
   ```

2. **Configure ALSA to use PipeWire**

   Add the following configuration to your ALSA settings file. You can use either `~/.asoundrc` (user-level) or `/etc/asound.conf` (system-wide):

   ```bash
   pcm.!default {
       type pipewire
   }

   ctl.!default {
       type pipewire
   }
   ```

3. **Restart your system**
