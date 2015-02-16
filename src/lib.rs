#![crate_type = "lib"]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]
#![feature(libc)]

extern crate libc;
use libc::c_int;
use libc::types::common::c95::c_void;

use app::{App, Settings, CefString, MainArgs};
use browser::{WindowInfo, BrowserSettings, Client};

pub mod base;
pub mod app;
pub mod browser;


#[link(name="cef")]
extern "stdcall" { //On windows32 stdcall. on 64 use C?
	fn cef_execute_process(args: *const MainArgs, app: *mut App, win_sandbox_info: *mut c_int) -> c_int;
	fn cef_initialize(args: *const MainArgs, settings: *mut Settings, app: *mut App ) -> c_int;
	fn cef_browser_host_create_browser(window_info: *const WindowInfo, client: *mut Client, url: *const CefString, browser_settings: *const BrowserSettings, request_context: *mut c_void ) -> c_int;
	fn cef_run_message_loop();
	fn cef_shutdown();
}

pub fn execute_process(args: *const MainArgs, app: *mut App, win_sandbox_info: *mut c_int) -> c_int {
	unsafe { return cef_execute_process(args, app, win_sandbox_info); }
}
pub fn initialize(args: *const MainArgs, settings: *mut Settings, app: *mut App ) -> c_int {
	unsafe { return cef_initialize(args, settings, app); }
}
pub fn browser_host_create_browser(window_info: *const WindowInfo, client: *mut Client, url: *const CefString, browser_settings: *const BrowserSettings, request_context: *mut c_void ) -> c_int {
	unsafe { return cef_browser_host_create_browser(window_info, client, url, browser_settings, request_context); }
}
pub fn run_message_loop(){
	unsafe { cef_run_message_loop(); }
}
pub fn shutdown(){
	unsafe { cef_shutdown() };
}
