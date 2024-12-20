pub fn command_exists(command: &str) -> bool {
    std::process::Command::new("which")
        .arg(command)
        .output()
        .expect("Failed to execute which command")
        .status
        .success()
}
