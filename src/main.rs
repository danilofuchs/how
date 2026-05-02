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
use package_manager::PackageManager;

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

/// Reports the resolution to stdout/stderr and returns the lookup name to
/// pass to the package managers, plus whether `type` resolved to anything.
fn report_resolution(command: &str, resolution: &Result<Resolution, String>) -> (String, bool) {
    match resolution {
        Ok(Resolution::Path(p)) => {
            println!("{} resolves to {}", command, p);
            (command.to_string(), true)
        }
        Ok(Resolution::Alias(target)) => {
            println!("{} is an alias for {}", command, target);
            (target.clone(), true)
        }
        Ok(Resolution::Function) => {
            println!("{} is a shell function", command);
            (command.to_string(), true)
        }
        Ok(Resolution::Builtin) => {
            println!("{} is a shell builtin", command);
            (command.to_string(), true)
        }
        Ok(Resolution::Keyword) => {
            println!("{} is a shell reserved word", command);
            (command.to_string(), true)
        }
        Ok(Resolution::NotFound) => (command.to_string(), false),
        Err(e) => {
            eprintln!("type resolver error: {}", e);
            (command.to_string(), false)
        }
    }
}

async fn find_installers(
    package_managers: Vec<Box<dyn PackageManager + Send>>,
    lookup: &str,
) -> Vec<String> {
    let tasks = package_managers.into_iter().map(|manager| {
        let lookup = lookup.to_string();
        task::spawn(async move {
            if !manager.is_installed() {
                return None;
            }
            match manager.is_command_installed(&lookup) {
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
    let (lookup, type_found) = report_resolution(command, &resolution);

    let installers = find_installers(all_package_managers(), &lookup).await;

    for manager_name in &installers {
        println!("{} installed by {}", lookup, manager_name);
    }

    if installers.is_empty() && !type_found {
        eprintln!("Failed to find package that installed command {}", command);
        exit(1)
    }
}
