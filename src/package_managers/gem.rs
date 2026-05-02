use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct GemPackageManager;

impl PackageManager for GemPackageManager {
    fn name(&self) -> &str {
        "gem"
    }

    fn is_installed(&self) -> bool {
        command_exists("gem")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Path shortcut: gem-installed executables live under
        // `gem env gembindir`. Outside that dir, gem can't be the source.
        if let Some(path) = cmd.path() {
            if let Some(bin_dir) = gem_bin_dir() {
                let prefix = format!("{}/", bin_dir.trim_end_matches('/'));
                if !path.starts_with(&prefix) {
                    return Ok(false);
                }
            }
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("gem")
            .arg("list")
            .arg("--no-versions")
            .output()
            .map_err(|e| format!("failed to run gem: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query gem for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        // Gem package name often differs from the binary name (e.g.
        // `bundler` ships `bundle`). The name match here will miss those.
        // For the common case where they match, this is correct.
        Ok(s.lines().any(|line| line.trim() == name))
    }
}

fn gem_bin_dir() -> Option<String> {
    let output = std::process::Command::new("gem")
        .arg("environment")
        .arg("gembindir")
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
