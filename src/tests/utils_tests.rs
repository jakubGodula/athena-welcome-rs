#[cfg(test)]
mod tests {
    use crate::utils::{detect_os, OsType};
    use crate::roles::{read_roles_config, validate_roles};
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_detect_os_arch() {
        // Create a temporary os-release file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ID=arch").unwrap();
        
        // Temporarily set the path
        let original_path = std::env::var("OS_RELEASE_PATH").ok();
        std::env::set_var("OS_RELEASE_PATH", temp_file.path());
        
        let os_type = detect_os(temp_file.path());
        assert!(matches!(os_type, OsType::Arch));
        
        // Restore original path if it existed
        if let Some(path) = original_path {
            std::env::set_var("OS_RELEASE_PATH", path);
        }
    }

    #[test]
    fn test_detect_os_nixos() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ID=nixos").unwrap();
        
        let original_path = std::env::var("OS_RELEASE_PATH").ok();
        std::env::set_var("OS_RELEASE_PATH", temp_file.path());
        
        let os_type = detect_os(temp_file.path());
        assert!(matches!(os_type, OsType::NixOS));
        
        if let Some(path) = original_path {
            std::env::set_var("OS_RELEASE_PATH", path);
        }
    }

    #[test]
    fn test_detect_os_unknown() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ID=unknown").unwrap();
        
        let original_path = std::env::var("OS_RELEASE_PATH").ok();
        std::env::set_var("OS_RELEASE_PATH", temp_file.path());
        
        let os_type = detect_os(temp_file.path());
        assert!(matches!(os_type, OsType::Other(_)));
        
        if let Some(path) = original_path {
            std::env::set_var("OS_RELEASE_PATH", path);
        }
    }

    #[test]
    fn test_validate_roles() {
        let valid_roles = vec!["blue".to_string(), "red".to_string()];
        let invalid_roles = vec!["invalid".to_string(), "blue".to_string()];
        
        let validated = validate_roles(&valid_roles);
        assert_eq!(validated.len(), 2);
        assert!(validated.contains(&"blue".to_string()));
        assert!(validated.contains(&"red".to_string()));
        
        let validated = validate_roles(&invalid_roles);
        assert_eq!(validated.len(), 1);
        assert!(validated.contains(&"blue".to_string()));
    }

    #[test]
    fn test_read_roles_config() {
        // Create a temporary config file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "blue\nred\ninvalid").unwrap();
        
        let roles = read_roles_config(temp_file.path().to_str().unwrap());
        assert_eq!(roles.len(), 2);
        assert!(roles.contains(&"blue".to_string()));
        assert!(roles.contains(&"red".to_string()));
    }

    #[test]
    fn test_read_roles_config_empty() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "").unwrap();
        
        let roles = read_roles_config(temp_file.path().to_str().unwrap());
        assert!(roles.is_empty());
    }

    #[test]
    fn test_read_roles_config_invalid_path() {
        let roles = read_roles_config("/nonexistent/path/roles.conf");
        assert!(roles.is_empty());
    }
} 