use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct MacPortsPackageManager;

impl PackageManager for MacPortsPackageManager {
    fn name(&self) -> &str {
        "macports"
    }

    fn is_installed(&self) -> bool {
        command_exists("port")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // MacPorts installs under /opt/local/. A path outside that prefix
        // can't be a port — short-circuit before the slow `port installed`.
        if let Some(path) = cmd.path() {
            if !path.starts_with("/opt/local/") {
                return Ok(false);
            }
        }

        let name = cmd.lookup_name();
        let output = std::process::Command::new("port")
            .arg("installed")
            .output()
            .map_err(|e| format!("failed to run port: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query port for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        // Each installed line looks like "  <name> @<version> (active)".
        Ok(s.lines().any(|line| {
            line.split_whitespace()
                .next()
                .map(|pkg| pkg == name)
                .unwrap_or(false)
        }))
    }
}
