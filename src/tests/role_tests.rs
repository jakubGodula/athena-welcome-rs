#[cfg(test)]
mod tests {
    use crate::roles::{CURRENT_ROLES, toggle_role, get_current_roles, save_roles, validate_roles};
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_toggle_role() {
        // Clear roles first
        let mut roles = CURRENT_ROLES.lock().unwrap();
        roles.clear();
        drop(roles);

        // Test adding a role
        toggle_role("blue".to_string());
        let roles = get_current_roles();
        assert!(roles.contains(&"blue".to_string()));

        // Test removing a role
        toggle_role("blue".to_string());
        let roles = get_current_roles();
        assert!(!roles.contains(&"blue".to_string()));

        // Test adding multiple roles
        toggle_role("red".to_string());
        toggle_role("web".to_string());
        let roles = get_current_roles();
        assert!(roles.contains(&"red".to_string()));
        assert!(roles.contains(&"web".to_string()));
    }

    #[test]
    fn test_get_current_roles() {
        // Clear roles first
        let mut roles = CURRENT_ROLES.lock().unwrap();
        roles.clear();
        drop(roles);

        // Test empty roles
        let roles = get_current_roles();
        assert!(roles.is_empty());

        // Test with some roles
        toggle_role("blue".to_string());
        toggle_role("red".to_string());
        let roles = get_current_roles();
        assert_eq!(roles.len(), 2);
        assert!(roles.contains(&"blue".to_string()));
        assert!(roles.contains(&"red".to_string()));
    }

    #[test]
    fn test_save_roles() {
        // Clear roles first
        let mut roles = CURRENT_ROLES.lock().unwrap();
        roles.clear();
        drop(roles);

        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Add some roles
        toggle_role("blue".to_string());
        toggle_role("red".to_string());

        // Save roles
        save_roles(path).unwrap();

        // Read back the file
        let content = fs::read_to_string(path).unwrap();
        let saved_roles: Vec<String> = content.lines()
            .map(|line| line.trim().to_string())
            .collect();

        assert!(saved_roles.contains(&"blue".to_string()));
        assert!(saved_roles.contains(&"red".to_string()));
        assert_eq!(saved_roles.len(), 2);
    }

    #[test]
    fn test_role_validation() {
        let roles = vec!["blue".to_string(), "red".to_string(), "invalid".to_string()];
        let validated = validate_roles(&roles);
        assert_eq!(validated.len(), 2);
        assert!(validated.contains(&"blue".to_string()));
        assert!(validated.contains(&"red".to_string()));
        assert!(!validated.contains(&"invalid".to_string()));
    }

    #[test]
    fn test_role_persistence() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Add some roles
        toggle_role("blue".to_string());
        toggle_role("red".to_string());

        // Save roles
        save_roles(path).unwrap();

        // Clear roles
        let mut roles = CURRENT_ROLES.lock().unwrap();
        roles.clear();
        drop(roles);

        // Read roles from file
        let roles = get_current_roles();
        assert!(roles.is_empty());
    }
} 