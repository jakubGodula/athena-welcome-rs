mod utils;
mod ui;
mod roles;

use gtk4::prelude::*;
use gtk4::{glib, gio, Application, ApplicationWindow, Notebook, Label, Box, Orientation, Align, Picture, ListBox, ListBoxRow, CheckButton, SelectionMode, MenuButton, Popover, Button, CssProvider, LinkButton, TextView, ScrolledWindow, WrapMode, PolicyType, Justification, ToggleButton};
use std::fs;
use std::io::{BufRead, BufReader};
use std::env;
use std::process::Command;
use std::thread;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use libc;

// Assume async_channel is added to Cargo.toml
// use async_channel; // Not strictly needed for `::unbounded()` call if prelude isn't used

use crate::utils::{detect_os, OsType};
use crate::roles::{read_roles_config, CURRENT_ROLES, ITEMS};
use crate::ui::{create_welcome_page, create_info_page, create_credits_page, CURRENT_PAGE};

#[derive(Debug, Clone, PartialEq)]
enum PackageManager {
    Pacman,
    Nix,
    Unknown,
}

fn detect_package_manager() -> PackageManager {
    // Check for pacman (Arch)
    if Command::new("pacman").arg("--version").output().is_ok() {
        return PackageManager::Pacman;
    }
    // Check for nix (NixOS) - nix-shell is a common command, or just "nix"
    if Command::new("nix").arg("--version").output().is_ok() || Command::new("nix-shell").arg("--version").output().is_ok() {
        return PackageManager::Nix;
    }
    PackageManager::Unknown
}

// Add this function to validate roles
fn validate_roles(roles: &[String]) -> Vec<String> {
    let valid_roles: Vec<&str> = ITEMS.iter().map(|(_, id)| *id).collect();
    roles.iter()
        .filter(|role| valid_roles.contains(&role.as_str()))
        .cloned()
        .collect()
}

fn setup_display_environment() {
    // Try to get the current user's display
    if let Ok(real_user) = Command::new("whoami").output() {
        let real_user = String::from_utf8_lossy(&real_user.stdout).trim().to_string();
        
        // Try to get display from environment first
        if env::var("DISPLAY").is_err() {
            // Try to get display from various sources
            let display_sources = [
                ("loginctl", vec!["show-user", &real_user, "-p", "Display"]),
                ("w", vec!["-hs", &real_user]),
                ("who", vec![]),
            ];

            for (cmd, args) in display_sources.iter() {
                if let Ok(output) = Command::new(cmd).args(args).output() {
                    let output = String::from_utf8_lossy(&output.stdout);
                    for line in output.lines() {
                        if line.contains(":") {
                            let display = line.split_whitespace()
                                .find(|s| s.contains(":"))
                                .unwrap_or(":0");
                            env::set_var("DISPLAY", display);
                            break;
                        }
                    }
                }
            }
        }

        // Set XAUTHORITY if not set
        if env::var("XAUTHORITY").is_err() {
            if let Ok(home) = env::var("HOME") {
                let xauth_path = format!("{}/.Xauthority", home);
                if std::path::Path::new(&xauth_path).exists() {
                    env::set_var("XAUTHORITY", xauth_path);
                }
            }
        }
    }
}

fn get_system_theme_settings() -> Vec<(String, String)> {
    let mut theme_settings = Vec::new();
    
    // Try to get theme settings from various sources
    let theme_sources = [
        // GNOME settings
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "gtk-theme"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "icon-theme"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "cursor-theme"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "font-name"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "monospace-font-name"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "document-font-name"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "titlebar-font"]),
        ("gsettings", vec!["get", "org.gnome.desktop.wm.preferences", "theme"]),
        ("gsettings", vec!["get", "org.gnome.desktop.wm.preferences", "button-layout"]),
        // GTK 3 settings
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "gtk-theme"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "icon-theme"]),
        ("gsettings", vec!["get", "org.gnome.desktop.interface", "cursor-theme"]),
        // XFCE settings
        ("xfconf-query", vec!["-c", "xsettings", "-p", "/Net/ThemeName"]),
        ("xfconf-query", vec!["-c", "xsettings", "-p", "/Net/IconThemeName"]),
        ("xfconf-query", vec!["-c", "xsettings", "-p", "/Gtk/CursorThemeName"]),
        // KDE settings
        ("kreadconfig5", vec!["--file", "kcmfonts", "--group", "General", "--key", "font"]),
        ("kreadconfig5", vec!["--file", "kcminputrc", "--group", "Mouse", "--key", "cursorTheme"]),
    ];

    for (cmd, args) in theme_sources.iter() {
        if let Ok(output) = Command::new(cmd).args(args).output() {
            if let Ok(value) = String::from_utf8(output.stdout) {
                let value = value.trim().trim_matches('\'');
                if !value.is_empty() {
                    match *cmd {
                        "gsettings" => {
                            if args[1] == "org.gnome.desktop.interface" {
                                match args[2] {
                                    "gtk-theme" => theme_settings.push(("GTK_THEME".to_string(), value.to_string())),
                                    "icon-theme" => theme_settings.push(("GTK_ICON_THEME_NAME".to_string(), value.to_string())),
                                    "cursor-theme" => theme_settings.push(("GTK_CURSOR_THEME_NAME".to_string(), value.to_string())),
                                    "font-name" => theme_settings.push(("GTK_FONT".to_string(), value.to_string())),
                                    "monospace-font-name" => theme_settings.push(("GTK_MONOSPACE_FONT".to_string(), value.to_string())),
                                    "document-font-name" => theme_settings.push(("GTK_DOCUMENT_FONT".to_string(), value.to_string())),
                                    "titlebar-font" => theme_settings.push(("GTK_TITLEBAR_FONT".to_string(), value.to_string())),
                                    _ => {}
                                }
                            } else if args[1] == "org.gnome.desktop.wm.preferences" {
                                match args[2] {
                                    "theme" => theme_settings.push(("GNOME_WM_THEME".to_string(), value.to_string())),
                                    "button-layout" => theme_settings.push(("GNOME_WM_BUTTON_LAYOUT".to_string(), value.to_string())),
                                    _ => {}
                                }
                            }
                        }
                        "xfconf-query" => {
                            match args[3] {
                                "/Net/ThemeName" => theme_settings.push(("GTK_THEME".to_string(), value.to_string())),
                                "/Net/IconThemeName" => theme_settings.push(("GTK_ICON_THEME_NAME".to_string(), value.to_string())),
                                "/Gtk/CursorThemeName" => theme_settings.push(("GTK_CURSOR_THEME_NAME".to_string(), value.to_string())),
                                _ => {}
                            }
                        }
                        "kreadconfig5" => {
                            if args[1] == "kcmfonts" {
                                theme_settings.push(("GTK_FONT".to_string(), value.to_string()));
                            } else if args[1] == "kcminputrc" {
                                theme_settings.push(("GTK_CURSOR_THEME_NAME".to_string(), value.to_string()));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Add GNOME-specific theme paths
    if let Ok(home) = env::var("HOME") {
        let theme_paths = [
            format!("{}/.themes", home),
            format!("{}/.local/share/themes", home),
            "/usr/share/themes".to_string(),
            "/usr/local/share/themes".to_string(),
        ];
        
        for path in theme_paths {
            if std::path::Path::new(&path).exists() {
                theme_settings.push(("GTK_THEME_PATH".to_string(), path));
            }
        }
    }

    // Add some default theme settings if none were found
    if theme_settings.is_empty() {
        theme_settings.extend_from_slice(&[
            ("GTK_THEME".to_string(), "Adwaita".to_string()),
            ("GTK_ICON_THEME_NAME".to_string(), "Adwaita".to_string()),
            ("GTK_CURSOR_THEME_NAME".to_string(), "Adwaita".to_string()),
            ("GTK_FONT".to_string(), "Cantarell 11".to_string()),
            ("GTK_MONOSPACE_FONT".to_string(), "Monospace 11".to_string()),
            ("GTK_DOCUMENT_FONT".to_string(), "Sans 11".to_string()),
            ("GTK_TITLEBAR_FONT".to_string(), "Cantarell Bold 11".to_string()),
            ("GNOME_WM_THEME".to_string(), "Adwaita".to_string()),
            ("GNOME_WM_BUTTON_LAYOUT".to_string(), "menu:minimize,maximize,close".to_string()),
        ]);
    }

    theme_settings
}

fn check_root_access() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn main() -> glib::ExitCode {
    // Check for root access first
    check_root_access();

    // Set up display environment first
    setup_display_environment();

    // Read roles from config file before creating the application
    let config_path = "/etc/athena-welcome/roles.conf";
    let roles = read_roles_config(config_path);
    *CURRENT_ROLES.lock().unwrap() = roles;

    let app = Application::builder()
        .application_id("org.athenaos.welcome")
        .build();

    // Load resources before connecting to activate
    gio::resources_register_include!("compiled_resources.rs")
        .expect("Failed to register resources from build script output.");

    app.connect_activate(build_ui);
    
    // Handle cleanup on exit
    app.connect_shutdown(move |_| {
        // Ensure all resources are properly cleaned up
        std::process::exit(0);
    });

    app.run()
}

fn build_ui(app: &Application) {
    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Athena Welcome")
        .default_width(1440)
        .default_height(1080)
        .build();

    // Create and set up the header bar
    let header_bar = gtk4::HeaderBar::new();
    header_bar.set_show_title_buttons(true);
    header_bar.set_title_widget(Some(&Label::new(Some("Athena Welcome"))));
    window.set_titlebar(Some(&header_bar));

    // CSS Provider
    let provider = CssProvider::new();
    let css_data = r#"
        button.rounded,
        checkbutton.rounded check,
        checkbutton.rounded indicator {
            border-radius: 12px;
        }
        .click-area {
            min-width: 20%;
            background-color: alpha(@theme_selected_bg_color, 0.15);
            transition: all 0.2s ease-in-out;
        }
        .click-area:hover {
            background-color: alpha(@theme_selected_bg_color, 0.3);
        }
        .left-area {
            border-radius: 12px 0 0 12px;
        }
        .right-area {
            border-radius: 0 12px 12px 0;
        }
        picture {
            max-width: 100%;
            max-height: 100%;
        }
        picture > image {
            max-width: 100%;
            max-height: 100%;
        }
        .welcome-image {
            max-width: 400px;
            max-height: 200px;
        }
        .htb-icon {
            max-width: 16px;
            max-height: 16px;
        }
        .button-icon {
            max-width: 16px;
            max-height: 16px;
        }
        .role-popover {
            margin: 0 auto;
            z-index: 1000;
        }
        .role-popover list {
            min-width: 300px;
            margin: 10px;
        }
        .role-popover checkbutton {
            padding: 8px 12px;
            margin: 4px 8px;
            border-radius: 8px;
            transition: all 0.2s ease-in-out;
        }
        .role-popover checkbutton:hover {
            background-color: alpha(@theme_selected_bg_color, 0.1);
        }
        .role-popover checkbutton:checked {
            background-color: alpha(@theme_selected_bg_color, 0.2);
        }
        notebook {
            margin: 15px;
            background-color: @theme_bg_color;
            border-radius: 12px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        notebook > header {
            background-color: @theme_bg_color;
            border-bottom: 1px solid @borders;
            padding: 0;
            margin: 0;
            border-radius: 12px 12px 0 0;
        }
        notebook > header > tabs {
            background-color: @theme_bg_color;
            padding: 0;
            margin: 0;
        }
        notebook > header > tabs > tab {
            padding: 6px 12px;
            margin: 0;
            min-width: 80px;
            border: none;
            background: none;
            color: @theme_fg_color;
            transition: all 0.2s ease-in-out;
        }
        notebook > header > tabs > tab:checked {
            background-color: @theme_selected_bg_color;
            color: @theme_selected_fg_color;
            border-radius: 6px 6px 0 0;
            transform: translateY(-2px);
        }
        notebook > header > tabs > tab:checked:hover {
            background-color: alpha(@theme_selected_bg_color, 0.85);
        }
        notebook > header > tabs > tab:hover {
            background-color: alpha(@theme_selected_bg_color, 0.1);
            border-radius: 6px;
        }
        menubutton {
            padding: 4px 8px;
            margin: 2px 4px;
            min-width: 150px;
        }
        menubutton > button {
            padding: 4px 8px;
            min-width: 150px;
        }
        window {
            background-color: @theme_bg_color;
        }
        headerbar {
            background-color: @theme_bg_color;
            border-bottom: 1px solid @borders;
        }
        headerbar button.titlebutton {
            min-width: 24px;
            min-height: 24px;
            padding: 0;
            margin: 0;
        }
        .page-tabs {
            background-color: @theme_bg_color;
            border-bottom: 1px solid @borders;
            padding: 0;
            margin: 0;
        }
        .page-tabs button {
            padding: 6px 12px;
            margin: 0;
            min-width: 80px;
            border: none;
            background: none;
            color: @theme_fg_color;
            transition: all 0.2s ease-in-out;
        }
        .page-tabs button:checked {
            background-color: @theme_selected_bg_color;
            color: @theme_selected_fg_color;
            border-radius: 6px 6px 0 0;
            transform: translateY(-2px);
        }
        .page-tabs button:checked:hover {
            background-color: alpha(@theme_selected_bg_color, 0.85);
        }
        .page-tabs button:hover {
            background-color: alpha(@theme_selected_bg_color, 0.1);
            border-radius: 6px;
        }
    "#;
    provider.load_from_data(css_data);
    gtk4::style_context_add_provider_for_display(
        &gtk4::prelude::WidgetExt::display(&window),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Create a notebook (tabbed interface)
    let notebook = Notebook::new();
    notebook.set_vexpand(true);
    notebook.set_hexpand(true);
    notebook.set_show_tabs(false); // Hide default tabs
    notebook.set_show_border(false); // Hide border to prevent clipping
    
    // Create click area overlays
    let left_overlay = Box::new(Orientation::Horizontal, 0);
    left_overlay.set_hexpand(false);
    left_overlay.set_vexpand(true);
    left_overlay.add_css_class("click-area");
    left_overlay.add_css_class("left-area");
    
    let right_overlay = Box::new(Orientation::Horizontal, 0);
    right_overlay.set_hexpand(false);
    right_overlay.set_vexpand(true);
    right_overlay.add_css_class("click-area");
    right_overlay.add_css_class("right-area");
    
    // Create main content box
    let main_box = Box::new(Orientation::Horizontal, 0);
    main_box.append(&left_overlay);
    main_box.append(&notebook);
    main_box.append(&right_overlay);
    
    let os_type = detect_os(std::path::Path::new("/etc/os-release"));

    // Create the three main tabs/pages
    let welcome_page = create_welcome_page(app, &os_type);
    let info_page = create_info_page();
    let credits_page = create_credits_page();
    
    // Add the pages to the notebook
    notebook.append_page(&welcome_page, None::<&Label>);
    notebook.append_page(&info_page, None::<&Label>);
    notebook.append_page(&credits_page, None::<&Label>);

    // Create click areas for navigation
    let notebook_clone = notebook.clone();
    let gesture = gtk4::GestureClick::new();
    gesture.connect_pressed(move |_, _, x, _| {
        let notebook = notebook_clone.clone();
        let window_width = notebook.width() as f64;
        let click_area = window_width * 0.2;

        if x < click_area {
            // Click on left side - go to previous page
            let mut current_page = *CURRENT_PAGE.lock().unwrap();
            let num_pages = notebook.n_pages() as i32;
            current_page = if current_page == 0 {
                num_pages - 1
            } else {
                current_page - 1
            };
            *CURRENT_PAGE.lock().unwrap() = current_page;
            notebook.set_page(current_page);
        } else if x > (window_width - click_area) {
            // Click on right side - go to next page
            let mut current_page = *CURRENT_PAGE.lock().unwrap();
            let num_pages = notebook.n_pages() as i32;
            current_page = if current_page == num_pages - 1 {
                0
            } else {
                current_page + 1
            };
            *CURRENT_PAGE.lock().unwrap() = current_page;
            notebook.set_page(current_page);
        }
    });
    notebook.add_controller(gesture);

    // Set the main box as the content of the window
    window.set_child(Some(&main_box));
    
    // Show the window
    window.present();
}
