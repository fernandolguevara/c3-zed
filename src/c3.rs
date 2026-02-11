use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
    path::Path,
};

use zed_extension_api::{
    self as zed, current_platform, download_file, latest_github_release, make_file_executable,
    settings::LspSettings, Architecture, GithubReleaseOptions, Os, Result,
};

struct C3Extension;

enum LspPathSource {
    Configured,
    Bundled,
}

impl C3Extension {
    fn default_lsp_path() -> &'static str {
        match current_platform() {
            (Os::Windows, Architecture::X8664) => "c3lsp/server/bin/release/c3lsp.exe",
            (Os::Mac, Architecture::Aarch64) => "c3lsp/server/bin/release/c3lsp",
            (Os::Linux, Architecture::X8664) => "c3lsp/server/bin/release/c3lsp",
            _ => "no available lsp!",
        }
    }

    fn path_from_c3lsp_json(worktree: &zed::Worktree) -> Option<String> {
        for config_file in ["c3lsp.json", "cs3lsp.json"] {
            let content = match worktree.read_text_file(config_file) {
                Ok(content) => content,
                Err(_) => continue,
            };

            let json: zed::serde_json::Value = match zed::serde_json::from_str(&content) {
                Ok(json) => json,
                Err(_) => continue,
            };

            let path = json
                .get("lsp")
                .and_then(|lsp| lsp.get("path"))
                .and_then(zed::serde_json::Value::as_str)
                .or_else(|| {
                    json.get("Lsp")
                        .and_then(|lsp| lsp.get("path"))
                        .and_then(zed::serde_json::Value::as_str)
                });

            if let Some(path) = path {
                let trimmed = path.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }

        None
    }

    fn path_from_zed_settings(worktree: &zed::Worktree) -> Option<String> {
        let settings = LspSettings::for_worktree("c3", worktree).ok()?;
        let path = settings.binary?.path?;
        let trimmed = path.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(trimmed.to_string())
    }

    fn download_lsp(release: &zed::GithubRelease) {
        {
            if let Ok(_) = match current_platform() {
                (Os::Windows, Architecture::X8664) => download_file(
                    &release.assets[2].download_url,
                    "c3lsp/",
                    zed_extension_api::DownloadedFileType::Zip,
                ),
                (Os::Mac, Architecture::Aarch64) => download_file(
                    &release.assets[0].download_url,
                    "c3lsp/",
                    zed_extension_api::DownloadedFileType::Zip,
                ),
                (Os::Linux, Architecture::X8664) => download_file(
                    &release.assets[1].download_url,
                    "c3lsp/",
                    zed_extension_api::DownloadedFileType::GzipTar,
                ),
                _ => Err("no available lsp!".to_string()),
            } {}
        }
    }
}

impl zed::Extension for C3Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let configured_path =
            Self::path_from_c3lsp_json(worktree).or_else(|| Self::path_from_zed_settings(worktree));

        let (path, source) = if let Some(path) = configured_path {
            let resolved_path = if Path::new(&path).is_absolute() {
                path
            } else {
                format!("{}/{}", worktree.root_path(), path)
            };
            (resolved_path, LspPathSource::Configured)
        } else {
            if let Ok(release) = latest_github_release(
                "pherrymason/c3-lsp",
                GithubReleaseOptions {
                    pre_release: false,
                    require_assets: false,
                },
            ) {
                let mut file = match File::open("lsp_ver") {
                    Ok(file_handle) => file_handle,
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => File::create("lsp_ver").unwrap(),
                        _ => return Err("Failed load file".to_string()),
                    },
                };
                let mut content = String::new();

                file.read_to_string(&mut content).unwrap_or_default();

                if content != release.version {
                    fs::write("lsp_ver", release.version.as_bytes())
                        .map_err(|_| "Failed to write file".to_string())?;
                    Self::download_lsp(&release);
                }
            }

            (Self::default_lsp_path().to_string(), LspPathSource::Bundled)
        };

        if matches!(source, LspPathSource::Bundled) {
            make_file_executable(&path)?;
        }

        Ok(zed::Command {
            command: path.to_string(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(C3Extension);
