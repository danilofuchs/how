use crate::command_resolver::Resolution;

pub struct ResolvedCommand<'a> {
    pub command: &'a str,
    pub resolution: &'a Resolution,
}

impl<'a> ResolvedCommand<'a> {
    pub fn lookup_name(&self) -> &str {
        match self.resolution {
            Resolution::Alias(target) => target,
            _ => self.command,
        }
    }
}

pub trait PackageManager {
    fn name(&self) -> &str;
    fn is_installed(&self) -> bool;
    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String>;
}
