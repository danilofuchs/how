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

pub trait PackageManager: Sync {
    fn name(&self) -> &str;
    fn is_installed(&self) -> bool;
    fn is_command_installed(&self, cmd: &ResolvedCommand) -> Result<bool, String>;
}

/// How a candidate `name` should be matched against a line from a
/// package-manager listing. Most managers print one package per line;
/// the rules differ in what surrounds the name.
#[derive(Clone, Copy)]
pub enum LineMatch {
    /// The name is a whole word at the start of the line, optionally followed
    /// by a separator from `terminators` (e.g. ` `, `/`, `\t`, `@`, `:`).
    /// Avoids `git` matching `gitleaks`.
    WordStart { terminators: &'static [char] },
    /// The name is the final path component of the line (e.g. `npm list
    /// --parseable` prints `…/lib/node_modules/<name>`).
    PathTail,
    /// The (trimmed) line equals the name. For listings that print one
    /// bare package per line (e.g. `gem list --no-versions`, `pacman -Qq`).
    ExactLine,
    /// The first whitespace-delimited token on the line equals the name.
    /// For columnar listings like `port installed` or `pipx list --short`.
    FirstToken,
}

pub fn listing_contains(output: &str, name: &str, mode: LineMatch) -> bool {
    output.lines().any(|line| match mode {
        LineMatch::WordStart { terminators } => match line.strip_prefix(name) {
            Some("") => true,
            Some(rest) => rest.starts_with(terminators),
            None => false,
        },
        LineMatch::PathTail => std::path::Path::new(line)
            .file_name()
            .map(|f| f == std::ffi::OsStr::new(name))
            .unwrap_or(false),
        LineMatch::ExactLine => line.trim() == name,
        LineMatch::FirstToken => line.split_whitespace().next() == Some(name),
    })
}

/// Run a subprocess and return its stdout, propagating spawn failures and
/// non-zero exits as `Err(String)` instead of panicking.
pub fn run_capture(bin: &str, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| format!("failed to execute {}: {}", bin, e))?;
    if !output.status.success() {
        return Err(format!(
            "{} {} exited with {}",
            bin,
            args.join(" "),
            output.status
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
