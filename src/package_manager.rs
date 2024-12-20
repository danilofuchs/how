pub trait PackageManager {
    fn name(&self) -> &str;
    fn is_installed(&self) -> bool;
    fn is_command_installed(&self, command: &str) -> Result<bool, String>;
}
