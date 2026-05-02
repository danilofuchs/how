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
        let output = std::process::Command::new("bun")
            .arg("pm")
            .arg("ls")
            .arg("-g")
            .output()
            .map_err(|e| format!("failed to run bun: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to query bun for command {}", name));
        }
        let s = String::from_utf8_lossy(&output.stdout);
        Ok(s.lines()
            .any(|line| line.contains(&format!(" {}@", name)) || line.ends_with(name)))
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
