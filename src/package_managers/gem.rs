use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct GemPackageManager;

impl PackageManager for GemPackageManager {
    fn name(&self) -> &str {
        "gem"
    }

    fn is_installed(&self) -> bool {
        command_exists("gem")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Path shortcut: gem-installed executables live under
        // `gem env gembindir`. Outside that dir, gem can't be the source.
        if let Some(path) = cmd.path() {
            if let Some(bin_dir) = gem_bin_dir() {
                let prefix = format!("{}/", bin_dir.trim_end_matches('/'));
                if !path.starts_with(&prefix) {
                    return Ok(false);
                }
            }
        }

        let name = cmd.lookup_name();
        let stdout = run_capture("gem", &["list", "--no-versions"])?;
        // Gem package name often differs from the binary name (e.g.
        // `bundler` ships `bundle`). The name match here will miss those.
        // For the common case where they match, this is correct.
        Ok(listing_contains(&stdout, name, LineMatch::ExactLine))
    }
}

fn gem_bin_dir() -> Option<String> {
    let stdout = run_capture("gem", &["environment", "gembindir"]).ok()?;
    let s = stdout.trim();
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
