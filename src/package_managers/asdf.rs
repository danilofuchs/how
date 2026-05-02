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
        // asdf-managed commands always go through a shim under its data dir.
        // The path tells us conclusively whether asdf is the source.
        if let Some(path) = cmd.path() {
            if let Some(shims) = asdf_shims_dir() {
                let prefix = format!("{}/", shims);
                return Ok(path.starts_with(&prefix));
            }
        }
        let name = cmd.lookup_name();
        let asdf_output = std::process::Command::new("asdf")
            .arg("plugin")
            .arg("list")
            .output()
            .expect("Failed to execute asdf command");

        if asdf_output.status.success() {
            let output_str = String::from_utf8_lossy(&asdf_output.stdout);
            if output_str.lines().any(|line| line.starts_with(name)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query asdf for command {}", name))
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
