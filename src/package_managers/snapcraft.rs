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
        let snap_output = std::process::Command::new("snap")
            .arg("list")
            .arg(name)
            .output();

        match snap_output {
            Ok(output) => {
                if output.status.success() {
                    Ok(true)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if stderr.contains("error: no matching snaps installed") {
                        Ok(false)
                    } else {
                        Err(format!("Failed to find package using Snapcraft {}", name))
                    }
                }
            }
            Err(error) => Err(format!(
                "Failed to query snap for command {}: {}",
                name, error
            )),
        }
    }
}
