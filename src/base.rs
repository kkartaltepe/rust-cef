extern crate libc;
use self::libc::size_t;
use std::default::Default;
use std::mem;

#[allow(unused_variables)]
extern fn nop_base(ptr: *mut CefBase) {
	return
}

#[allow(unused_variables)]
extern fn nop_base_int(ptr: *mut CefBase) -> i32 {
	return 0;
}


#[repr(C)]
pub struct CefBase {
    pub size: size_t,
    add_ref: extern fn(this: *mut CefBase),
    release: extern fn(this: *mut CefBase) -> i32,
    has_one_ref: extern fn(this: *mut CefBase) -> i32
}

impl Default for CefBase {
	fn default() -> Self {
		return CefBase::get::<CefBase>();
	}
}

impl CefBase {
	pub fn get<T>() -> CefBase {
		return CefBase {
			size: mem::size_of::<T>() as size_t,
			add_ref: nop_base,
			release: nop_base_int,
			has_one_ref: nop_base_int
		}
	}
}

