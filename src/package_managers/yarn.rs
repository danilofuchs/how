use crate::{
    command_resolver::command_exists,
    package_manager::{run_capture, PackageManager, ResolvedCommand},
};

pub struct YarnPackageManager;

impl PackageManager for YarnPackageManager {
    fn name(&self) -> &str {
        "yarn"
    }

    fn is_installed(&self) -> bool {
        command_exists("yarn")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // Yarn Classic exposes globally-installed binaries under `yarn global
        // bin`. When we have a path, that prefix is authoritative.
        if let Some(path) = cmd.path() {
            if let Some(bin_dir) = yarn_global_bin() {
                let prefix = format!("{}/", bin_dir.trim_end_matches('/'));
                return Ok(path.starts_with(&prefix));
            }
        }

        let name = cmd.lookup_name();
        let stdout = run_capture("yarn", &["global", "list", "--depth=0"])?;
        // Lines look like: info "pkg@1.2.3" has binaries: ...
        // Match the quoted package name as a whole token.
        let needle = format!("\"{}@", name);
        Ok(stdout.lines().any(|line| line.contains(&needle)))
    }
}

fn yarn_global_bin() -> Option<String> {
    let stdout = run_capture("yarn", &["global", "bin"]).ok()?;
    let s = stdout.trim();
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
