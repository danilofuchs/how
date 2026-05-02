use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct PacmanPackageManager;

impl PackageManager for PacmanPackageManager {
    fn name(&self) -> &str {
        "pacman"
    }

    fn is_installed(&self) -> bool {
        command_exists("pacman")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // `pacman -Qo <path>` returns 0 iff the file is owned by an
        // installed package. This is the fastest, most accurate check.
        if let Some(path) = cmd.path() {
            let output = std::process::Command::new("pacman")
                .arg("-Qo")
                .arg(path)
                .output()
                .map_err(|e| format!("failed to run pacman: {}", e))?;
            return Ok(output.status.success());
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("pacman")
            .arg("-Qq")
            .output()
            .map_err(|e| format!("failed to run pacman: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query pacman for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        Ok(s.lines().any(|line| line == name))
    }
}
