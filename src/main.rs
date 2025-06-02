use gtk::prelude::*;
use gtk::{glib, gio, Application, ApplicationWindow, Notebook, Label, Box, Orientation, Align, Picture, ListBox, ListBoxRow, CheckButton, SelectionMode, MenuButton, Popover, Button, CssProvider, LinkButton, TextView, ScrolledWindow};
use std::fs;
use std::io::{BufRead, BufReader};
use std::env;
use std::process::Command;
use std::thread;

// Assume async_channel is added to Cargo.toml
// use async_channel; // Not strictly needed for `::unbounded()` call if prelude isn't used

static ITEMS: [(&str, i32); 12] = [
    ("💙 Blue Teamer 💙", 0),
    ("🐞 Bug Bounty Hunter 🐞", 1),
    ("🍘 Cracker Specialist 🍘", 2),
    ("💀 DoS Tester 💀", 3),
    ("🎓 Enthusiast Student 🎓", 4),
    ("🔍 Forensic Analyst 🔍", 5),
    ("🦠 Malware Analyst 🦠", 6),
    ("📱 Mobile Analyst 📱", 7),
    ("🌐 Network Analyst 🌐", 8),
    ("🕵️ OSINT Specialist 🕵️", 9),
    ("❤️ Red Teamer ❤️", 10),
    ("🕸️ Web Pentester 🕸️", 11),
];

#[derive(Debug, Clone)]
enum OsType {
    Arch,
    NixOS,
    Other(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
enum PackageManager {
    Pacman,
    Nix,
    Unknown,
}

fn detect_os() -> OsType {
    if let Ok(file) = fs::File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        let mut os_id = String::new();
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.starts_with("ID=") {
                    os_id = line_content.trim_start_matches("ID=").trim_matches('"').to_lowercase();
                    break;
                }
            }
        }
        match os_id.as_str() {
            "arch" => OsType::Arch,
            "nixos" => OsType::NixOS,
            _ if !os_id.is_empty() => OsType::Other(os_id),
            _ => OsType::Unknown,
        }
    } else {
        OsType::Unknown
    }
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

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.athenaos.welcome")
        .build();

    // Load resources before connecting to activate
    gio::resources_register_include!("compiled_resources.rs")
        .expect("Failed to register resources from build script output.");

    app.connect_activate(build_ui);
    // CSS Provider setup moved to build_ui
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

    // CSS Provider
    let provider = CssProvider::new();
    let css_data = r#"
        button.rounded,
        checkbutton.rounded check,
        checkbutton.rounded indicator {
            border-radius: 12px;
        }
        notebook > header > tabs > tab {
            padding: 6px 12px; /* Adjust padding for tab buttons */
        }
        notebook > header > tabs > tab:checked:hover {
            background-color: alpha(@theme_selected_bg_color, 0.85); /* Hover effect for active tab */
            /* As an alternative, for a more distinct color, you could try something like: */
            /* background-image: none; */
            /* background-color: #e0e0e0; */
        }
    "#;
    provider.load_from_string(css_data);
    gtk::style_context_add_provider_for_display(
        &gtk::prelude::WidgetExt::display(&window),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Create a notebook (tabbed interface)
    let notebook = Notebook::new();
    notebook.set_margin_top(15); // Add top padding to the notebook
    
    let os_type = detect_os();

    // Create the three main tabs/pages
    let welcome_page = create_welcome_page(&os_type);
    let info_page = create_info_page();
    let credits_page = create_credits_page();
    
    // Add the pages to the notebook with their labels
    let welcome_label = Label::new(Some("Welcome"));
    welcome_label.set_hexpand(true);
    notebook.append_page(&welcome_page, Some(&welcome_label));

    let info_label = Label::new(Some("Information"));
    info_label.set_hexpand(true);
    notebook.append_page(&info_page, Some(&info_label));

    let credits_label = Label::new(Some("Credits"));
    credits_label.set_hexpand(true);
    notebook.append_page(&credits_page, Some(&credits_label));
    
    // Set the notebook as the main content of the window
    window.set_child(Some(&notebook));
    
    // Show the window
    window.present();
}

fn create_welcome_page(os_type: &OsType) -> Box {
    let content = Box::new(Orientation::Vertical, 10);

    // OS Warning
    match os_type {
        OsType::Other(id) => {
            let warning_text = format!(
               "<span><b>⚠️ Warning: Athena OS is primarily designed for Arch Linux and NixOS.</b>\\n\
                You are running on an untested OS ({}). Some features might not work as expected.</span>",
                id
            );
            let warning_label = Label::new(None);
            warning_label.set_markup(&warning_text);
            warning_label.set_margin_top(15);
            warning_label.set_margin_bottom(5);
            warning_label.set_halign(Align::Center);
            warning_label.set_wrap(true);
            content.append(&warning_label);
        }
        OsType::Unknown => {
           let warning_text = format!(
               "<span><b>⚠️ Warning: Athena OS is primarily designed for Arch Linux and NixOS.</b>\\n\
                Could not determine your OS ({}). Some features might not work as expected.</span>",
               "Unknown"
           );
           let warning_label = Label::new(None);
           warning_label.set_markup(&warning_text);
           warning_label.set_margin_top(15);
           warning_label.set_margin_bottom(5);
           warning_label.set_halign(Align::Center);
           warning_label.set_wrap(true);
           content.append(&warning_label);
        }
        _ => {}
    }
    
    let welcome_text =
    "
    <span>
    <b>
    This is the welcome page of Athena OS.
    </b>
    </span>
    ";
    let description = Label::new(None);
    description.set_markup(welcome_text);
    
    let image_path = match os_type {
        OsType::Arch => "/org/athenaos/welcome/images/athena-arch-one-liner.png",
        OsType::NixOS => "/org/athenaos/welcome/images/athena-nix-one-liner.png",
        _ => "/org/athenaos/welcome/images/athena-arch-one-liner.png",
    };
    let athena_pic = Picture::for_resource(image_path);
    
    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::Multiple);

    for (item_text, _item_id) in ITEMS.iter() {
        let check_button = CheckButton::with_label(item_text);
        check_button.set_halign(Align::Center);
        let row = ListBoxRow::new();
        row.set_child(Some(&check_button));
        list_box.append(&row);
    }

    let popover = Popover::new();
    popover.set_child(Some(&list_box));
    popover.set_has_arrow(false);
    popover.set_autohide(true);

    let select_role_button = MenuButton::new();
    select_role_button.set_popover(Some(&popover));
    select_role_button.set_label("🔥 Choose your Role 🔥");
        
    let console_output = TextView::new();
    console_output.set_editable(false);
    console_output.set_cursor_visible(false);
    console_output.set_wrap_mode(gtk::WrapMode::WordChar);
    let buffer = console_output.buffer();
    
    let initial_shell_prompt = match env::var("SHELL") {
        Ok(shell_path) => format!("Current shell: {}\\nWelcome to Athena OS!\\nThe buttons below will help you get started.\\n", shell_path),
        Err(_) => String::from("Could not determine shell.\\nWelcome to Athena OS!\\nThe buttons below will help you get started.\\n"),
    };
    buffer.set_text(&initial_shell_prompt);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&console_output));
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_min_content_height(400);

    let buttons_box = Box::new(Orientation::Horizontal, 10);
    buttons_box.set_halign(Align::Center);
    buttons_box.set_spacing(10);

    let package_update_button = Button::with_label("Package Update");
    package_update_button.add_css_class("rounded");
    buttons_box.append(&package_update_button);

    let htb_update_button = Button::with_label("HTB Update");
    htb_update_button.add_css_class("rounded");
    buttons_box.append(&htb_update_button);

    let set_cyber_role_button = Button::with_label("Set Cyber Role");
    set_cyber_role_button.add_css_class("rounded");
    buttons_box.append(&set_cyber_role_button);

    let update_mirrors_button = Button::with_label("Update Mirrors");
    update_mirrors_button.add_css_class("rounded");
    buttons_box.append(&update_mirrors_button);
    
    // --- Button Logic ---

    // Helper to append text to console and scroll
    let append_to_console_output = |text_buffer: &gtk::TextBuffer, tv: &gtk::TextView, text: &str| {
        let mut end_iter = text_buffer.end_iter();
        text_buffer.insert(&mut end_iter, text);
        
        // Get end iterator again after insert for scrolling
        let mut scroll_iter = text_buffer.end_iter(); 
        tv.scroll_to_iter(&mut scroll_iter, 0.0, true, 0.0, 1.0);
    };

    // Update Mirrors Button
    let buffer_mirrors = buffer.clone();
    let console_output_mirrors = console_output.clone();
    let append_clone_mirrors = append_to_console_output.clone();
    let update_mirrors_button_clone = update_mirrors_button.clone();
    let detected_pkg_manager_mirrors = detect_package_manager();
    let os_type_mirrors = os_type.clone();

    // Channel for communication between the command thread and the main GTK thread
    // The message will be the result of the command execution
    let (tx_cmd_result, rx_cmd_result) = async_channel::unbounded::<Result<std::process::Output, std::io::Error>>();

    update_mirrors_button.connect_clicked(move |_| {
        update_mirrors_button_clone.set_sensitive(false);
        let initial_message: String;
        let mut command_to_run_opt: Option<String> = None;

        match &os_type_mirrors {
            OsType::Arch => {
                if Command::new("reflector").arg("--version").output().is_ok() {
                    let cmd = "reflector --latest 5 --sort rate --save /etc/pacman.d/mirrorlist";
                    command_to_run_opt = Some(cmd.to_string());
                    initial_message = format!("Detected OS: Arch Linux.\nAttempting to update mirrors with reflector: {}\n", cmd);
                } else {
                    initial_message = "Detected OS: Arch Linux.\n'reflector' command not found. Please install it to enable mirror updates.\n".to_string();
                }
            }
            OsType::NixOS => {
                initial_message = "Detected OS: NixOS.\nMirror management in NixOS is typically handled by `nix-channel --update` and `nixos-rebuild switch` (available under 'Package Update').\nNo separate mirror update step is usually required.\n".to_string();
            }
            OsType::Other(id) => {
                match detected_pkg_manager_mirrors {
                    PackageManager::Pacman => {
                         if Command::new("reflector").arg("--version").output().is_ok() {
                            let cmd = "reflector --latest 5 --sort rate --save /etc/pacman.d/mirrorlist";
                            command_to_run_opt = Some(cmd.to_string());
                            initial_message = format!("Detected OS: {} (Pacman found).\nAttempting to update mirrors with reflector: {}\n", id, cmd);
                        } else {
                            initial_message = format!("Detected OS: {} (Pacman found).\n'reflector' command not found. Please install it to enable mirror updates.\n", id);
                        }
                    }
                    _ => {
                        initial_message = format!("Mirror update not configured for this OS ({}).\n", id);
                    }
                }
            }
            OsType::Unknown => {
                initial_message = "Could not determine OS. Mirror update not configured.\n".to_string();
            }
        }

        append_clone_mirrors(&buffer_mirrors, &console_output_mirrors, &initial_message);

        if let Some(command_to_run_str) = command_to_run_opt {
            let execution_prompt = format!(
                "Attempting to execute with pkexec: sudo {}\nThis may require you to enter your password in a graphical prompt.\nExecuting...",
                command_to_run_str
            );
            append_clone_mirrors(&buffer_mirrors, &console_output_mirrors, &execution_prompt);

            let sender_clone = tx_cmd_result.clone();
            let command_to_run_for_thread = command_to_run_str.clone();

            thread::spawn(move || {
                let command_output_result = Command::new("pkexec")
                    .arg("sh")
                    .arg("-c")
                    .arg(&format!("sudo {}", command_to_run_for_thread))
                    .output();
                // Use send_blocking as we are in a synchronous thread
                if sender_clone.send_blocking(command_output_result).is_err() {
                    // eprintln or some other logging if receiver is closed, though less likely here
                }
            });
            // The receiver part is handled outside connect_clicked, once, in build_ui
        } else {
            // No command to run, re-enable button immediately
            update_mirrors_button_clone.set_sensitive(true);
        }
    });

    // Receiver for command results - run this once
    let buffer_mirrors_recv = buffer.clone();
    let console_output_mirrors_recv = console_output.clone();
    let append_clone_mirrors_recv = append_to_console_output.clone();
    let update_mirrors_button_recv_clone = update_mirrors_button.clone();
    // We need to capture command_to_run_str for the error message if pkexec fails to start.
    // However, command_to_run_str is defined inside the connect_clicked closure.
    // This is tricky. We'll simplify for now: the error message for pkexec not found
    // won't include the command if we handle rx outside.
    // A more complex solution would involve sending the command string itself via the channel.
    // For now, the message is generic if pkexec itself fails.

    glib::MainContext::default().spawn_local(async move {
        while let Ok(command_output_result_from_thread) = rx_cmd_result.recv().await {
            match command_output_result_from_thread {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let status_message: String = if output.status.success() {
                        "Mirror update command executed successfully.".to_string()
                    } else {
                        format!("Mirror update command execution potentially failed or was cancelled (exit status: {}).", output.status)
                    };
                    let full_output_message = format!(
                        "{}\nStdout:\n{}\nStderr:\n{}\n",
                        status_message, stdout, stderr
                    );
                    append_clone_mirrors_recv(&buffer_mirrors_recv, &console_output_mirrors_recv, &full_output_message);
                }
                Err(e) => { // This error is if Command::new("pkexec")... itself failed (e.g., pkexec not found)
                    let error_message = format!("Failed to start command execution process (e.g., pkexec not found or permission issues).
Error: {}
Please ensure Polkit is installed and pkexec is in your PATH.
", e);
                    append_clone_mirrors_recv(&buffer_mirrors_recv, &console_output_mirrors_recv, &error_message);
                }
            }
            update_mirrors_button_recv_clone.set_sensitive(true); // Re-enable button
        }
    });

    // Set Cyber Role Button
    let buffer_roles = buffer.clone();
    let console_output_roles = console_output.clone();
    let list_box_roles = list_box.clone();
    let append_clone_roles = append_to_console_output.clone();
    set_cyber_role_button.connect_clicked(move |_| {
        let mut selected_ids = Vec::new();
        // Iterate directly over the children of the ListBox
        let mut current_child = list_box_roles.first_child();
        while let Some(child_widget) = current_child {
            if let Some(list_box_row) = child_widget.downcast_ref::<gtk::ListBoxRow>() {
                if let Some(check_button) = list_box_row.child().and_then(|w| w.downcast::<gtk::CheckButton>().ok()) {
                    if check_button.is_active() {
                        if let Some(label_text_gstr) = check_button.label() {
                            let label_text = label_text_gstr.as_str();
                            for (text, id) in ITEMS.iter() {
                                if *text == label_text {
                                    selected_ids.push(*id);
                                    break; // Found the ID for this active check_button
                                }
                            }
                        }
                    }
                }
            }
            current_child = child_widget.next_sibling();
        }

        let message = format!("Selected Role IDs: {:?}\n", selected_ids);
        append_clone_roles(&buffer_roles, &console_output_roles, &message);
    });

    // Package Update Button - Updated Logic
    let buffer_pkg = buffer.clone();
    let console_output_pkg = console_output.clone();
    let append_clone_pkg = append_to_console_output.clone();
    
    let detected_pkg_manager = detect_package_manager(); 
    let os_type_clone_for_fallback = os_type.clone(); 

    package_update_button.connect_clicked(move |_| {
        let mut command_to_run_opt: Option<String> = None;
        let initial_message: String;

        match &detected_pkg_manager {
            PackageManager::Pacman => {
                let cmd = "sudo pacman -Syu";
                command_to_run_opt = Some(cmd.to_string());
                initial_message = format!("Detected Package Manager: Pacman.\nUpdate command: {}\n", cmd);
            }
            PackageManager::Nix => {
                let cmd = "sudo nix-channel --update && sudo nixos-rebuild switch";
                command_to_run_opt = Some(cmd.to_string());
                initial_message = format!("Detected Package Manager: Nix.\nUpdate command: {}\n", cmd);
            }
            PackageManager::Unknown => {
                match &os_type_clone_for_fallback { 
                    OsType::Arch => {
                        let cmd = "sudo pacman -Syu";
                        command_to_run_opt = Some(cmd.to_string());
                        initial_message = format!("Could not definitively detect package manager, but OS is Arch-based.\nUpdate command: {}\n", cmd);
                    }
                    OsType::NixOS => {
                        let cmd = "sudo nix-channel --update && sudo nixos-rebuild switch";
                        command_to_run_opt = Some(cmd.to_string());
                        initial_message = format!("Could not definitively detect package manager, but OS is NixOS-based.\nUpdate command: {}\n", cmd);
                    }
                    OsType::Other(id) => {
                         initial_message = format!("Package manager not automatically detected for this OS ({}).\nPackage update not configured.\n", id);
                    }
                    OsType::Unknown => {
                        initial_message = "Package manager not automatically detected.\nPackage update not configured for Unknown OS.\n".to_string();
                    }
                }
            }
        }

        append_clone_pkg(&buffer_pkg, &console_output_pkg, &initial_message);

        if let Some(command_to_run) = command_to_run_opt {
            let execution_prompt = format!(
                "Attempting to execute with pkexec: {}\nThis may require you to enter your password in a graphical prompt.\nExecuting...",
                command_to_run
            );
            append_clone_pkg(&buffer_pkg, &console_output_pkg, &execution_prompt);

            match Command::new("pkexec").arg("sh").arg("-c").arg(&command_to_run).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let status_message: String = if output.status.success() {
                        "Command executed successfully.".to_string()
                    } else {
                        format!("Command execution potentially failed or was cancelled (exit status: {}).", output.status)
                    };
                    let full_output_message = format!(
                        "{}\nStdout:\n{}\nStderr:\n{}\n",
                        status_message, stdout, stderr
                    );
                    append_clone_pkg(&buffer_pkg, &console_output_pkg, &full_output_message);
                }
                Err(e) => {
                    let error_message = if e.kind() == std::io::ErrorKind::NotFound {
                        format!("pkexec command not found. Please ensure Polkit is installed and pkexec is in your PATH.\nCannot execute: {}\nPlease run it manually in a terminal.\n", command_to_run)
                    } else {
                        format!("Failed to start command with pkexec: {}\nError: {}\nCannot execute: {}\nPlease run it manually in a terminal.\n", command_to_run, e, command_to_run)
                    };
                    append_clone_pkg(&buffer_pkg, &console_output_pkg, &error_message);
                }
            }
        } 
        // If command_to_run_opt was None, initial_message (explaining why) was already printed.
    });

    // HTB Update Button
    let buffer_htb = buffer.clone();
    let console_output_htb = console_output.clone();
    let append_clone_htb = append_to_console_output.clone();
    htb_update_button.connect_clicked(move |_| {
        append_clone_htb(&buffer_htb, &console_output_htb, "HTB Update button clicked. API key setup would go here.\n");
    });

    // --- End Button Logic ---

    content.append(&description);

    let interactive_controls_box = Box::new(Orientation::Vertical, 15);
    interactive_controls_box.set_halign(Align::Center);
    interactive_controls_box.append(&select_role_button); // Changed from select_role
    interactive_controls_box.append(&buttons_box);
    content.append(&interactive_controls_box);
    
    content.append(&athena_pic);
    content.append(&scrolled_window);
    
    content
}

fn create_info_page() -> Box {
    let content = Box::new(Orientation::Vertical, 10);
    
    let info_text = 
    "
    <span>
    <b>Welcome to Athena OS</b> \n
    Choose your role an click th Set <b>Cyber Role</b> button ro retrive the main pentesting resources you need! \n
    Blick HTB Update to set your Hack The Box API key and start your hacking experience! \n
    Get started on Athena. We communicate with our community through <a href=\"https://discord.gg/athenaos\">Discord</a> or <a href=\"https://github.com/Athena-OS\">GitHub</a>.
    Join us to learn the latest news, ask questions or just for chatting.
    Open a ticket for any issues or proposals.\n
    Learn, study and have fun! Visit our <a href=\"https://athenaos.org\">website</a>.
    </span>
    ";
    
    let info_label = Label::new(None);
    info_label.set_markup(info_text);
    info_label.set_margin_top(10);
    info_label.set_wrap(true);
    info_label.set_justify(gtk::Justification::Center);
    info_label.set_halign(Align::Center); // Keep overall centering of the label block
    content.append(&info_label);

    let social_media_box = Box::new(Orientation::Horizontal, 15);
    social_media_box.set_halign(Align::Center);
    social_media_box.set_margin_top(20);

    // Discord
    let discord_icon = Picture::for_resource("/org/athenaos/welcome/images/discord.png");
    let discord_link = LinkButton::with_label("YOUR_DISCORD_URL_HERE", "Discord");
    let discord_button_box = Box::new(Orientation::Horizontal, 5);
    discord_button_box.append(&discord_icon);
    discord_button_box.append(&discord_link);
    social_media_box.append(&discord_button_box);

    // Instagram
    let instagram_icon = Picture::for_resource("/org/athenaos/welcome/images/insta.png");
    let instagram_link = LinkButton::with_label("YOUR_INSTAGRAM_URL_HERE", "Instagram");
    let instagram_button_box = Box::new(Orientation::Horizontal, 5);
    instagram_button_box.append(&instagram_icon);
    instagram_button_box.append(&instagram_link);
    social_media_box.append(&instagram_button_box);

    content.append(&social_media_box);
  
    
    content
}

fn create_credits_page() -> Box {
    let content = Box::new(Orientation::Vertical, 10);

    let credits_text = "
    <span>
    Athena OS Team
    Built by <a href=\"https://github.com/jakubGodula\">Jakub Godula</a> with <a href=\"https://gtk-rs.org/\">GTK Rust bindings</a>.
    </span>
    ";
    
    let credits_label = Label::new(None); // Changed to allow markup
    credits_label.set_markup(credits_text); // Set text as markup
    credits_label.set_margin_top(10);
    credits_label.set_wrap(true); // Add wrap property
    credits_label.set_justify(gtk::Justification::Center);
    credits_label.set_halign(Align::Center); // Keep overall centering of the label block
    content.append(&credits_label);
    
    content
}
