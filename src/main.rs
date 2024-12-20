mod package_manager;
mod package_managers {
    pub mod apt;
    pub mod brew;
    pub mod cargo;
    pub mod npm;
    pub mod pip;
    pub mod pip3;
    pub mod snapcraft;
    pub mod asdf;
}
use crate::package_managers::{
    apt::AptPackageManager, brew::BrewPackageManager, cargo::CargoPackageManager,
    npm::NpmPackageManager, pip::PipPackageManager, pip3::Pip3PackageManager,
    snapcraft::SnapCraftPackageManager, asdf::AsdfPackageManager
};
use package_manager::PackageManager;

mod command_exists;
use crate::command_exists::command_exists;

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

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let command = &args.command;

    let package_managers: Vec<Box<dyn PackageManager + Send>> = vec![
        Box::new(AptPackageManager) as Box<dyn PackageManager + Send>,
        Box::new(NpmPackageManager) as Box<dyn PackageManager + Send>,
        Box::new(BrewPackageManager) as Box<dyn PackageManager + Send>,
        Box::new(CargoPackageManager) as Box<dyn PackageManager + Send>,
        Box::new(SnapCraftPackageManager) as Box<dyn PackageManager + Send>,
        Box::new(PipPackageManager) as Box<dyn PackageManager + Send>,
        Box::new(Pip3PackageManager) as Box<dyn PackageManager + Send>,
        Box::new(AsdfPackageManager) as Box<dyn PackageManager + Send>,
        // TODO: Add other package managers here
    ];

    if !command_exists("which") {
        eprintln!("command 'which' not found in PATH. It is required for this program to work.");
        exit(1)
    }

    // Some programs are installed by name (eg graphite) but the executable is different (eg gt)
    // if !command_exists(command) {
    //     eprintln!("command '{}' not found in PATH", command);
    //     exit(1)
    // }

    let mut tasks = vec![];

    for manager in package_managers {
        let command = command.clone();
        tasks.push(task::spawn(async move {
            if !manager.is_installed() {
                return None;
            }

            match manager.is_command_installed(&command) {
                Ok(true) => Some(manager.name().to_string()),
                Ok(false) => None,
                Err(e) => {
                    eprintln!("{} Error: {}", manager.name(), e);
                    None
                }
            }
        }));
    }

    let results = join_all(tasks).await;

    let mut matches = 0;

    for result in results {
        if let Some(manager_name) = result.unwrap() {
            println!("{} installed by {}", command, manager_name);
            matches += 1;
        }
    }

    if matches == 0 {
        eprintln!("Failed to find package that installed command {}", command);
        exit(1)
    }
}
