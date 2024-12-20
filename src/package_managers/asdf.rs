use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct AsdfPackageManager;

impl PackageManager for AsdfPackageManager {
    fn name(&self) -> &str {
        "asdf"
    }

    fn is_installed(&self) -> bool {
        command_exists("asdf")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let asdf_output = std::process::Command::new("asdf")
            .arg("plugin")
            .arg("list")
            .output()
            .expect("Failed to execute asdf command");

        if asdf_output.status.success() {
            let output_str = String::from_utf8_lossy(&asdf_output.stdout);
            if output_str.lines().any(|line| line.starts_with(command)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query asdf for command {}", command))
    }
}
