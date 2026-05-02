use crate::package_manager::{PackageManager, ResolvedCommand};

pub struct GoPackageManager;

impl PackageManager for GoPackageManager {
    fn name(&self) -> &str {
        "go"
    }

    fn is_installed(&self) -> bool {
        match go_bin_dir() {
            Some(d) => std::path::Path::new(&d).is_dir(),
            None => false,
        }
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        // `go install` only ever drops binaries into $GOBIN (or
        // $GOPATH/bin, default ~/go/bin). No listing command exists, so
        // path containment is the authoritative check.
        let bin_dir = match go_bin_dir() {
            Some(d) => d,
            None => return Ok(false),
        };
        let prefix = format!("{}/", bin_dir);

        if let Some(path) = cmd.path() {
            return Ok(path.starts_with(&prefix));
        }

        let name = cmd.lookup_name();
        Ok(std::path::Path::new(&format!("{}{}", prefix, name)).exists())
    }
}

fn go_bin_dir() -> Option<String> {
    if let Ok(dir) = std::env::var("GOBIN") {
        if !dir.is_empty() {
            return Some(dir);
        }
    }
    if let Ok(gopath) = std::env::var("GOPATH") {
        if !gopath.is_empty() {
            // GOPATH may be colon-separated; the first entry wins for `go install`.
            let first = gopath.split(':').next().unwrap_or(&gopath);
            return Some(format!("{}/bin", first));
        }
    }
    std::env::var("HOME").ok().map(|h| format!("{}/go/bin", h))
}
