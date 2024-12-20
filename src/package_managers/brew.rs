use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct BrewPackageManager;

impl PackageManager for BrewPackageManager {
    fn name(&self) -> &str {
        "brew"
    }

    fn is_installed(&self) -> bool {
        command_exists("brew")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let brew_output = std::process::Command::new("brew")
            .arg("list")
            .arg("--installed-on-request")
            .arg("-1")
            .output()
            .expect("Failed to execute brew command");

        if brew_output.status.success() {
            let output_str = String::from_utf8_lossy(&brew_output.stdout);
            if output_str.lines().any(|line| line.starts_with(command)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query brew for command {}", command))
    }
}
