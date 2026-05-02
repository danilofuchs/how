use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct BunPackageManager;

impl PackageManager for BunPackageManager {
    fn name(&self) -> &str {
        "bun"
    }

    fn is_installed(&self) -> bool {
        command_exists("bun")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Bun symlinks globally-installed binaries into $BUN_INSTALL/bin
        // (default ~/.bun/bin). A path-based check settles it.
        let bin_dir = bun_bin_dir();

        if let Some(path) = cmd.path() {
            if let Some(dir) = &bin_dir {
                let prefix = format!("{}/", dir);
                return Ok(path.starts_with(&prefix));
            }
            return Ok(false);
        }

        let name = cmd.lookup_name();
        // `bun pm ls -g` exits non-zero when nothing is globally installed —
        // not a real failure for our purposes. Treat any non-success as
        // "not found" instead of surfacing it as an error.
        let output = std::process::Command::new("bun")
            .args(["pm", "ls", "-g"])
            .output()
            .map_err(|e| format!("failed to run bun: {}", e))?;
        if !output.status.success() {
            return Ok(false);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Lines look like `├── pkg@1.2.3`. Match the package name as a
        // whole token followed by `@`.
        let needle = format!(" {}@", name);
        Ok(stdout.lines().any(|line| line.contains(&needle)))
    }
}

fn bun_bin_dir() -> Option<String> {
    if let Ok(dir) = std::env::var("BUN_INSTALL") {
        if !dir.is_empty() {
            return Some(format!("{}/bin", dir));
        }
    }
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.bun/bin", h))
}
