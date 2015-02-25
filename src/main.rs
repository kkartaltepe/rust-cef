#![crate_type = "bin"]
#![feature(libc,std_misc)]

extern crate rust_cef;
extern crate libc;

use rust_cef::app::{App, Settings, CefString};
use rust_cef::browser::{BrowserSettings, Client};
use rust_cef::platform::linux::{MainArgs, WindowInfo};

fn main() {
	println!("Hello World");
		
	let args = MainArgs::get();
	let mut app = App::new();
    check_args(args);

	println!("Cef execute process");
	let exec_ret = rust_cef::execute_process(&args, &mut app, std::ptr::null_mut());
    println!("huh whats this? {} ", exec_ret);
	if exec_ret >= 0 {
		return;
	}

	let mut settings = Settings::default();
	println!("Settings size is : {}", settings.size);
	

	println!("Cef initialize");
	let init_ret = rust_cef::initialize(&args, &mut settings, &mut app);
	if init_ret != 1 {
        println!("Init failed with {}", init_ret);
        return;
	}


	let window_info = WindowInfo::default();
	let mut client = Client::default();
	let url = CefString::from("www.google.com");
	let browser_settings = BrowserSettings::default();

	println!("Cef browser host create browser");
	rust_cef::browser_host_create_browser(&window_info, &mut client, &url, &browser_settings, std::ptr::null_mut() );

	println!("Cef run message loop");
	rust_cef::run_message_loop();

	println!("Cef shutdown");
	rust_cef::shutdown();	
}

fn check_args(args: MainArgs) {
    println!("{} arguments to this thread", args.argc);
    let argv_slice = unsafe { std::slice::from_raw_parts(args.argv, args.argc as usize) };
    for &c_string_p in argv_slice {
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(c_string_p);
            let strslice = std::str::from_utf8(c_str.to_bytes()).unwrap();
            println!("Argument: '{}'", strslice);
        }
    }

}
