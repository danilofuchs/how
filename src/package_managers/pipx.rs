use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct PipxPackageManager;

impl PackageManager for PipxPackageManager {
    fn name(&self) -> &str {
        "pipx"
    }

    fn is_installed(&self) -> bool {
        command_exists("pipx")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let output = std::process::Command::new("pipx")
            .arg("list")
            .arg("--short")
            .output()
            .map_err(|e| format!("failed to run pipx: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query pipx for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        // `pipx list --short` outputs "<package> <version>" per line.
        // The package name usually matches the primary binary name.
        Ok(s.lines().any(|line| {
            line.split_whitespace()
                .next()
                .map(|pkg| pkg == name)
                .unwrap_or(false)
        }))
    }
}
