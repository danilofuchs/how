use crate::command_resolver::CommandResolution;

pub struct ResolvedCommand<'a> {
    pub command: &'a str,
    pub resolution: &'a CommandResolution,
}

impl<'a> ResolvedCommand<'a> {
    pub fn lookup_name(&self) -> &str {
        match self.resolution {
            CommandResolution::Alias(target) => target,
            _ => self.command,
        }
    }

    pub fn path(&self) -> Option<&str> {
        if let CommandResolution::Path(p) = self.resolution {
            Some(p)
        } else {
            None
        }
    }

    /// True for shell builtins, reserved words, or functions — things no
    /// package manager could have installed.
    pub fn is_shell_internal(&self) -> bool {
        matches!(
            self.resolution,
            CommandResolution::Builtin | CommandResolution::Keyword | CommandResolution::Function
        )
    }
}

pub trait PackageManager {
    fn name(&self) -> &str;
    fn is_installed(&self) -> bool;
    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String>;
}
