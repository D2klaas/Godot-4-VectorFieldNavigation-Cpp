use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let profile = env::var("PROFILE").unwrap(); // "debug" oder "release"
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Dein Crate-Name = Dateiname der DLL
    let dll_name = if cfg!(target_os = "windows") {
        "vector_field_navigation.dll"
    } else if cfg!(target_os = "linux") {
        "libvector_field_navigation.so"
    } else {
        panic!("Unsupported platform");
    };

    // Zielpfad relativ zum Cargo-Projektverzeichnis
    let output_path = manifest_dir
        .parent().unwrap()      // .. (aus /Rust)
        .join("godot")          // ../godot
        .join("addons")           // ../godot/bin
        .join("vector_field_navigation_rust")           // ../godot/bin
        .join("gdnative")           // ../godot/bin
        .join("bin")           // ../godot/bin
        .join("windows");           // ../godot/bin

    // Pfad zur erzeugten Datei
    let build_path = manifest_dir
        .join("target")
        .join(&profile)
        .join(&dll_name);

    println!("cargo:rerun-if-changed=build.rs");

    if !build_path.exists() {
        panic!("Konnte DLL nicht finden: {:?}", build_path);
    }

    fs::create_dir_all(&output_path).unwrap();
    fs::copy(&build_path, &output_path.join(&dll_name)).expect("DLL konnte nicht kopiert werden");

    println!("→ DLL erfolgreich nach {:?} kopiert", output_path);
}
