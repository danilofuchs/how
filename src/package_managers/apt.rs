use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct AptPackageManager;

impl PackageManager for AptPackageManager {
    fn name(&self) -> &str {
        "apt"
    }

    fn is_installed(&self) -> bool {
        command_exists("apt")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let stdout = run_capture("apt", &["list", "--installed"])?;
        // `apt list --installed` formats lines as `pkg/distro version arch [installed]`.
        Ok(listing_contains(
            &stdout,
            name,
            LineMatch::WordStart {
                terminators: &['/'],
            },
        ))
    }
}
