use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct YarnPackageManager;

impl PackageManager for YarnPackageManager {
    fn name(&self) -> &str {
        "yarn"
    }

    fn is_installed(&self) -> bool {
        command_exists("yarn")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Yarn Classic exposes globally-installed binaries under `yarn global
        // bin`. A path outside that dir can't be a yarn global install.
        if let Some(path) = cmd.path() {
            if let Some(bin_dir) = yarn_global_bin() {
                let prefix = format!("{}/", bin_dir.trim_end_matches('/'));
                if !path.starts_with(&prefix) {
                    return Ok(false);
                }
                return Ok(true);
            }
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("yarn")
            .arg("global")
            .arg("list")
            .arg("--depth=0")
            .output()
            .map_err(|e| format!("failed to run yarn: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query yarn for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        // Lines look like: info "pkg@1.2.3" has binaries: ...
        // Keep it simple: any occurrence of the binary name on a binary line.
        Ok(s.lines()
            .any(|line| line.contains(&format!("\"{}\"", name)) || line.contains(name)))
    }
}

fn yarn_global_bin() -> Option<String> {
    let output = std::process::Command::new("yarn")
        .arg("global")
        .arg("bin")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
