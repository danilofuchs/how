use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct Pip3PackageManager;

impl PackageManager for Pip3PackageManager {
    fn name(&self) -> &str {
        "pip3"
    }

    fn is_installed(&self) -> bool {
        command_exists("pip3")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let pip_output = std::process::Command::new("pip3")
            .arg("list")
            .output()
            .expect("Failed to execute pip3 command");

        if pip_output.status.success() {
            let output_str = String::from_utf8_lossy(&pip_output.stdout);
            if output_str.lines().any(|line| line.starts_with(command)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query pip3 for command {}", command))
    }
}
