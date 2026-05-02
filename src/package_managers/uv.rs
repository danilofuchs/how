use crate::{
    command_resolver::command_exists,
    package_manager::{run_capture, PackageManager, ResolvedCommand},
};

pub struct UvPackageManager;

impl PackageManager for UvPackageManager {
    fn name(&self) -> &str {
        "uv"
    }

    fn is_installed(&self) -> bool {
        command_exists("uv")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let stdout = run_capture("uv", &["tool", "list"])?;
        // `uv tool list` emits a header line per tool ("pkg vX.Y.Z") followed
        // by "- <binary>" lines. Match either.
        Ok(stdout.lines().any(|line| {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("- ") {
                rest == name
            } else {
                trimmed
                    .split_whitespace()
                    .next()
                    .map(|pkg| pkg == name)
                    .unwrap_or(false)
            }
        }))
    }
}
