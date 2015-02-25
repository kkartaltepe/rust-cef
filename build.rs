#![feature(io)]
use std::old_io::process::Command;

fn main() {
    add_platform_flags();
}

#[cfg(target_os = "linux")]
fn add_platform_flags() {
    println!("cargo:rustc-flags=-L ../CEF/lib/x64_linux");
}

#[cfg(target_os = "windows")]
fn add_platform_flags() {
    println!("cargo:rustc-flags=-L ../CEF/lib/x64_windows");
}
