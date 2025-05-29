use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Notebook, Label, Box, Orientation, Align, Picture, ListBox, ListBoxRow, CheckButton, SelectionMode, MenuButton, Popover, Button, CssProvider, LinkButton, TextView, ScrolledWindow};
use std::fs;
use std::io::{BufRead, BufReader};
use std::env;

static ITEMS: [(&str, i32); 11] = [
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
    // ("🕸️ Web Pentester 🕸️", 11) - User removed this, ensure array size is 11
];

#[derive(Debug, Clone)]
enum OsType {
    Arch,
    NixOS,
    Other(String),
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

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.athenaos.welcome")
        .build();

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
        OsType::Arch => "images/athena-arch-one-liner.png",
        OsType::NixOS => "images/athena-nix-one-liner.png",
        _ => "images/athena-arch-one-liner.png",
    };
    let athena_pic = Picture::for_filename(image_path);
    
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

    let package_update_button = Button::with_label("Package update");
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
    update_mirrors_button.connect_clicked(move |_| {
        append_clone_mirrors(&buffer_mirrors, &console_output_mirrors, "Updating Mirrors...\n");
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

    // Package Update Button
    let buffer_pkg = buffer.clone();
    let console_output_pkg = console_output.clone();
    let os_type_pkg = os_type.clone();
    let append_clone_pkg = append_to_console_output.clone();
    package_update_button.connect_clicked(move |_| {
        let message: String;
        match &os_type_pkg {
            OsType::Arch => {
                let command_to_run = "sudo pacman -Syu";
                message = format!("Command for Arch Linux update: {}\nConsider running this in a terminal.\n", command_to_run);
            }
            OsType::NixOS => {
                let command_to_run = "sudo nix-channel --update && sudo nixos-rebuild switch";
                message = format!("Command for NixOS update: {}\nConsider running this in a terminal.\n", command_to_run);
            }
            OsType::Other(id) => {
                message = format!("Package update not configured for this OS ({}).\n", id);
            }
            OsType::Unknown => {
                message = "Package update not configured for Unknown OS.\n".to_string();
            }
        }
        append_clone_pkg(&buffer_pkg, &console_output_pkg, &message);
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
    let discord_icon = Picture::for_filename("images/discord.png");
    let discord_link = LinkButton::with_label("YOUR_DISCORD_URL_HERE", "Discord");
    let discord_button_box = Box::new(Orientation::Horizontal, 5);
    discord_button_box.append(&discord_icon);
    discord_button_box.append(&discord_link);
    social_media_box.append(&discord_button_box);

    // Instagram
    let instagram_icon = Picture::for_filename("images/insta.png");
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
