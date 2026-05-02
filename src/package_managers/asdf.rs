use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct AsdfPackageManager;

impl PackageManager for AsdfPackageManager {
    fn name(&self) -> &str {
        "asdf"
    }

    fn is_installed(&self) -> bool {
        command_exists("asdf")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // asdf-managed commands always resolve through a shim under its data
        // dir. If we don't have a path (alias / shell-internal / not found),
        // asdf can't be the source.
        let path = match cmd.path() {
            Some(p) => p,
            None => return Ok(false),
        };
        let shims = match asdf_shims_dir() {
            Some(d) => d,
            None => return Ok(false),
        };
        Ok(path.starts_with(&format!("{}/", shims)))
    }
}

fn asdf_shims_dir() -> Option<String> {
    if let Ok(dir) = std::env::var("ASDF_DATA_DIR") {
        return Some(format!("{}/shims", dir));
    }
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.asdf/shims", h))
}
