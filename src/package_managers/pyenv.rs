use crate::package_manager::{PackageManager, ResolvedCommand};

pub struct PyenvPackageManager;

impl PackageManager for PyenvPackageManager {
    fn name(&self) -> &str {
        "pyenv"
    }

    fn is_installed(&self) -> bool {
        match pyenv_root() {
            Some(d) => std::path::Path::new(&d).is_dir(),
            None => false,
        }
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let root = match pyenv_root() {
            Some(d) => d,
            None => return Ok(false),
        };
        let prefix = format!("{}/", root);

        if let Some(path) = cmd.path() {
            return Ok(path.starts_with(&prefix));
        }

        // Fall back to checking the shim dir for the command name.
        let name = cmd.lookup_name();
        Ok(std::path::Path::new(&format!("{}shims/{}", prefix, name)).exists())
    }
}

fn pyenv_root() -> Option<String> {
    if let Ok(dir) = std::env::var("PYENV_ROOT") {
        if !dir.is_empty() {
            return Some(dir);
        }
    }
    std::env::var("HOME").ok().map(|h| format!("{}/.pyenv", h))
}
