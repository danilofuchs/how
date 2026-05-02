use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
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
        let stdout = run_capture("port", &["installed"])?;
        // Each installed line looks like "  <name> @<version> (active)".
        Ok(listing_contains(&stdout, name, LineMatch::FirstToken))
    }
}
