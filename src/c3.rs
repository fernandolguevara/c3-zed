use std::{
    fs::{self, File},
    io::ErrorKind,
    path::{Path, PathBuf},
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

    fn parse_lsp_path_from_config(content: &str) -> Option<String> {
        let json: zed::serde_json::Value = zed::serde_json::from_str(content).ok()?;

        let path = json
            .get("lsp")
            .and_then(|lsp| lsp.get("path"))
            .and_then(zed::serde_json::Value::as_str)
            .or_else(|| {
                json.get("Lsp")
                    .and_then(|lsp| lsp.get("path"))
                    .and_then(zed::serde_json::Value::as_str)
            })?;

        let trimmed = path.trim();
        if trimmed.is_empty() {
            return None;
        }

        Some(trimmed.to_string())
    }

    fn path_from_c3lsp_json(worktree: &zed::Worktree) -> Option<String> {
        let mut current = PathBuf::from(worktree.root_path());

        loop {
            for config_file in ["c3lsp.json", "cs3lsp.json"] {
                let config_path = current.join(config_file);
                let content = match fs::read_to_string(&config_path) {
                    Ok(content) => content,
                    Err(_) => continue,
                };

                let path = match Self::parse_lsp_path_from_config(&content) {
                    Some(path) => path,
                    None => continue,
                };

                let path_ref = Path::new(&path);
                if path_ref.is_absolute() {
                    return Some(path);
                }

                let resolved = current.join(path_ref);
                return Some(resolved.to_string_lossy().to_string());
            }

            if !current.pop() {
                break;
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
            let bundled_path = Self::default_lsp_path().to_string();
            let bundled_exists = Path::new(&bundled_path).exists();

            if !bundled_exists {
                if let Ok(release) = latest_github_release(
                    "fernandolguevara/c3-lsp",
                    GithubReleaseOptions {
                        pre_release: false,
                        require_assets: false,
                    },
                ) {
                    match File::open("lsp_ver") {
                        Ok(_) => {}
                        Err(e) => match e.kind() {
                            ErrorKind::NotFound => {
                                File::create("lsp_ver")
                                    .map_err(|_| "Failed load file".to_string())?;
                            }
                            _ => return Err("Failed load file".to_string()),
                        },
                    }

                    fs::write("lsp_ver", release.version.as_bytes())
                        .map_err(|_| "Failed to write file".to_string())?;
                    Self::download_lsp(&release);
                }
            }

            (bundled_path, LspPathSource::Bundled)
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
