use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct MisePackageManager;

impl PackageManager for MisePackageManager {
    fn name(&self) -> &str {
        "mise"
    }

    fn is_installed(&self) -> bool {
        command_exists("mise")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // mise routes commands through shims (e.g. ~/.local/share/mise/shims)
        // or, with shimless activation, through paths under the install root
        // (~/.local/share/mise/installs/<plugin>/<version>/...). If we have a
        // concrete path, ask mise directly via `mise which`.
        if let Some(path) = cmd.path() {
            for dir in mise_path_prefixes() {
                if path.starts_with(&dir) {
                    return Ok(true);
                }
            }
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("mise")
            .arg("which")
            .arg(name)
            .output()
            .map_err(|e| format!("failed to run mise: {}", e))?;
        Ok(output.status.success())
    }
}

fn mise_path_prefixes() -> Vec<String> {
    let home = std::env::var("HOME").ok();
    let xdg_data = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| home.as_ref().map(|h| format!("{}/.local/share", h)));

    let mut prefixes = Vec::new();
    if let Ok(data_dir) = std::env::var("MISE_DATA_DIR") {
        if !data_dir.is_empty() {
            prefixes.push(format!("{}/installs/", data_dir));
            prefixes.push(format!("{}/shims/", data_dir));
        }
    }
    if let Some(d) = xdg_data {
        prefixes.push(format!("{}/mise/installs/", d));
        prefixes.push(format!("{}/mise/shims/", d));
    }
    if let Some(h) = home {
        prefixes.push(format!("{}/.local/share/mise/installs/", h));
        prefixes.push(format!("{}/.local/share/mise/shims/", h));
    }
    prefixes
}
