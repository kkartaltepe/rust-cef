extern crate libc;

use app::CefString;
use base::CefBase;
use self::libc::size_t;
use libc::types::common::c95::c_void;

use std;
use std::default::Default;
use std::mem;

#[repr(C)]
pub struct WindowInfo {
	ex_style: u32,
	window_name: CefString,
	style: u32,
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	parent_window: *const c_void,
	menu: *const c_void,
	windowless_rendering_enabled: libc::c_int,
	transparent_painting_enabled: libc::c_int,
	window: *const c_void
}
impl Default for WindowInfo {
	#[allow(non_snake_case)]
	fn default() -> Self {
		let CW_USEDEFAULT = 0x80000000u32;
		let WS_CLIPCHILDREN = 0x02000000u32;
		let WS_CLIPSIBLINGS = 0x04000000u32;
		let WS_VISIBLE = 0x10000000u32;
		let WS_THINTITLEWINDOW =  0x00C00000u32 | 0x00080000u32 | 0x00040000u32 | 0x00010000u32 | 0x00020000u32; // 0x00080000;
		let window_style = WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_VISIBLE | WS_THINTITLEWINDOW;
		return WindowInfo {
				ex_style: 0u32,
				window_name: Default::default(),
				style: window_style,
				x: CW_USEDEFAULT,
				y: CW_USEDEFAULT,
				width: CW_USEDEFAULT,
				height: CW_USEDEFAULT,
				parent_window: std::ptr::null(),
				menu: std::ptr::null(),
				windowless_rendering_enabled: 0i32,
				transparent_painting_enabled: 0i32,
				window: std::ptr::null()
		}
	}
}


#[repr(C)]
struct WindowInfoLinux {
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	parent_window: libc::c_int,
	windowless_rendering_enabled: libc::c_int,
	transparent_painting_enabled: libc::c_int,
	window: libc::c_int
}

#[repr(C)]
struct ProcessMessage {
	base: CefBase,
	is_valid: extern fn(this: *mut ProcessMessage) -> i32,
	is_read_only: extern fn(this: *mut ProcessMessage) -> i32,
	copy_of: extern fn(this: *mut ProcessMessage) -> *mut ProcessMessage,
	get_name : extern fn(this: *mut ProcessMessage) -> *mut CefString, //Must be freed by caller
	get_argument_list: extern fn(this: *mut ProcessMessage) -> libc::c_int
}

#[repr(C)]
pub struct Client {
	base: CefBase,
	get_context_menu_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_dialog_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_display_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_download_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_drag_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_find_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_focus_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_geolocation_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_jsdialog_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_keyboard_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_life_span_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_load_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_render_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_request_handler: extern fn(this: *mut Client) -> *mut c_void,
	on_process_message_received: extern fn(this: *mut Client, browser: libc::c_int, source_process: i32, message: *mut ProcessMessage) -> i32
}

#[allow(unused_variables)]
extern fn nop_handler (this: *mut Client) -> *mut c_void {
	return std::ptr::null_mut();
}

#[allow(unused_variables)]
extern fn nop_processor (this: *mut Client, browser: libc::c_int, source_process: i32, message: *mut ProcessMessage) -> i32 {
	return 0;
}

impl Default for Client {
	fn default() -> Self {
		return Client {
			base: CefBase {
				size: (mem::size_of::<libc::c_int>()*19) as libc::size_t,
				..Default::default()
			},
			get_context_menu_handler: nop_handler,
			get_dialog_handler: nop_handler,
			get_display_handler: nop_handler,
			get_download_handler: nop_handler,
			get_drag_handler: nop_handler,
			get_find_handler: nop_handler,
			get_focus_handler: nop_handler,
			get_geolocation_handler: nop_handler,
			get_jsdialog_handler: nop_handler,
			get_keyboard_handler: nop_handler,
			get_life_span_handler: nop_handler,
			get_load_handler: nop_handler,
			get_render_handler: nop_handler,
			get_request_handler: nop_handler,
			on_process_message_received: nop_processor
		}
	}
}


#[repr(C)]
enum CefState {
	Default = 0,
	Enabled,
	Disabled
}

impl Default for CefState {
	fn default() -> CefState {
		return CefState::Default;
	}
}

#[repr(C)]
#[derive(Default)]
pub struct BrowserSettings {
	size: size_t,
	standard_font_family: CefString,
	fixed_font_family: CefString,
	serif_font_family: CefString,
	sans_serif_font_family: CefString,
	cursive_font_family: CefString,
	fantasy_font_family: CefString,
	default_font_size: i32,
	default_fixed_font_size: i32,
	minimum_font_size: i32,
	minimum_logical_font_size: i32,
	default_encoding: CefString,
	remote_fonts: CefState,
	javascript: CefState,
	javascript_open_windows: CefState,
	javascript_close_windows: CefState,
	javascript_access_clipboard: CefState,
	javascript_dom_paste: CefState,
	caret_browsing: CefState,
	java: CefState,
	plugins: CefState,
	universal_access_from_file_urls: CefState,
	file_access_from_file_urls: CefState,
	web_security: CefState,
	image_loading: CefState,
	image_shrink_standalone_to_fit: CefState,
	text_area_resize: CefState,
	tab_to_links: CefState,
	local_storage: CefState,
	databases: CefState,
	application_cache: CefState,
	webgl: CefState,
	background_color: i32
}