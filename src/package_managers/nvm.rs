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
        // nvm-managed binaries always live under $NVM_DIR/versions/node/. If
        // we don't have a concrete path, nvm can't be the source.
        let path = match cmd.path() {
            Some(p) => p,
            None => return Ok(false),
        };
        let prefix = match nvm_versions_dir() {
            Some(d) => format!("{}/", d),
            None => return Ok(false),
        };
        Ok(path.starts_with(&prefix))
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
