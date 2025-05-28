use neopilot_extension_api::{self as neopilot, Result, settings::LspSettings};

const PROTOBUF_LANGUAGE_SERVER_NAME: &str = "protobuf-language-server";

struct ProtobufLanguageServerBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct ProtobufExtension;

impl ProtobufExtension {
    fn language_server_binary(
        &self,
        _language_server_id: &neopilot::LanguageServerId,
        worktree: &neopilot::Worktree,
    ) -> Result<ProtobufLanguageServerBinary> {
        let binary_settings = LspSettings::for_worktree("protobuf-language-server", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(ProtobufLanguageServerBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which(PROTOBUF_LANGUAGE_SERVER_NAME) {
            return Ok(ProtobufLanguageServerBinary {
                path,
                args: binary_args,
            });
        }

        Err(format!("{PROTOBUF_LANGUAGE_SERVER_NAME} not found in PATH",))
    }
}

impl neopilot::Extension for ProtobufExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &neopilot_extension_api::LanguageServerId,
        worktree: &neopilot_extension_api::Worktree,
    ) -> neopilot_extension_api::Result<neopilot_extension_api::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(neopilot::Command {
            command: binary.path,
            args: binary
                .args
                .unwrap_or_else(|| vec!["-logs".into(), "".into()]),
            env: Default::default(),
        })
    }
}

neopilot::register_extension!(ProtobufExtension);
