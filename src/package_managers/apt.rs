use crate::package_manager::PackageManager;

pub struct AptPackageManager;

impl PackageManager for AptPackageManager {
    fn name(&self) -> &str {
        "apt"
    }

    fn is_installed(&self, command: &str) -> Result<bool, String> {
        let which_output = std::process::Command::new("which")
            .arg(command)
            .output()
            .expect("Failed to execute which command");

        if !which_output.status.success() {
            return Err(format!("Failed to find command {} in PATH", command));
        }

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
        }

        Err(format!(
            "Failed to find package that installed command {}",
            command
        ))
    }
}
