use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct PipxPackageManager;

impl PackageManager for PipxPackageManager {
    fn name(&self) -> &str {
        "pipx"
    }

    fn is_installed(&self) -> bool {
        command_exists("pipx")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let stdout = run_capture("pipx", &["list", "--short"])?;
        // `pipx list --short` outputs "<package> <version>" per line.
        // The package name usually matches the primary binary name.
        Ok(listing_contains(&stdout, name, LineMatch::FirstToken))
    }
}
