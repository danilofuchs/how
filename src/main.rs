mod package_manager;
mod package_managers {
    pub mod apt;
    pub mod asdf;
    pub mod brew;
    pub mod bun;
    pub mod cargo;
    pub mod dnf;
    pub mod gem;
    pub mod go;
    pub mod macports;
    pub mod mise;
    pub mod npm;
    pub mod nvm;
    pub mod pacman;
    pub mod pip;
    pub mod pipx;
    pub mod pnpm;
    pub mod pyenv;
    pub mod rbenv;
    pub mod snapcraft;
    pub mod uv;
    pub mod yarn;
}
use crate::package_managers::{
    apt::AptPackageManager, asdf::AsdfPackageManager, brew::BrewPackageManager,
    bun::BunPackageManager, cargo::CargoPackageManager, dnf::DnfPackageManager,
    gem::GemPackageManager, go::GoPackageManager, macports::MacPortsPackageManager,
    mise::MisePackageManager, npm::NpmPackageManager, nvm::NvmPackageManager,
    pacman::PacmanPackageManager, pip::PipPackageManager, pipx::PipxPackageManager,
    pnpm::PnpmPackageManager, pyenv::PyenvPackageManager, rbenv::RbenvPackageManager,
    snapcraft::SnapCraftPackageManager, uv::UvPackageManager, yarn::YarnPackageManager,
};
use package_manager::{PackageManager, ResolvedCommand};

mod command_resolver;
use crate::command_resolver::{resolve, CommandResolution};

use std::process::exit;
use std::sync::Arc;

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

fn all_package_managers() -> Vec<Box<dyn PackageManager + Send + Sync>> {
    vec![
        Box::new(AptPackageManager),
        Box::new(NpmPackageManager),
        Box::new(NvmPackageManager),
        Box::new(BrewPackageManager),
        Box::new(CargoPackageManager),
        Box::new(SnapCraftPackageManager),
        Box::new(PipPackageManager { bin: "pip" }),
        Box::new(PipPackageManager { bin: "pip3" }),
        Box::new(AsdfPackageManager),
        Box::new(PnpmPackageManager),
        Box::new(YarnPackageManager),
        Box::new(BunPackageManager),
        Box::new(GemPackageManager),
        Box::new(GoPackageManager),
        Box::new(PipxPackageManager),
        Box::new(UvPackageManager),
        Box::new(MacPortsPackageManager),
        Box::new(DnfPackageManager),
        Box::new(PacmanPackageManager),
        Box::new(MisePackageManager),
        Box::new(PyenvPackageManager),
        Box::new(RbenvPackageManager),
    ]
}

/// Reports the resolution to stdout/stderr and returns the resolution to
/// hand to the package managers, plus whether `type` resolved to anything.
fn report_resolution(
    command: &str,
    resolution: Result<CommandResolution, String>,
) -> (CommandResolution, bool) {
    match resolution {
        Ok(CommandResolution::Path(p)) => {
            println!("{} resolves to {}", command, p);
            (CommandResolution::Path(p), true)
        }
        Ok(CommandResolution::Alias(target)) => {
            println!("{} is an alias for {}", command, target);
            (CommandResolution::Alias(target), true)
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
    package_managers: Vec<Box<dyn PackageManager + Send + Sync>>,
    command: &str,
    resolution: &CommandResolution,
) -> Vec<String> {
    // Filter to installed managers up front — no point spawning a task per
    // manager just to check `command_exists`.
    let installed: Vec<Arc<dyn PackageManager + Send + Sync>> = package_managers
        .into_iter()
        .filter(|m| m.is_installed())
        .map(Arc::from)
        .collect();

    let tasks = installed.into_iter().map(|manager| {
        let command = command.to_string();
        let resolution = resolution.clone();
        // Each manager shells out — that's blocking work. `spawn_blocking`
        // runs it on the blocking pool so subprocesses don't starve the
        // async runtime threads.
        task::spawn_blocking(move || {
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

    let lookup = cmd.lookup_name();
    for manager_name in &installers {
        println!("{} installed by {}", lookup, manager_name);
    }

    if installers.is_empty() && !type_found {
        eprintln!("Failed to find package that installed command {}", command);
        exit(1)
    }
}
