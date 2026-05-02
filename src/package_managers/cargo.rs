use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

pub struct CargoPackageManager;

impl PackageManager for CargoPackageManager {
    fn name(&self) -> &str {
        "cargo"
    }

    fn is_installed(&self) -> bool {
        command_exists("npm")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // `cargo install` always lands in $CARGO_HOME/bin. A path outside
        // that dir can't be a cargo install — short-circuit. We don't claim
        // the inverse: rustup-managed binaries (cargo, rustc, rustup) also
        // live there, so the slow `cargo install --list` is still the truth.
        if let Some(path) = cmd.path() {
            if let Some(bin_dir) = cargo_bin_dir() {
                let bin_prefix = format!("{}/", bin_dir);
                if !path.starts_with(&bin_prefix) {
                    return Ok(false);
                }
            }
        }
        let name = cmd.lookup_name();
        let cargo_output = std::process::Command::new("cargo")
            .arg("install")
            .arg("--list")
            .output()
            .expect("Failed to execute cargo command");

        if cargo_output.status.success() {
            let output_str = String::from_utf8_lossy(&cargo_output.stdout);
            if output_str.lines().any(|line| line.starts_with(name)) {
                return Ok(true);
            }
            return Ok(false);
        }

        Err(format!("Failed to query cargo for command {}", name))
    }
}

fn cargo_bin_dir() -> Option<String> {
    if let Ok(home) = std::env::var("CARGO_HOME") {
        return Some(format!("{}/bin", home));
    }
    std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.cargo/bin", h))
}
