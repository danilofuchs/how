mod package_manager;
mod package_managers {
    pub mod apt;
}
use crate::package_managers::apt::AptPackageManager;
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
    let args = Args::parse();

    let package_managers: Vec<Box<dyn PackageManager>> = vec![
        Box::new(AptPackageManager) as Box<dyn PackageManager>,
        // TODO: Add other package managers here
    ];

    for package in package_managers {
        match package.is_installed(&args.command) {
            Ok(true) => {
                println!("{} installed by {}", args.command, package.name());
                return;
            }
            Ok(false) => {
                continue;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
    }
}
