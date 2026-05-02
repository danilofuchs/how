use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct DnfPackageManager;

impl PackageManager for DnfPackageManager {
    fn name(&self) -> &str {
        "dnf"
    }

    fn is_installed(&self) -> bool {
        command_exists("dnf") || command_exists("rpm")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // When we have a concrete path, `rpm -qf` answers the ownership
        // question directly and far faster than listing all installed
        // packages.
        if let Some(path) = cmd.path() {
            if command_exists("rpm") {
                let output = std::process::Command::new("rpm")
                    .arg("-qf")
                    .arg("--queryformat")
                    .arg("%{NAME}\n")
                    .arg(path)
                    .output()
                    .map_err(|e| format!("failed to run rpm: {}", e))?;
                return Ok(output.status.success());
            }
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("dnf")
            .arg("list")
            .arg("--installed")
            .output()
            .map_err(|e| format!("failed to run dnf: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query dnf for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        // Lines look like "<name>.<arch>   <version>   <repo>".
        Ok(s.lines().any(|line| {
            line.split_whitespace()
                .next()
                .and_then(|first| first.split('.').next())
                .map(|pkg| pkg == name)
                .unwrap_or(false)
        }))
    }
}
