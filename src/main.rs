mod package_manager;
mod package_managers {
    pub mod apt;
    pub mod brew;
    pub mod cargo;
    pub mod npm;
    pub mod snapcraft;
}
use crate::package_managers::{
    apt::AptPackageManager, brew::BrewPackageManager, cargo::CargoPackageManager,
    npm::NpmPackageManager, snapcraft::SnapCraftPackageManager,
};
use package_manager::PackageManager;

mod command_exists;
use crate::command_exists::command_exists;

use std::process::exit;

use clap::Parser;

/// Finds the package manager that installed a command
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the command to search
    command: String,
}

fn main() {
    let args: Args = Args::parse();
    let command = &args.command;

    let package_managers: Vec<Box<dyn PackageManager>> = vec![
        Box::new(AptPackageManager) as Box<dyn PackageManager>,
        Box::new(NpmPackageManager) as Box<dyn PackageManager>,
        Box::new(BrewPackageManager) as Box<dyn PackageManager>,
        Box::new(CargoPackageManager) as Box<dyn PackageManager>,
        Box::new(SnapCraftPackageManager) as Box<dyn PackageManager>,
        // TODO: Add other package managers here
    ];

    if !command_exists("which") {
        eprintln!("which command not found in PATH. It is required for this program to work.");
        exit(1)
    }

    if !command_exists(command) {
        eprintln!("command not found in PATH");
        exit(1)
    }

    let mut matches = 0;

    // Match all package managers to see if the command is installed
    for manager in package_managers {
        if !manager.is_installed() {
            continue;
        }

        match manager.is_command_installed(&args.command) {
            Ok(true) => {
                println!("{} installed by {}", args.command, manager.name());
                matches += 1;
                continue;
            }
            Ok(false) => {
                continue;
            }
            Err(e) => {
                eprintln!("{} Error: {}", manager.name(), e);
                continue;
            }
        }
    }

    if matches == 0 {
        eprintln!(
            "Failed to find package that installed command {}",
            args.command
        );
        exit(1)
    }
}
