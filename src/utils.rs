use std::ffi::CStr;

pub fn string_from(ptr: *const i8) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_string_lossy().into_owned()
}

pub fn non_zero<T: PartialEq + Default>(val: T) -> Option<T> {
    // if val == 0
    if val == T::default() {
        None
    } else {
        Some(val)
    }
}
