use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::{ Drop, Range };
use std::ptr;
use std::panic;

// TODO: JSFunc
//
// use js fn stack with Drop to pop
//
// #[no_mangle]
// unsafe extern "C" fn get_func(func: JSFunc) {
//     func.call(&params[..]);
// }


// imported functions

extern "C" {
    fn stack_push(num: usize);
    fn console_stack(type_: usize); // 0 - log, 2 - warn, 3 - error
    pub fn math_random() -> f64;
}

// called on init

#[no_mangle]
extern "C" fn web_main() {
     panic_hook();
     crate::main();
}

// importing strings from JS

#[no_mangle]
unsafe extern "C" fn alloc_str(cap: usize) -> JSString {
    let mut d = Vec::with_capacity(cap);
    d.set_len(cap);
    let s = Box::new(String::from_utf8_unchecked(d));
    JSString(Box::into_raw(s))
}

#[no_mangle]
extern "C" fn get_mut_str(string: JSString) -> *mut u8 {
    string.as_mut_ptr()
}

#[repr(transparent)]
pub struct JSString(pub *mut String);

impl JSString {
    pub fn to_string(mut self) -> String {
        let boxed_string = unsafe { Box::from_raw(self.0) };
        self.0 = ptr::null_mut();
        *boxed_string
    }
    fn as_mut_ptr(mut self) -> *mut u8 {
        let ptr = unsafe { (&mut *self.0).as_mut_vec() }.as_mut_ptr();
        self.0 = ptr::null_mut();
        ptr
    }
}

impl Drop for JSString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { Box::from_raw(self.0); }
        }
    }
}

// exportings strings

fn export_string_raw(rust_string: &str) {
    let rust_string_length = rust_string.len();
    let c_string = std::ffi::CString::new(rust_string)
        .expect("must be a valid C string");
    unsafe {
        stack_push(rust_string_length);
        stack_push(c_string.into_raw() as usize);
    }
}

pub fn export_string(rust_string: &str) {
    rust_string.as_bytes()
        .chunks(10000)
        .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
        .for_each(export_string_raw);
}

#[no_mangle]
extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe { let _ = CString::from_raw(ptr); }
}

// console

pub fn console_log(string: &str, type_: usize) {
    export_string_raw(string);
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

// random

#[allow(unused)]
pub fn range_random(range: Range<usize>) -> usize {
    let (start, end) = (
        range.start as f64,
        (range.end - 1) as f64,
    );
    unsafe {
        ((math_random() * end) + start) as usize
    }
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
