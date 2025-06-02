extern crate glib_build_tools;
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // The source directory for glib-compile-resources.
    // Since your .gresource.xml lists paths like "images/discord.png",
    // and your images folder is at the project root, manifest_dir (project root) is the correct source_dir.
    let source_search_dir = manifest_dir.clone();

    glib_build_tools::compile_resources(
        &[source_search_dir.to_str().unwrap()],
        manifest_dir.join("resources.gresource.xml").to_str().unwrap(),
        out_dir.join("compiled_resources.rs").to_str().unwrap(),
    );

    println!("cargo:rerun-if-changed=resources.gresource.xml");
    // Tell cargo to rerun if any images change.
    // A simple way is to depend on the directory, or list each file.
    println!("cargo:rerun-if-changed=images/athena-arch-one-liner.png");
    println!("cargo:rerun-if-changed=images/athena-nix-one-liner.png");
    println!("cargo:rerun-if-changed=images/discord.png");
    println!("cargo:rerun-if-changed=images/insta.png");
} 