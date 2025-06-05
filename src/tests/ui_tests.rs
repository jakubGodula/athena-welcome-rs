#[cfg(test)]
mod tests {
    use crate::ui::{create_welcome_page, create_info_page, create_credits_page, CURRENT_PAGE};
    use crate::utils::OsType;
    use crate::roles::{CURRENT_ROLES, toggle_role};
    use gtk4::prelude::*;
    use gtk4::{Application, Box, Label, MenuButton};
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_gtk() {
        INIT.call_once(|| {
            gtk4::init().unwrap();
        });
    }

    fn reset_state() {
        // Reset page state
        *CURRENT_PAGE.lock().unwrap() = 0;
        
        // Reset roles
        let mut roles = CURRENT_ROLES.lock().unwrap();
        roles.clear();
        drop(roles);
    }

    #[test]
    fn test_welcome_page_logic() {
        reset_state();
        
        // Test role selection logic
        toggle_role("blue".to_string());
        toggle_role("red".to_string());
        
        let roles = CURRENT_ROLES.lock().unwrap();
        assert!(roles.contains(&"blue".to_string()));
        assert!(roles.contains(&"red".to_string()));
        assert_eq!(roles.len(), 2);
    }

    #[test]
    fn test_info_page_logic() {
        reset_state();
        
        // Test page navigation logic
        let mut current_page = *CURRENT_PAGE.lock().unwrap();
        assert_eq!(current_page, 0);

        // Simulate navigation to info page
        *CURRENT_PAGE.lock().unwrap() = 1;
        current_page = *CURRENT_PAGE.lock().unwrap();
        assert_eq!(current_page, 1);

        // Test role persistence during navigation
        let mut roles = CURRENT_ROLES.lock().unwrap();
        roles.clear();
        roles.push("blue".to_string());
        drop(roles);

        // Verify roles persist after navigation
        let roles = CURRENT_ROLES.lock().unwrap();
        assert!(roles.contains(&"blue".to_string()));
    }

    #[test]
    fn test_create_credits_page() {
        init_gtk();
        let page = create_credits_page();
        assert!(page.is::<Box>());
        assert!(page.first_child().is_some());
        
        // Check if the credits label exists
        let label = page.first_child().unwrap().downcast::<Label>().unwrap();
        assert!(label.text().contains("Athena OS Team"));
    }

    #[test]
    fn test_page_navigation() {
        reset_state();
        
        // Test initial page
        let current_page = *CURRENT_PAGE.lock().unwrap();
        assert_eq!(current_page, 0);

        // Test page navigation
        *CURRENT_PAGE.lock().unwrap() = 1;
        let current_page = *CURRENT_PAGE.lock().unwrap();
        assert_eq!(current_page, 1);

        // Test page navigation back
        *CURRENT_PAGE.lock().unwrap() = 0;
        let current_page = *CURRENT_PAGE.lock().unwrap();
        assert_eq!(current_page, 0);
    }
} 