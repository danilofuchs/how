mod package_manager;
mod package_managers {
    pub mod apt;
    pub mod brew;
    pub mod cargo;
    pub mod npm;
}
use std::process::exit;

use crate::package_managers::apt::AptPackageManager;
use crate::package_managers::brew::BrewPackageManager;
use crate::package_managers::cargo::CargoPackageManager;
use crate::package_managers::npm::NpmPackageManager;
use clap::Parser;
use package_manager::PackageManager;

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
        // TODO: Add other package managers here
    ];

    let which_output = std::process::Command::new("which")
        .arg(command)
        .output()
        .expect("Failed to execute which command");

    if !which_output.status.success() {
        println!("Failed to find command {} in PATH", command);
        exit(1)
    }

    let mut matches = 0;

    // Match all package managers to see if the command is installed
    for manager in package_managers {
        match manager.is_installed(&args.command) {
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
