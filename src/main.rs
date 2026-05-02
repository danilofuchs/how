mod package_manager;
mod package_managers {
    pub mod apt;
    pub mod asdf;
    pub mod brew;
    pub mod cargo;
    pub mod npm;
    pub mod pip;
    pub mod pip3;
    pub mod snapcraft;
}
use crate::package_managers::{
    apt::AptPackageManager, asdf::AsdfPackageManager, brew::BrewPackageManager,
    cargo::CargoPackageManager, npm::NpmPackageManager, pip::PipPackageManager,
    pip3::Pip3PackageManager, snapcraft::SnapCraftPackageManager,
};
use package_manager::{PackageManager, ResolvedCommand};

mod command_resolver;
use crate::command_resolver::{resolve, CommandResolution};

use std::process::exit;

use clap::Parser;
use futures::future::join_all;
use tokio::task;

/// Finds the package manager that installed a command
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the command to search
    command: String,
}

fn all_package_managers() -> Vec<Box<dyn PackageManager + Send>> {
    vec![
        Box::new(AptPackageManager),
        Box::new(NpmPackageManager),
        Box::new(BrewPackageManager),
        Box::new(CargoPackageManager),
        Box::new(SnapCraftPackageManager),
        Box::new(PipPackageManager),
        Box::new(Pip3PackageManager),
        Box::new(AsdfPackageManager),
        // TODO: Add other package managers here
    ]
}

/// Reports the resolution to stdout/stderr and returns the resolution to
/// hand to the package managers, plus whether `type` resolved to anything.
fn report_resolution(
    command: &str,
    resolution: Result<CommandResolution, String>,
) -> (CommandResolution, bool) {
    match resolution {
        Ok(r @ CommandResolution::Path(_)) => {
            if let CommandResolution::Path(p) = &r {
                println!("{} resolves to {}", command, p);
            }
            (r, true)
        }
        Ok(r @ CommandResolution::Alias(_)) => {
            if let CommandResolution::Alias(target) = &r {
                println!("{} is an alias for {}", command, target);
            }
            (r, true)
        }
        Ok(CommandResolution::Function) => {
            println!("{} is a shell function", command);
            (CommandResolution::Function, true)
        }
        Ok(CommandResolution::Builtin) => {
            println!("{} is a shell builtin", command);
            (CommandResolution::Builtin, true)
        }
        Ok(CommandResolution::Keyword) => {
            println!("{} is a shell reserved word", command);
            (CommandResolution::Keyword, true)
        }
        Ok(CommandResolution::NotFound) => (CommandResolution::NotFound, false),
        Err(e) => {
            eprintln!("type resolver error: {}", e);
            (CommandResolution::NotFound, false)
        }
    }
}

async fn find_installers(
    package_managers: Vec<Box<dyn PackageManager + Send>>,
    command: &str,
    resolution: &CommandResolution,
) -> Vec<String> {
    let tasks = package_managers.into_iter().map(|manager| {
        let command = command.to_string();
        let resolution = resolution.clone();
        task::spawn(async move {
            if !manager.is_installed() {
                return None;
            }
            let cmd = ResolvedCommand {
                command: &command,
                resolution: &resolution,
            };
            match manager.is_command_installed(&cmd) {
                Ok(true) => Some(manager.name().to_string()),
                Ok(false) => None,
                Err(e) => {
                    eprintln!("{} Error: {}", manager.name(), e);
                    None
                }
            }
        })
    });

    join_all(tasks)
        .await
        .into_iter()
        .filter_map(|r| r.ok().flatten())
        .collect()
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let command = &args.command;

    let resolution = resolve(command);
    let (resolution, type_found) = report_resolution(command, resolution);

    // Shell builtins, keywords, and functions can't have come from a package
    // manager — skip the per-manager queries entirely.
    let cmd = ResolvedCommand {
        command,
        resolution: &resolution,
    };
    let installers = if cmd.is_shell_internal() {
        Vec::new()
    } else {
        find_installers(all_package_managers(), command, &resolution).await
    };

    let lookup = match &resolution {
        CommandResolution::Alias(target) => target.as_str(),
        _ => command.as_str(),
    };
    for manager_name in &installers {
        println!("{} installed by {}", lookup, manager_name);
    }

    if installers.is_empty() && !type_found {
        eprintln!("Failed to find package that installed command {}", command);
        exit(1)
    }
}
