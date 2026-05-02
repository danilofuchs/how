use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct PnpmPackageManager;

impl PackageManager for PnpmPackageManager {
    fn name(&self) -> &str {
        "pnpm"
    }

    fn is_installed(&self) -> bool {
        command_exists("pnpm")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // pnpm exposes globally-installed binaries (and the pnpm-managed
        // node itself) as a linkfarm directly under $PNPM_HOME (default
        // ~/.local/share/pnpm). A path under that prefix is conclusive,
        // and catches the node binary that `pnpm list -g` doesn't report.
        if let Some(path) = cmd.path() {
            if let Some(home) = pnpm_home() {
                let prefix = format!("{}/", home.trim_end_matches('/'));
                if path.starts_with(&prefix) {
                    return Ok(true);
                }
                return Ok(false);
            }
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("pnpm")
            .arg("list")
            .arg("--global")
            .arg("--depth=0")
            .arg("--parseable")
            .output()
            .map_err(|e| format!("failed to run pnpm: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query pnpm for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        Ok(s.lines().any(|line| line.ends_with(name)))
    }
}

fn pnpm_home() -> Option<String> {
    if let Ok(dir) = std::env::var("PNPM_HOME") {
        if !dir.is_empty() {
            return Some(dir);
        }
    }
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.local/share/pnpm", h))
}
