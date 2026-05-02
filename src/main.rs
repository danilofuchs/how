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
use crate::command_resolver::{resolve, Resolution};

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
fn report_resolution(command: &str, resolution: Result<Resolution, String>) -> (Resolution, bool) {
    match resolution {
        Ok(r @ Resolution::Path(_)) => {
            if let Resolution::Path(p) = &r {
                println!("{} resolves to {}", command, p);
            }
            (r, true)
        }
        Ok(r @ Resolution::Alias(_)) => {
            if let Resolution::Alias(target) = &r {
                println!("{} is an alias for {}", command, target);
            }
            (r, true)
        }
        Ok(Resolution::Function) => {
            println!("{} is a shell function", command);
            (Resolution::Function, true)
        }
        Ok(Resolution::Builtin) => {
            println!("{} is a shell builtin", command);
            (Resolution::Builtin, true)
        }
        Ok(Resolution::Keyword) => {
            println!("{} is a shell reserved word", command);
            (Resolution::Keyword, true)
        }
        Ok(Resolution::NotFound) => (Resolution::NotFound, false),
        Err(e) => {
            eprintln!("type resolver error: {}", e);
            (Resolution::NotFound, false)
        }
    }
}

async fn find_installers(
    package_managers: Vec<Box<dyn PackageManager + Send>>,
    command: &str,
    resolution: &Resolution,
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

    let installers = find_installers(all_package_managers(), command, &resolution).await;

    let lookup = match &resolution {
        Resolution::Alias(target) => target.as_str(),
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
