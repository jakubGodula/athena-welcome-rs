use gtk4::prelude::*;
use gtk4::{Application, Box, Label, MenuButton, Notebook, Orientation, Align, Picture, Image, ListBox, ListBoxRow, CheckButton, SelectionMode, Popover, Button, Justification};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::path::Path;
use crate::utils::{OsType, resize_image};
use crate::roles::{ITEMS, CURRENT_ROLES, toggle_role, save_roles};


pub static CURRENT_PAGE: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

pub fn create_welcome_page(app: &Application, os_type: &OsType) -> Box {
    let centered_box = Box::new(Orientation::Vertical, 0);
    centered_box.set_vexpand(true);
    centered_box.set_hexpand(true);
    centered_box.set_valign(Align::Center);
    centered_box.set_halign(Align::Center);

    let content = Box::new(Orientation::Vertical, 10);
    content.set_vexpand(false);
    content.set_hexpand(false);
    content.set_valign(Align::Center);
    content.set_halign(Align::Center);
    
    let welcome_text = "<span><b>This is the welcome page of Athena OS.</b></span>";
    let description = Label::new(None);
    description.set_markup(welcome_text);
    description.set_halign(Align::Center);
    content.append(&description);

    let role_box = Box::new(Orientation::Vertical, 10);
    role_box.set_halign(Align::Center);
    role_box.set_valign(Align::Center);
    role_box.set_margin_start(20);
    role_box.set_margin_end(20);
    role_box.set_margin_top(20);
    role_box.set_margin_bottom(20);
    content.append(&role_box);
    // ListBox is a GTK4 widget that displays a list of items in a vertical layout.
    // It's commonly used for creating scrollable lists of selectable items.
    // In this case, it's being used to display a list of role checkboxes that users can select.
    // The list is set to allow multiple selections and has a fixed width of 300 pixels.
    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::Multiple);
    list_box.set_size_request(300, -1);
    list_box.set_halign(Align::Center);

    // Get a fresh copy of current roles
    let current_roles = CURRENT_ROLES.lock().unwrap().clone();

    for (item_text, item_id) in ITEMS.iter() {
        let check_button = CheckButton::with_label(item_text);
        check_button.set_halign(Align::Center);
        
        // Set initial state based on current roles
        let is_active = current_roles.contains(&item_id.to_string());
        check_button.set_active(is_active);
        
        // Connect the toggle signal
        let item_id = item_id.to_string();
        check_button.connect_toggled(move |button| {
            toggle_role(item_id.clone());
            // Save roles after toggling
            if let Err(e) = save_roles(Path::new("/etc/athena-welcome/roles.conf")) {
                eprintln!("Failed to save roles: {}", e);
            }
        });
        
        let row = ListBoxRow::new();
        row.set_child(Some(&check_button));
        list_box.append(&row);
    }

    let select_role_button = MenuButton::new();
    select_role_button.set_label("🔥 Choose your Role 🔥");
    select_role_button.set_halign(Align::Center);

    let popover = Popover::new();
    popover.add_css_class("role-popover");
    popover.set_child(Some(&list_box));
    popover.set_has_arrow(true);
    popover.set_autohide(true);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.set_offset(0, 10);

    // Set the popover on the button
    select_role_button.set_popover(Some(&popover));
    role_box.append(&select_role_button);

    let image_path = match os_type {
        OsType::Arch => "/org/athenaos/welcome/images/athena-arch-one-liner.png",
        OsType::NixOS => "/org/athenaos/welcome/images/athena-nix-one-liner.png",
        _ => "/org/athenaos/welcome/images/athena-arch-one-liner.png",
    };
    
    // Create welcome image
    let athena_pic = Picture::for_resource(image_path);
    athena_pic.set_halign(Align::Center);
    athena_pic.add_css_class("welcome-image");
    content.append(&athena_pic);

    let buttons_box = Box::new(Orientation::Horizontal, 10);
    buttons_box.set_halign(Align::Center);
    buttons_box.set_spacing(10);
    buttons_box.set_margin_top(20);
    buttons_box.set_size_request(-1, 75);
    buttons_box.set_valign(Align::Center);

    let package_update_button = Button::with_label("Package Update");
    package_update_button.add_css_class("rounded");
    buttons_box.append(&package_update_button);

    let htb_button_box = Box::new(Orientation::Horizontal, 5);
    htb_button_box.set_valign(Align::Center);
    
    let htb_icon_container = Box::new(Orientation::Horizontal, 0);
    htb_icon_container.set_size_request(8, 8);
    htb_icon_container.set_halign(Align::Center);
    htb_icon_container.set_valign(Align::Center);
    
    // Create HTB icon
    let htb_icon = Picture::for_resource("/org/athenaos/welcome/images/htb.png");
    htb_icon.add_css_class("htb-icon");
    htb_icon_container.append(&htb_icon);
    
    let htb_label = Label::new(Some("HTB Update"));
    htb_button_box.append(&htb_icon_container);
    htb_button_box.append(&htb_label);
    
    let htb_update_button = Button::new();
    htb_update_button.set_child(Some(&htb_button_box));
    htb_update_button.add_css_class("rounded");
    buttons_box.append(&htb_update_button);

    let set_cyber_role_button = Button::with_label("Set Cyber Role");
    set_cyber_role_button.add_css_class("rounded");
    buttons_box.append(&set_cyber_role_button);

    let update_mirrors_button = Button::with_label("Update Mirrors");
    update_mirrors_button.add_css_class("rounded");
    buttons_box.append(&update_mirrors_button);
    
    content.append(&buttons_box);
    centered_box.append(&content);
    
    centered_box
}

pub fn create_info_page() -> Box {
    let content = Box::new(Orientation::Vertical, 10);
    
    let info_text = "<span><b>Welcome to Athena OS</b> \nChoose your role an click th Set <b>Cyber Role</b> button ro retrive the main pentesting resources you need! \nBlick HTB Update to set your Hack The Box API key and start your hacking experience! \nGet started on Athena. We communicate with our community through <a href=\"https://discord.gg/athenaos\">Discord</a> or <a href=\"https://github.com/Athena-OS\">GitHub</a>. Join us to learn the latest news, ask questions or just for chatting. Open a ticket for any issues or proposals.\nLearn, study and have fun! Visit our <a href=\"https://athenaos.org\">website</a>.</span>";
    
    let info_label = Label::new(None);
    info_label.set_markup(info_text);
    info_label.set_margin_top(10);
    info_label.set_wrap(true);
    info_label.set_justify(Justification::Center);
    info_label.set_halign(Align::Center);
    content.append(&info_label);
    
    content
}

pub fn create_credits_page() -> Box {
    let content = Box::new(Orientation::Vertical, 10);

    let credits_text = "<span>Athena OS Team\nBuilt by <a href=\"https://github.com/jakubGodula\">Jakub Godula</a> with <a href=\"https://gtk-rs.org/\">GTK Rust bindings</a>.</span>";
    
    let credits_label = Label::new(None);
    credits_label.set_markup(credits_text);
    credits_label.set_margin_top(10);
    credits_label.set_wrap(true);
    credits_label.set_justify(Justification::Center);
    credits_label.set_halign(Align::Center);
    content.append(&credits_label);
    
    content
} 