use std::env;
use std::process::Command;

#[derive(Clone)]
pub enum CommandResolution {
    Path(String),
    Alias(String),
    Function,
    Builtin,
    Keyword,
    NotFound,
}

fn is_safe_command_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| {
            c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '+' | ':' | '@' | '/' | '-')
        })
}

/// Use the user's interactive shell so aliases/functions defined in their rc
/// files are visible. Falls back to `bash` then `sh` if `$SHELL` is unset —
/// `sh` on Debian is dash, whose `type` builtin doesn't accept `--` and
/// exits 127, which would make every lookup return `NotFound`.
fn user_shell() -> String {
    if let Ok(shell) = env::var("SHELL") {
        if !shell.is_empty() {
            return shell;
        }
    }
    if Command::new("bash").arg("-c").arg(":").output().is_ok() {
        return "bash".to_string();
    }
    "sh".to_string()
}

pub fn resolve(command: &str) -> Result<CommandResolution, String> {
    if !is_safe_command_name(command) {
        return Ok(CommandResolution::NotFound);
    }

    let shell = user_shell();
    let output = Command::new(&shell)
        .arg("-ic")
        .arg(format!("type -- {}", command))
        .output()
        .map_err(|e| format!("failed to execute '{} -ic type': {}", shell, e))?;

    if !output.status.success() {
        return Ok(CommandResolution::NotFound);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or("").trim();
    if line.is_empty() {
        return Ok(CommandResolution::NotFound);
    }

    parse_type_output(command, line)
}

fn parse_type_output(command: &str, line: &str) -> Result<CommandResolution, String> {
    // bash/sh: "<cmd> is /path"  /  "<cmd> is aliased to '<expansion>'"
    // zsh:     "<cmd> is /path"  /  "<cmd> is an alias for <expansion>"
    //          "<cmd>: aliased to <expansion>"  (some sh variants)
    let prefix = format!("{} is ", command);
    let prefix_colon = format!("{}: ", command);
    let rest_owned = if let Some(r) = line.strip_prefix(&prefix) {
        r.to_string()
    } else if let Some(r) = line.strip_prefix(&prefix_colon) {
        format!("aliased to {}", r.strip_prefix("aliased to ").unwrap_or(r))
    } else {
        return Err(format!("unrecognized 'type' output: {}", line));
    };
    let rest = rest_owned.as_str();

    if rest.starts_with('/') {
        return Ok(CommandResolution::Path(rest.to_string()));
    }
    let alias_expansion = rest
        .strip_prefix("aliased to ")
        .or_else(|| rest.strip_prefix("an alias for "));
    if let Some(expansion) = alias_expansion {
        let trimmed = expansion.trim_matches(|c: char| c == '\'' || c == '`' || c == '"');
        let target = trimmed.split_whitespace().next().unwrap_or("").to_string();
        if target.is_empty() {
            return Err(format!("could not parse alias expansion: {}", line));
        }
        return Ok(CommandResolution::Alias(target));
    }
    if rest == "a shell builtin" || rest == "a special shell builtin" {
        return Ok(CommandResolution::Builtin);
    }
    if rest == "a function"
        || rest == "a shell function"
        || rest.starts_with("a function")
        || rest.starts_with("a shell function")
    {
        return Ok(CommandResolution::Function);
    }
    if rest == "a shell keyword" || rest == "a reserved word" {
        return Ok(CommandResolution::Keyword);
    }
    if rest.starts_with("hashed") {
        if let (Some(open), Some(close)) = (rest.find('('), rest.rfind(')')) {
            if open + 1 < close {
                return Ok(CommandResolution::Path(rest[open + 1..close].to_string()));
            }
        }
    }

    Err(format!("unrecognized 'type' output: {}", line))
}

/// Fast PATH-only lookup. Used to check whether tools like `brew` or `apt` are
/// available — we deliberately don't source the user's rc files here, both
/// because it would be slow and because aliases don't matter for binaries.
pub fn command_exists(command: &str) -> bool {
    if !is_safe_command_name(command) {
        return false;
    }
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v -- {}", command))
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
