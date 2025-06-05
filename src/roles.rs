use std::fs;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static CURRENT_ROLES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub const ITEMS: [(&str, &str); 12] = [
    ("💙 Blue Teamer 💙", "blue"),
    ("🐞 Bug Bounty Hunter 🐞", "hunter"),
    ("🍘 Cracker Specialist 🍘", "cracker"),
    ("💀 DoS Tester 💀", "dos"),
    ("🎓 Enthusiast Student 🎓", "student"),
    ("🔍 Forensic Analyst 🔍", "forensic"),
    ("🦠 Malware Analyst 🦠", "malware"),
    ("📱 Mobile Analyst 📱", "mobile"),
    ("🌐 Network Analyst 🌐", "network"),
    ("🕵️ OSINT Specialist 🕵️", "osint"),
    ("❤️ Red Teamer ❤️", "red"),
    ("🕸️ Web Pentester 🕸️", "web"),
];

pub fn validate_roles(roles: &[String]) -> Vec<String> {
    let valid_roles: Vec<&str> = ITEMS.iter().map(|(_, id)| *id).collect();
    roles.iter()
        .filter(|role| valid_roles.contains(&role.as_str()))
        .cloned()
        .collect()
}

pub fn read_roles_config(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(content) => {
            let roles: Vec<String> = content.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect();
            
            validate_roles(&roles)
        }
        Err(_) => Vec::new()
    }
}

pub fn save_roles(path: &std::path::Path) -> std::io::Result<()> {
    let roles = CURRENT_ROLES.lock().unwrap();
    let content = roles.join("\n");
    fs::write(path, content)
}

pub fn toggle_role(role: String) {
    let mut roles = CURRENT_ROLES.lock().unwrap();
    if roles.contains(&role) {
        roles.retain(|r| r != &role);
    } else {
        roles.push(role);
    }
}

pub fn get_current_roles() -> Vec<String> {
    CURRENT_ROLES.lock().unwrap().clone()
} 