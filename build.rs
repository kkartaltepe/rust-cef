#![feature(env)]
use std::env;
use std::old_io::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut split = target.split('-');
    let arch = split.next().unwrap();
    let manufact = split.next().unwrap();
    let os = split.next().unwrap();
	println!("cargo:rustc-flags=-L ../CEF/lib/{}_{}", arch, os);
}
