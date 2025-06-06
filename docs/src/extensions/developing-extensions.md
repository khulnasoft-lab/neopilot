# Developing Extensions

## Extension Capabilities

Extensions can add the following capabilities to Neopilot:

- [Languages](./languages.md)
- [Themes](./themes.md)
- [Icon Themes](./icon-themes.md)
- [Slash Commands](./slash-commands.md)
- [Context Servers](./context-servers.md)

## Developing an Extension Locally

Before starting to develop an extension for Neopilot, be sure to [install Rust via rustup](https://www.rust-lang.org/tools/install).

> Rust must be installed via rustup. If you have Rust installed via homebrew or otherwise, installing dev extensions will not work.

When developing an extension, you can use it in Neopilot without needing to publish it by installing it as a _dev extension_.

From the extensions page, click the `Install Dev Extension` button and select the directory containing your extension.

If you already have a published extension with the same name installed, your dev extension will override it.

## Directory Structure of a Neopilot Extension

A Neopilot extension is a Git repository that contains an `extension.toml`. This file must contain some
basic information about the extension:

```toml
id = "my-extension"
name = "My extension"
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "My cool extension"
repository = "https://github.com/your-name/my-neopilot-extension"
```

In addition to this, there are several other optional files and directories that can be used to add functionality to a Neopilot extension. An example directory structure of an extension that provides all capabilities is as follows:

```
my-extension/
  extension.toml
  Cargo.toml
  src/
    lib.rs
  languages/
    my-language/
      config.toml
      highlights.scm
  themes/
    my-theme.json
```

## WebAssembly

Procedural parts of extensions are written in Rust and compiled to WebAssembly. To develop an extension that includes custom code, include a `Cargo.toml` like this:

```toml
[package]
name = "my-extension"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neopilot_extension_api = "0.1.0"
```

Use the latest version of the [`neopilot_extension_api`](https://crates.io/crates/neopilot_extension_api) available on crates.io. Make sure it's still [compatible with Neopilot versions](https://github.com/khulnasoft-lab/neopilot/blob/main/crates/extension_api#compatible-neopilot-versions) you want to support.

In the `src/lib.rs` file in your Rust crate you will need to define a struct for your extension and implement the `Extension` trait, as well as use the `register_extension!` macro to register your extension:

```rs
use neopilot_extension_api as neopilot;

struct MyExtension {
    // ... state
}

impl neopilot::Extension for MyExtension {
    // ...
}

neopilot::register_extension!(MyExtension);
```

## Publishing your extension

To publish an extension, open a PR to [the `khulnasoft-lab/extensions` repo](https://github.com/khulnasoft-lab/extensions).

> Note: It is very helpful if you fork the `khulnasoft-lab/extensions` repo to a personal GitHub account instead of a GitHub organization, as this allows Neopilot staff to push any needed changes to your PR to expedite the publishing process.

In your PR, do the following:

1. Add your extension as a Git submodule within the `extensions/` directory

```sh
git submodule add https://github.com/your-username/foobar-neopilot.git extensions/foobar
git add extensions/foobar
```

2. Add a new entry to the top-level `extensions.toml` file containing your extension:

```toml
[my-extension]
submodule = "extensions/my-extension"
version = "0.0.1"
```

> If your extension is in a subdirectory within the submodule you can use the `path` field to point to where the extension resides.

3. Run `pnpm sort-extensions` to ensure `extensions.toml` and `.gitmodules` are sorted

Once your PR is merged, the extension will be packaged and published to the Neopilot extension registry.

> Extension IDs and names should not contain `neopilot` or `Neopilot`, since they are all Neopilot extensions.

## Updating an extension

To update an extension, open a PR to [the `khulnasoft-lab/extensions` repo](https://github.com/khulnasoft-lab/extensions).

In your PR do the following:

1. Update the extension's submodule to the commit of the new version.
2. Update the `version` field for the extension in `extensions.toml`
   - Make sure the `version` matches the one set in `extension.toml` at the particular commit.

If you'd like to automate this process, there is a [community GitHub Action](https://github.com/huacnlee/neopilot-extension-action) you can use.
