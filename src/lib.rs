#![crate_type = "lib"]
#![allow(dead_code)]
#![allow(unstable)]
#![allow(missing_copy_implementations)]

extern crate libc;
use libc::{c_int, size_t};
use libc::types::common::c95::c_void;

use app::{App, Settings, CefString, MainArgs};
use browser::{WindowInfo, BrowserSettings, Client};

pub mod base;
pub mod app;
pub mod browser;


#[link(name="cef")]
extern "stdcall" { //On windows32 stdcall. on 64 use C?
	pub fn cef_execute_process(args: *const MainArgs, app: *mut App, win_sandbox_info: *mut c_int) -> c_int;
	pub fn cef_initialize(args: *const MainArgs, settings: *mut Settings, app: *mut App ) -> c_int;
	pub fn cef_browser_host_create_browser(windowInfo: *const WindowInfo, client: *mut Client, url: *const CefString, browserSettings: *const BrowserSettings, requestContext: *mut c_void ) -> c_int;
	pub fn cef_run_message_loop();
	pub fn cef_shutdown();
}