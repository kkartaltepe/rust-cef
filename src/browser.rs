extern crate libc;

use app::CefString;
use base::CefBase;
use self::libc::size_t;
use libc::types::common::c95::c_void;

use std;
use std::mem;

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

impl Client {
	pub fn default() -> Client {
		return Client {
			base: CefBase::get::<Client>(),
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

#[repr(C)]
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

impl BrowserSettings {
    pub fn default() -> BrowserSettings {
        return BrowserSettings {
	        size: mem::size_of::<Self>() as size_t,
	        standard_font_family: CefString::empty(),
	        fixed_font_family: CefString::empty(),
	        serif_font_family: CefString::empty(),
	        sans_serif_font_family: CefString::empty(),
	        cursive_font_family: CefString::empty(),
	        fantasy_font_family: CefString::empty(),
	        default_font_size: 0i32,
	        default_fixed_font_size: 0i32,
	        minimum_font_size: 0i32,
	        minimum_logical_font_size: 0i32,
	        default_encoding: CefString::empty(),
	        remote_fonts: CefState::Default,
	        javascript: CefState::Default,
	        javascript_open_windows: CefState::Default,
	        javascript_close_windows: CefState::Default,
	        javascript_access_clipboard: CefState::Default,
	        javascript_dom_paste: CefState::Default,
	        caret_browsing: CefState::Default,
	        java: CefState::Default,
	        plugins: CefState::Default,
	        universal_access_from_file_urls: CefState::Default,
	        file_access_from_file_urls: CefState::Default,
	        web_security: CefState::Default,
	        image_loading: CefState::Default,
	        image_shrink_standalone_to_fit: CefState::Default,
	        text_area_resize: CefState::Default,
	        tab_to_links: CefState::Default,
	        local_storage: CefState::Default,
	        databases: CefState::Default,
	        application_cache: CefState::Default,
	        webgl: CefState::Default,
	        background_color: 0i32
        }
    }
}
