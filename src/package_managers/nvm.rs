use std::process::Command;

use crate::package_manager::{PackageManager, ResolvedCommand};

pub struct NvmPackageManager;

impl PackageManager for NvmPackageManager {
    fn name(&self) -> &str {
        "nvm"
    }

    fn is_installed(&self) -> bool {
        match nvm_versions_dir() {
            Some(d) => std::path::Path::new(&d).is_dir(),
            None => false,
        }
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let prefix = match nvm_versions_dir() {
            Some(d) => format!("{}/", d),
            None => return Ok(false),
        };

        if let Some(path) = cmd.path() {
            return Ok(path.starts_with(&prefix));
        }

        let name = cmd.lookup_name();
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("command -v -- {}", name))
            .output()
            .map_err(|e| format!("failed to run command -v: {}", e))?;

        if !output.status.success() {
            return Ok(false);
        }
        let resolved = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(resolved.starts_with(&prefix))
    }
}

fn nvm_versions_dir() -> Option<String> {
    let base = if let Ok(dir) = std::env::var("NVM_DIR") {
        if dir.is_empty() {
            return std::env::var("HOME").ok().map(|h| format!("{}/.nvm", h));
        }
        dir
    } else {
        format!("{}/.nvm", std::env::var("HOME").ok()?)
    };
    Some(format!("{}/versions/node", base))
}
