use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct AptPackageManager;

impl PackageManager for AptPackageManager {
    fn name(&self) -> &str {
        "apt"
    }

    fn is_installed(&self) -> bool {
        command_exists("apt")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let apt_output = std::process::Command::new("apt")
            .arg("list")
            .arg("--installed")
            .output()
            .expect("Failed to execute apt command");

        if apt_output.status.success() {
            let output_str = String::from_utf8_lossy(&apt_output.stdout);
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
