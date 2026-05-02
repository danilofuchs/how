use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct BrewPackageManager;

const BREW_PREFIXES: &[&str] = &[
    "/opt/homebrew/",
    "/usr/local/",
    "/home/linuxbrew/.linuxbrew/",
];

impl PackageManager for BrewPackageManager {
    fn name(&self) -> &str {
        "brew"
    }

    fn is_installed(&self) -> bool {
        command_exists("brew")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Brew installs under one of these prefixes. If the command resolves
        // to a concrete path outside all of them, brew can't be the source.
        if let Some(path) = cmd.path() {
            if !BREW_PREFIXES.iter().any(|p| path.starts_with(p)) {
                return Ok(false);
            }
        }
        let name = cmd.lookup_name();
        // `brew list -1` includes formulae installed as dependencies; we want
        // those too — `--installed-on-request` would miss commands brought in
        // transitively.
        let stdout = run_capture("brew", &["list", "-1"])?;
        Ok(listing_contains(
            &stdout,
            name,
            LineMatch::WordStart { terminators: &[] },
        ))
    }
}
