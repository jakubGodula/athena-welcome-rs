use std::fs;
use std::io::{BufRead, BufReader, Cursor};
use std::process::Command;
use std::path::Path;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use gtk4::glib::Bytes;
use gtk4::gio;

#[derive(Debug, Clone, PartialEq)]
pub enum OsType {
    Arch,
    NixOS,
    Other(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackageManager {
    Pacman,
    Nix,
    Other(String),
    Unknown,
}

pub fn detect_os(path: &std::path::Path) -> OsType {
    if let Ok(file) = fs::File::open(path) {
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

pub fn detect_package_manager(os_type: &OsType) -> PackageManager {
    match os_type {
        OsType::Arch => PackageManager::Pacman,
        OsType::NixOS => PackageManager::Nix,
        OsType::Other(name) => PackageManager::Other(name.clone()),
        OsType::Unknown => PackageManager::Unknown,
    }
}

pub fn resize_image(resource_path: &str, max_width: u32, max_height: u32) -> Option<Bytes> {
    // Get the resource
    let resource = gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE).ok()?;
    
    // Create an image reader from the resource data
    let reader = ImageReader::new(Cursor::new(resource.as_ref()))
        .with_guessed_format()
        .ok()?;
    
    // Decode the image
    let img = reader.decode().ok()?;
    
    // Calculate new dimensions while maintaining aspect ratio
    let (width, height) = img.dimensions();
    let ratio = width as f32 / height as f32;
    
    let (new_width, new_height) = if width > height {
        let new_width = max_width.min(width);
        let new_height = (new_width as f32 / ratio) as u32;
        (new_width, new_height)
    } else {
        let new_height = max_height.min(height);
        let new_width = (new_height as f32 * ratio) as u32;
        (new_width, new_height)
    };
    
    // Resize the image
    let resized = img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
    
    // Convert to PNG bytes
    let mut cursor = Cursor::new(Vec::new());
    resized.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
    
    // Convert to GTK Bytes
    Some(Bytes::from_owned(cursor.into_inner()))
} 