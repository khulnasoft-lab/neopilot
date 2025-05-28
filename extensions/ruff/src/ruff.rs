use neopilot::LanguageServerId;
use neopilot_extension_api::{self as neopilot, Result, settings::LspSettings};
use std::fs;

struct RuffBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct RuffExtension {
    cached_binary_path: Option<String>,
}

impl RuffExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &neopilot::Worktree,
    ) -> Result<RuffBinary> {
        let binary_settings = LspSettings::for_worktree("ruff", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(RuffBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("ruff") {
            return Ok(RuffBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(RuffBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        neopilot::set_language_server_installation_status(
            language_server_id,
            &neopilot::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = neopilot::latest_github_release(
            "astral-sh/ruff",
            neopilot::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = neopilot::current_platform();

        let asset_stem = format!(
            "ruff-{arch}-{os}",
            arch = match arch {
                neopilot::Architecture::Aarch64 => "aarch64",
                neopilot::Architecture::X86 => "x86",
                neopilot::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                neopilot::Os::Mac => "apple-darwin",
                neopilot::Os::Linux => "unknown-linux-gnu",
                neopilot::Os::Windows => "pc-windows-msvc",
            }
        );
        let asset_name = format!(
            "{asset_stem}.{suffix}",
            suffix = match platform {
                neopilot::Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("ruff-{}", release.version);
        let binary_path = match platform {
            neopilot::Os::Windows => format!("{version_dir}/ruff.exe"),
            _ => format!("{version_dir}/{asset_stem}/ruff"),
        };

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            neopilot::set_language_server_installation_status(
                language_server_id,
                &neopilot::LanguageServerInstallationStatus::Downloading,
            );
            let file_kind = match platform {
                neopilot::Os::Windows => neopilot::DownloadedFileType::Zip,
                _ => neopilot::DownloadedFileType::GzipTar,
            };
            neopilot::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(RuffBinary {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl neopilot::Extension for RuffExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &neopilot::Worktree,
    ) -> Result<neopilot::Command> {
        let ruff_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(neopilot::Command {
            command: ruff_binary.path,
            args: ruff_binary.args.unwrap_or_else(|| vec!["server".into()]),
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &neopilot_extension_api::Worktree,
    ) -> Result<Option<neopilot_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &neopilot_extension_api::Worktree,
    ) -> Result<Option<neopilot_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

neopilot::register_extension!(RuffExtension);
