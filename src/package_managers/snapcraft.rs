use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct SnapCraftPackageManager;

impl PackageManager for SnapCraftPackageManager {
    fn name(&self) -> &str {
        "snapcraft"
    }

    fn is_installed(&self) -> bool {
        command_exists("snap")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // snap exposes commands via /snap/bin/. A path resolution settles it.
        if let Some(path) = cmd.path() {
            return Ok(path.starts_with("/snap/bin/"));
        }
        let name = cmd.lookup_name();
        let output = std::process::Command::new("snap")
            .arg("list")
            .arg(name)
            .output()
            .map_err(|e| format!("failed to execute snap: {}", e))?;

        if output.status.success() {
            return Ok(true);
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("error: no matching snaps installed") {
            return Ok(false);
        }
        Err(format!("Failed to find package using Snapcraft {}", name))
    }
}
