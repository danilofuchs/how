use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct PnpmPackageManager;

impl PackageManager for PnpmPackageManager {
    fn name(&self) -> &str {
        "pnpm"
    }

    fn is_installed(&self) -> bool {
        command_exists("pnpm")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // pnpm exposes globally-installed binaries (and the pnpm-managed
        // node itself) as a linkfarm directly under $PNPM_HOME (default
        // ~/.local/share/pnpm). A path under that prefix is conclusive,
        // and catches the node binary that `pnpm list -g` doesn't report.
        if let Some(path) = cmd.path() {
            if let Some(home) = pnpm_home() {
                let prefix = format!("{}/", home.trim_end_matches('/'));
                return Ok(path.starts_with(&prefix));
            }
        }

        let name = cmd.lookup_name();
        let stdout = run_capture("pnpm", &["list", "--global", "--depth=0", "--parseable"])?;
        // `--parseable` prints absolute paths whose final component is the
        // package name; match by file_name to avoid `foo` matching `barfoo`.
        Ok(listing_contains(&stdout, name, LineMatch::PathTail))
    }
}

fn pnpm_home() -> Option<String> {
    if let Ok(dir) = std::env::var("PNPM_HOME") {
        if !dir.is_empty() {
            return Some(dir);
        }
    }
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.local/share/pnpm", h))
}
