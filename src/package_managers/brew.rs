use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct BrewPackageManager;

impl PackageManager for BrewPackageManager {
    fn name(&self) -> &str {
        "brew"
    }

    fn is_installed(&self) -> bool {
        command_exists("brew")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Brew installs under one of these prefixes. If the command resolves
        // to a concrete path outside all of them, brew can't be the source —
        // skip the slow `brew list` call entirely.
        if let Some(path) = cmd.path() {
            const BREW_PREFIXES: &[&str] = &[
                "/opt/homebrew/",
                "/usr/local/",
                "/home/linuxbrew/.linuxbrew/",
            ];
            if !BREW_PREFIXES.iter().any(|p| path.starts_with(p)) {
                return Ok(false);
            }
        }
        let name = cmd.lookup_name();
        let brew_output = std::process::Command::new("brew")
            .arg("list")
            .arg("--installed-on-request")
            .arg("-1")
            .output()
            .expect("Failed to execute brew command");

        if brew_output.status.success() {
            let output_str = String::from_utf8_lossy(&brew_output.stdout);
            if output_str.lines().any(|line| line.starts_with(name)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query brew for command {}", name))
    }
}
