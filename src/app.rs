extern crate libc;

use base::CefBase;
use libc::{c_int, size_t};
use libc::types::common::c95::c_void;
use std::default::Default;
use std::mem;
use std;

//TODO: move to lib.rs somehow.
#[link="cef"]
extern "stdcall" {
	fn cef_string_utf8_to_utf16(src: *const u8, src_len: size_t, cef_string: *mut CefString) -> c_int;
}

#[repr(C)]
pub struct CefString { //UTF 8/16
	str_ptr: *const u8,
	length: size_t,
	dtor: extern fn(strPtr:libc::c_int)
}

#[allow(unused_variables)]
extern fn nop_string(ptr: libc::c_int) {
	return
}

#[allow(unused_variables)]
extern fn nop_app(ptr: *mut App) {
	return
}

#[allow(unused_variables)]
extern fn nop_app_ptr(ptr: *mut App) -> libc::c_int{
	return 0;
}

impl Default for CefString {
	fn default() -> Self {
		return CefString::empty();
	}
}

impl CefString {
	pub fn from(string: &'static str) -> CefString {
		let mut cef_string = CefString{..Default::default()};
		unsafe { cef_string_utf8_to_utf16(string.as_ptr(), string.len() as size_t, &mut cef_string); }
		return cef_string;
	}

	pub fn empty() -> CefString {
		return CefString {
			str_ptr: std::ptr::null(),
			length: 0,
			dtor: nop_string
		}
	}
}

#[repr(C)]
pub struct App {
    base: CefBase,
    on_before_command_line_processing: extern fn(this: *mut App),
    on_register_custom_schemes: extern fn(this: *mut App),
    get_resource_bundle_handler: extern fn(this: *mut App)-> libc::c_int,
    get_browser_process_handler: extern fn(this: *mut App)-> libc::c_int,
    get_render_process_handler: extern fn(this: *mut App) -> libc::c_int
}

impl Default for App {
	fn default() -> Self {
		return App {
			base: CefBase::get::<App>(),
			on_before_command_line_processing: nop_app,
		    on_register_custom_schemes: nop_app,
		    get_resource_bundle_handler: nop_app_ptr,
		    get_browser_process_handler: nop_app_ptr,
		    get_render_process_handler: nop_app_ptr
		}
	}
}

#[repr(C)]
pub struct MainArgsLinux {
	pub argc: libc::c_int,
	pub argv: libc::c_int
}

#[repr(C)]
pub struct MainArgs {
	pub instance: *mut c_void
}

// To enable the sandbox on Windows the following requirements must be met:
// 1. Use the same executable for the browser process and all sub-processes.
// 2. Link the executable with the cef_sandbox static library.
// 3. Call the cef_sandbox_info_create() function from within the executable
//    (not from a separate DLL) and pass the resulting pointer into both the
//    CefExecutProcess() and CefInitialize() functions via the
//    |windows_sandbox_info| parameter.

#[repr(C)]
pub struct Settings {
	size: size_t,
	single_process: libc::c_int,
	pub no_sandbox: libc::c_int,
	browser_subprocess_path: CefString,
	multi_threaded_message_loop: libc::c_int,
	windowless_rendering_enabled: libc::c_int,
	command_line_args_disabled: libc::c_int,
	cache_path: CefString,
	persist_session_cookies: libc::c_int,
	user_agent: CefString,
	product_version: CefString,
	locale: CefString,
	log_file: CefString,
	log_severity: libc::c_int,
	javascript: CefString,
	reources_dir_path: CefString,
	locales_dir_path: CefString,
	pack_loading_disabled: libc::c_int,
	remote_debugging_port: libc::c_int,
	uncaught_exception_stack_size: libc::c_int,
	context_safety_implementation: libc::c_int,
	ignore_certificate_errors: libc::c_int,
	background_color: u32
}

impl Default for Settings {
	fn default() -> Self {
		return Settings {
			size: mem::size_of::<Self>() as size_t,
			single_process: 0,
			no_sandbox: 0,
			browser_subprocess_path: Default::default(),
			multi_threaded_message_loop: 0,
			windowless_rendering_enabled: 0,
			command_line_args_disabled: 0,
			cache_path: Default::default(),
			persist_session_cookies: 0,
			user_agent: Default::default(),
			product_version: Default::default(),
			locale: Default::default(),
			log_file: Default::default(),
			log_severity: 0, //Default, Verbose, Info, Warning, Error, 99=disabled
			javascript: Default::default(),
			reources_dir_path: Default::default(),
			locales_dir_path: Default::default(),
			pack_loading_disabled: 0,
			remote_debugging_port: 0,
			uncaught_exception_stack_size: 0,
			context_safety_implementation: 0,
			ignore_certificate_errors: 0, //Consider 1?
			background_color: 0u32
		}
	}
}
