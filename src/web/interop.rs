use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;

// imported functions

#[no_mangle]
extern "C" {
    fn stack_push(num: usize);
    fn console_log_stack();
}

// importing strings from JS

#[no_mangle]
pub unsafe extern "C" fn alloc_js_string(cap: usize) -> JSString {
    let mut d = Vec::with_capacity(cap);
    d.set_len(cap);
    let s = Box::new(String::from_utf8_unchecked(d));
    JSString(Box::into_raw(s))
}

#[no_mangle]
pub extern "C" fn get_mut_js_string(mut string: JSString) -> *mut u8 {
    string.as_mut_ptr()
}

#[repr(transparent)]
pub struct JSString(pub *mut String);

impl JSString {
    fn to_owned(&mut self) -> Box<String> {
        let boxed_string = unsafe { Box::from_raw(self.0) };
        self.0 = ptr::null_mut();
        boxed_string
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
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

// exports to js

#[no_mangle]
pub extern "C" fn dealloc_rust_string(ptr: *mut c_char) {
    unsafe { let _ = CString::from_raw(ptr); }
}

macro_rules! export_string {
    ($name:ident, $exec:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            let rust_string = $exec;
            let rust_string_length = rust_string.len();
            let c_string = std::ffi::CString::new(rust_string)
                .expect("must be a valid C string");
            unsafe { stack_push(rust_string_length); }
            unsafe { stack_push(c_string.into_raw() as usize); }
        }
    }
}

// helpers

pub fn console_log(string: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        for c in string.chars() {
            unsafe { stack_push(c as usize); }
        }
        unsafe { console_log_stack(); }
    }
}

// examples

export_string!(TEST_STRING, get_test_string());

fn get_test_string() -> &'static str {
    "test string"
}

#[no_mangle]
pub extern "C" fn receive_string(mut string: JSString) {
    console_log(&format!("string came from rust: {:?}", &string.to_owned()));
}
