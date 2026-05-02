use crate::package_manager::{PackageManager, ResolvedCommand};

pub struct RbenvPackageManager;

impl PackageManager for RbenvPackageManager {
    fn name(&self) -> &str {
        "rbenv"
    }

    fn is_installed(&self) -> bool {
        match rbenv_root() {
            Some(d) => std::path::Path::new(&d).is_dir(),
            None => false,
        }
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let root = match rbenv_root() {
            Some(d) => d,
            None => return Ok(false),
        };
        let prefix = format!("{}/", root);

        if let Some(path) = cmd.path() {
            return Ok(path.starts_with(&prefix));
        }

        let name = cmd.lookup_name();
        Ok(std::path::Path::new(&format!("{}shims/{}", prefix, name)).exists())
    }
}

fn rbenv_root() -> Option<String> {
    if let Ok(dir) = std::env::var("RBENV_ROOT") {
        if !dir.is_empty() {
            return Some(dir);
        }
    }
    std::env::var("HOME").ok().map(|h| format!("{}/.rbenv", h))
}
