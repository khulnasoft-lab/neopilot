use neopilot::settings::LspSettings;
use neopilot_extension_api::{self as neopilot, LanguageServerId, Result, serde_json};
use std::fs;

struct GlslExtension {
    cached_binary_path: Option<String>,
}

impl GlslExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &neopilot::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("glsl_analyzer") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        neopilot::set_language_server_installation_status(
            language_server_id,
            &neopilot::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = neopilot::latest_github_release(
            "nolanderc/glsl_analyzer",
            neopilot::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = neopilot::current_platform();
        let asset_name = format!(
            "{arch}-{os}.zip",
            arch = match arch {
                neopilot::Architecture::Aarch64 => "aarch64",
                neopilot::Architecture::X86 => "x86",
                neopilot::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                neopilot::Os::Mac => "macos",
                neopilot::Os::Linux => "linux-musl",
                neopilot::Os::Windows => "windows",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("glsl_analyzer-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!("{version_dir}/bin/glsl_analyzer");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            neopilot::set_language_server_installation_status(
                language_server_id,
                &neopilot::LanguageServerInstallationStatus::Downloading,
            );

            neopilot::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    neopilot::Os::Mac | neopilot::Os::Linux => neopilot::DownloadedFileType::Zip,
                    neopilot::Os::Windows => neopilot::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            neopilot::make_file_executable(&binary_path)?;

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
        Ok(binary_path)
    }
}

impl neopilot::Extension for GlslExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &neopilot::LanguageServerId,
        worktree: &neopilot::Worktree,
    ) -> Result<neopilot::Command> {
        Ok(neopilot::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &neopilot::LanguageServerId,
        worktree: &neopilot::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("glsl_analyzer", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "glsl_analyzer": settings
        })))
    }
}

neopilot::register_extension!(GlslExtension);
