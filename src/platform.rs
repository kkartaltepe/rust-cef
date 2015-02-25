extern crate libc;


#[cfg(target_os = "windows")]
pub mod windows {
    use std;
    use libc::types::os::arch::c95::wchar_t;
    use libc::types::common::c95::c_void;
    use libc::{c_int, size_t};

    use app::CefString;

    #[link(name = "kernel32")]
    extern "stdcall" {
        fn GetModuleHandleW(moduleName: *const wchar_t) -> *mut c_void;
    }

    #[repr(C)]
    pub struct MainArgs {
        instance: *mut c_void
    }

    impl MainArgs {
        pub fn get() -> MainArgs {
            let hinst = unsafe { GetModuleHandleW(std::ptr::null_mut()) };
            return MainArgs { instance: hinst }
        }
    }

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
	    windowless_rendering_enabled: c_int,
	    transparent_painting_enabled: c_int,
	    window: *const c_void
    }
    impl WindowInfo {
	    #[allow(non_snake_case)]
	    pub fn default() -> Self {
		    let CW_USEDEFAULT = 0x80000000u32;
		    let WS_CLIPCHILDREN = 0x02000000u32;
		    let WS_CLIPSIBLINGS = 0x04000000u32;
		    let WS_VISIBLE = 0x10000000u32;
		    let WS_TILEDWINDOW =  0x00C00000u32 | 0x00080000u32 | 0x00040000u32 | 0x00010000u32 | 0x00020000u32;
		    let window_style = WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_VISIBLE | WS_TILEDWINDOW;
		    return WindowInfo {
				    ex_style: 0u32,
				    window_name: CefString::from("Rust CEF"),
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
}

#[cfg(target_os = "linux")]
pub mod linux {
    use std;
    use libc;
    use libc::{c_int,c_char};
    use std::env;
    use std::mem;
    use std::ptr;


    #[repr(C)]
    #[derive(Copy)]
    pub struct MainArgs {
       pub argc: c_int,
       pub argv: *const *const c_char
    }

    impl MainArgs {
        pub fn get() -> MainArgs {
            let args = env::args();
            let mut container: Vec<*const c_char> = Vec::with_capacity(args.len());
            for arg in  args {
                unsafe { // Unsafe copy but no one should play with this but CEF
                    let c_string = std::ffi::CString::new(arg.as_slice()).unwrap();
                    let c_string_len: libc::size_t = (arg.len()+1) as libc::size_t;
                    let size : libc::size_t = mem::size_of::<c_char>() as libc::size_t * c_string_len;
                    let string_p: *mut c_char = libc::malloc(size) as *mut c_char;
                    ptr::copy_memory(string_p, c_string.as_ptr(), c_string_len as usize);
                    container.push(string_p);
                }
            }
            return MainArgs {
                argc : container.len() as i32,
                argv : container.as_ptr()
            }
        }
    }

    #[repr(C)]
    pub struct WindowInfo {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        parent_window: *const c_int,
        windowless_rendering_enabled: c_int,
        transparent_painting_enabled: c_int,
        window: *const c_int
    }

    impl WindowInfo {
        pub fn default() -> WindowInfo {
            return WindowInfo {
                x: 0u32,
                y: 0u32,
                width: 0u32,
                height: 0u32,
                parent_window: std::ptr::null(),
                windowless_rendering_enabled: 0i32,
                transparent_painting_enabled: 0i32,
                window: std::ptr::null()
            }
        }
    }


}
