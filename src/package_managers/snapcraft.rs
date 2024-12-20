use crate::{command_exists::command_exists, package_manager::PackageManager};

pub struct SnapCraftPackageManager;

impl PackageManager for SnapCraftPackageManager {
    fn name(&self) -> &str {
        "snapcraft"
    }

    fn is_installed(&self) -> bool {
        command_exists("snap")
    }

    fn is_command_installed(&self, command: &str) -> Result<bool, String> {
        let snap_output = std::process::Command::new("snap")
            .arg("list")
            .arg(command)
            .output();

        match snap_output {
            Ok(output) => {
                if output.status.success() {
                    return Ok(true);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if stderr.contains("error: no matching snaps installed") {
                        return Ok(false);
                    }

                    return Err(format!(
                        "Failed to find package using Snapcraft {}",
                        command
                    ));
                }
            }
            Err(error) => {
                return Err(format!(
                    "Failed to find package using Snapcraft {}: {}",
                    command, error
                ));
            }
        }
    }
}
