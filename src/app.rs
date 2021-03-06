extern crate libc;

use base::CefBase;
use libc::{c_int, size_t};
use std::mem;
use std;

//TODO: move to lib.rs somehow.
#[cfg(target_os = "windows")]
#[link="cef"]
extern "stdcall" {
    fn cef_string_utf8_to_utf16(src: *const u8, src_len: size_t, cef_string: *mut CefString) -> c_int;
}
#[cfg(target_os = "linux")]
#[link="cef"]
extern "C" {
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


impl CefString {
    pub fn empty() -> CefString {
        return CefString {
            str_ptr: std::ptr::null(),
            length: 0,
            dtor: nop_string
        }
    }

    pub fn from(string: &'static str) -> CefString {
        let mut cef_string  = CefString::empty();
        unsafe { cef_string_utf8_to_utf16(string.as_ptr(), string.len() as size_t, &mut cef_string); }
        return cef_string;
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

impl App {
    pub fn new() -> App {
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


// To enable the sandbox on Windows the following requirements must be met:
// 1. Use the same executable for the browser process and all sub-processes.
// 2. Link the executable with the cef_sandbox static library.
// 3. Call the cef_sandbox_info_create() function from within the executable
//    (not from a separate DLL) and pass the resulting pointer into both the
//    CefExecutProcess() and CefInitialize() functions via the
//    |windows_sandbox_info| parameter.

#[repr(C)]
pub struct Settings {
    pub size: size_t,
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

impl Settings {
   pub fn default() -> Settings {
        return Settings {
            size: mem::size_of::<Settings>() as size_t,
            single_process: 0,
            no_sandbox: 1, //Default for testing since sandbox doesnt work on windows
            browser_subprocess_path: CefString::empty(),
            multi_threaded_message_loop: 0,
            windowless_rendering_enabled: 0,
            command_line_args_disabled: 0,
            cache_path: CefString::empty(),
            persist_session_cookies: 0,
            user_agent: CefString::empty(),
            product_version: CefString::empty(),
            locale: CefString::empty(),
            log_file: CefString::empty(),
            log_severity: 0, //Default, Verbose, Info, Warning, Error, 99=disabled
            javascript: CefString::empty(),
            reources_dir_path: CefString::from("/home/kk/projects/CEF/resources"),
            locales_dir_path: CefString::from("/home/kk/projects/CEF/resources/locales"),
            pack_loading_disabled: 0,
            remote_debugging_port: 0,
            uncaught_exception_stack_size: 0,
            context_safety_implementation: 0,
            ignore_certificate_errors: 0, //Consider 1?
            background_color: 0u32
        }
    }
}

