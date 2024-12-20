use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct CargoPackageManager;

impl PackageManager for CargoPackageManager {
    fn name(&self) -> &str {
        "cargo"
    }

    fn is_installed(&self) -> bool {
        command_exists("npm")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let cargo_output = std::process::Command::new("cargo")
            .arg("install")
            .arg("--list")
            .output()
            .expect("Failed to execute cargo command");

        if cargo_output.status.success() {
            let output_str = String::from_utf8_lossy(&cargo_output.stdout);
            if output_str.lines().any(|line| line.starts_with(command)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!(
            "Failed to find package that installed command {}",
            command
        ))
    }
}
