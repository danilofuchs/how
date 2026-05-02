use crate::{
    command_resolver::command_exists,
    package_manager::{listing_contains, run_capture, LineMatch, PackageManager, ResolvedCommand},
};

pub struct PipPackageManager {
    pub bin: &'static str,
}

impl PackageManager for PipPackageManager {
    fn name(&self) -> &str {
        self.bin
    }

    fn is_installed(&self) -> bool {
        command_exists(self.bin)
    }

    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String> {
        let name = cmd.lookup_name();
        let stdout = run_capture(self.bin, &["list"])?;
        // `pip list` columns: `Package    Version`. Whitespace-terminated.
        Ok(listing_contains(
            &stdout,
            name,
            LineMatch::WordStart {
                terminators: &[' ', '\t'],
            },
        ))
    }
}
