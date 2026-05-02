use crate::{
    command_resolver::command_exists,
    package_manager::{PackageManager, ResolvedCommand},
};

/// Commands that corepack manages as shims. Anything else can't have come
/// from corepack regardless of where it lives on disk.
const COREPACK_SHIMS: &[&str] = &["yarn", "yarnpkg", "pnpm", "pnpx"];

pub struct CorepackPackageManager;

impl PackageManager for CorepackPackageManager {
    fn name(&self) -> &str {
        "corepack"
    }

    fn is_installed(&self) -> bool {
        command_exists("corepack")
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        if !COREPACK_SHIMS.contains(&cmd.lookup_name()) {
            return Ok(false);
        }
        let path = match cmd.path() {
            Some(p) => p,
            None => return Ok(false),
        };

        // Newer corepack stores shims under $COREPACK_HOME (default
        // ~/.cache/node/corepack on Linux, ~/Library/Caches/node/corepack
        // on macOS).
        if let Some(home) = corepack_home() {
            if path.starts_with(&format!("{}/", home)) {
                return Ok(true);
            }
        }

        // Otherwise corepack installs shims as symlinks into the corepack
        // npm package directory, e.g.
        //   /usr/bin/yarn -> ../lib/node_modules/corepack/dist/yarn.js
        // The shim's *target basename* is `yarn.js`/`pnpm.js`/etc., not
        // `corepack` — what's invariant is that the resolved target lives
        // inside a directory called `corepack`.
        let p = std::path::Path::new(path);
        let target = match std::fs::read_link(p) {
            Ok(t) => t,
            Err(_) => return Ok(false),
        };
        let joined = match p.parent() {
            Some(parent) => parent.join(&target),
            None => target.clone(),
        };
        let resolved = std::fs::canonicalize(&joined).unwrap_or(joined);
        Ok(resolved
            .components()
            .any(|c| c.as_os_str() == std::ffi::OsStr::new("corepack")))
    }
}

fn corepack_home() -> Option<String> {
    if let Ok(dir) = std::env::var("COREPACK_HOME") {
        if !dir.is_empty() {
            return Some(dir);
        }
    }
    let home = std::env::var("HOME").ok()?;
    if cfg!(target_os = "macos") {
        Some(format!("{}/Library/Caches/node/corepack", home))
    } else {
        Some(format!("{}/.cache/node/corepack", home))
    }
}
