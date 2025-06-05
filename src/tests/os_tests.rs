#[cfg(test)]
mod tests {
    use crate::utils::{detect_os, detect_package_manager, OsType, PackageManager};
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_detect_os() {
        // Test Arch Linux
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ID=arch").unwrap();
        let os_type = detect_os(temp_file.path());
        assert!(matches!(os_type, OsType::Arch));

        // Test NixOS
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ID=nixos").unwrap();
        let os_type = detect_os(temp_file.path());
        assert!(matches!(os_type, OsType::NixOS));

        // Test unknown OS
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ID=unknown").unwrap();
        let os_type = detect_os(temp_file.path());
        assert!(matches!(os_type, OsType::Other(_)));

        // Test missing file
        let os_type = detect_os(std::path::Path::new("/nonexistent/path"));
        assert!(matches!(os_type, OsType::Unknown));
    }

    #[test]
    fn test_detect_package_manager() {
        // Test Arch Linux package manager
        let package_manager = detect_package_manager(&OsType::Arch);
        assert!(matches!(package_manager, PackageManager::Pacman));

        // Test NixOS package manager
        let package_manager = detect_package_manager(&OsType::NixOS);
        assert!(matches!(package_manager, PackageManager::Nix));

        // Test unknown OS package manager
        let package_manager = detect_package_manager(&OsType::Unknown);
        assert!(matches!(package_manager, PackageManager::Unknown));
    }

    #[test]
    fn test_os_package_manager_integration() {
        // Test Arch Linux integration
        let os_type = OsType::Arch;
        let package_manager = detect_package_manager(&os_type);
        assert!(matches!(package_manager, PackageManager::Pacman));

        // Test NixOS integration
        let os_type = OsType::NixOS;
        let package_manager = detect_package_manager(&os_type);
        assert!(matches!(package_manager, PackageManager::Nix));
    }
} 