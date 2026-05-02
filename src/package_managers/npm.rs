use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct NpmPackageManager;

impl PackageManager for NpmPackageManager {
    fn name(&self) -> &str {
        "npm"
    }

    fn is_installed(&self) -> bool {
        command_exists("npm")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let stdout = run_capture("npm", &["list", "--global", "--depth=0", "--parseable"])?;
        // `--parseable` prints one absolute path per line; the package is the
        // final path component. Match by file_name to avoid `foo` matching
        // `barfoo`.
        Ok(listing_contains(&stdout, name, LineMatch::PathTail))
    }
}
