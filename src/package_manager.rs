pub trait PackageManager {
    fn name(&self) -> &str;
    fn is_installed(&self, command: &str) -> Result<bool, String>;
}
