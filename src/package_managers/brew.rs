use crate::package_manager::PackageManager;

pub struct BrewPackageManager;

impl PackageManager for BrewPackageManager {
    fn name(&self) -> &str {
        "brew"
    }

    fn is_installed(&self, command: &str) -> Result<bool, String> {
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

        Err(format!(
            "Failed to find package that installed command {}",
            command
        ))
    }
}
