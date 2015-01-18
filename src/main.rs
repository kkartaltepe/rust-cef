#![crate_type = "bin"]
#![allow(unstable)]

extern crate rust_cef;
extern crate libc;

use libc::types::os::arch::c95::wchar_t;
use libc::types::common::c95::c_void;
#[allow(unused_imports)]
use libc::{c_int,c_uint};

use rust_cef::app::{App, Settings, CefString, MainArgs};
use rust_cef::browser::{WindowInfo, BrowserSettings, Client};
use std::default::Default;

#[link(name = "kernel32")]
extern "stdcall" {
    fn GetModuleHandleW(moduleName: *const wchar_t) -> *mut c_void;
}

fn main() {
	println!("Hello World");
	let hinst = unsafe { GetModuleHandleW(std::ptr::null_mut()) };
		
	let args = MainArgs {instance: hinst};
	let mut app = App{..Default::default()};

	println!("Cef execute process");
	let exec_ret = rust_cef::execute_process(&args, &mut app, std::ptr::null_mut());
	if exec_ret >= 0 {
		return;
	}

	let mut settings = Settings {
		no_sandbox: 1, 
		..Default::default()
	};

	println!("Cef initialize");
	rust_cef::initialize(&args, &mut settings, &mut app);


	let window_info = WindowInfo{..Default::default()};
	let mut client = Client{..Default::default()};
	let url = CefString::from("www.google.com");
	let browser_settings = BrowserSettings{..Default::default()};

	println!("Cef browser host create browser");
	rust_cef::browser_host_create_browser(&window_info, &mut client, &url, &browser_settings, std::ptr::null_mut() );

	println!("Cef run message loop");
	rust_cef::run_message_loop();

	println!("Cef shutdown");
	rust_cef::shutdown();	
}
