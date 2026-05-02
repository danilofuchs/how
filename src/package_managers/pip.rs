use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct PipPackageManager;

impl PackageManager for PipPackageManager {
    fn name(&self) -> &str {
        "pip"
    }

    fn is_installed(&self) -> bool {
        command_exists("pip")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let pip_output = std::process::Command::new("pip")
            .arg("list")
            .output()
            .expect("Failed to execute pip command");

        if pip_output.status.success() {
            let output_str = String::from_utf8_lossy(&pip_output.stdout);
            if output_str.lines().any(|line| line.starts_with(name)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query pip for command {}", name))
    }
}
