use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct NpmPackageManager;

impl PackageManager for NpmPackageManager {
    fn name(&self) -> &str {
        "npm"
    }

    fn is_installed(&self) -> bool {
        command_exists("npm")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let npm_output = std::process::Command::new("npm")
            .arg("list")
            .arg("--global")
            .arg("--depth=0")
            .arg("--parseable")
            .output()
            .expect("Failed to execute npm command");

        if npm_output.status.success() {
            let output_str = String::from_utf8_lossy(&npm_output.stdout);
            if output_str.lines().any(|line| line.ends_with(command)) {
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
