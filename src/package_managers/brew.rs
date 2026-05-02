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
