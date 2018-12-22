use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::panic;

// imported functions

#[no_mangle]
extern "C" {
    pub fn stack_push(num: usize);
    fn console_stack(type_: usize); // 0 - log, 2 - warn, 3 - error
}

// called on init

#[no_mangle]
extern "C" fn web_main() {
     panic_hook();
     crate::main();
}

// importing strings from JS

#[no_mangle]
unsafe extern "C" fn alloc_js_string(cap: usize) -> JSString {
    let mut d = Vec::with_capacity(cap);
    d.set_len(cap);
    let s = Box::new(String::from_utf8_unchecked(d));
    JSString(Box::into_raw(s))
}

#[no_mangle]
extern "C" fn get_mut_js_string(mut string: JSString) -> *mut u8 {
    string.as_mut_ptr()
}

#[repr(transparent)]
pub struct JSString(pub *mut String);

impl JSString {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        let ptr = unsafe { (&mut *self.0).as_mut_vec() }.as_mut_ptr();
        self.0 = ptr::null_mut();
        ptr
    }
    pub fn to_string(&mut self) -> String {
        let boxed_string = unsafe { Box::from_raw(self.0) };
        self.0 = ptr::null_mut();
        *boxed_string
    }
}

impl Drop for JSString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { Box::from_raw(self.0); }
        }
    }
}

// exports to js

#[no_mangle]
extern "C" fn dealloc_rust_string(ptr: *mut c_char) {
    unsafe { let _ = CString::from_raw(ptr); }
}

pub fn export_string(rust_string: &str) {
    let rust_string_length = rust_string.len();
    let c_string = std::ffi::CString::new(rust_string)
        .expect("must be a valid C string");
    unsafe { stack_push(rust_string_length); }
    unsafe { stack_push(c_string.into_raw() as usize); }
}

#[macro_export]
macro_rules! export_string {
    ($name:ident => $exec:expr) => {
        #[no_mangle]
        extern "C" fn $name() {
            use crate::web::interop::stack_push;
            let rust_string = $exec;
            let rust_string_length = rust_string.len();
            let c_string = std::ffi::CString::new(rust_string)
                .expect("must be a valid C string");
            unsafe { stack_push(rust_string_length); }
            unsafe { stack_push(c_string.into_raw() as usize); }
        }
    }
}

// console

pub fn console_log(string: &str, type_: usize) {
    // TODO: pass a pointer instead
    for c in string.chars() {
        unsafe { stack_push(c as usize); }
    }
    unsafe { console_stack(type_); }
}

#[macro_export]
macro_rules! console {
    ( $x:expr, $( $y:expr ),* ) => {
        #[cfg(target_arch = "wasm32")]
        crate::web::interop::console_log(&format!($x, $($y),*), 0);
        #[cfg(not(target_arch = "wasm32"))]
        println!($x, $($y),*);
    };
    ( $x:expr ) => {
        #[cfg(target_arch = "wasm32")]
        crate::web::interop::console_log(&format!($x), 0);
        #[cfg(not(target_arch = "wasm32"))]
        println!($x);
    };
}

// panic

#[inline]
fn panic_hook() {
    use std::sync::{ONCE_INIT, Once};
    static SET_HOOK: Once = ONCE_INIT;
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(|info| {
            console_log(&info.to_string(), 2);
        }));
    });
}
