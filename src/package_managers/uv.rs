use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
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
        let output = std::process::Command::new("uv")
            .arg("tool")
            .arg("list")
            .output()
            .map_err(|e| format!("failed to run uv: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query uv for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        // `uv tool list` emits a header line per tool ("pkg vX.Y.Z") followed
        // by "- <binary>" lines. Match either.
        Ok(s.lines().any(|line| {
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
