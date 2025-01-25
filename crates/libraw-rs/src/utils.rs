use std::ffi::CStr;

pub fn string_from(ptr: *const i8) -> Option<String> {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str().map(|s| s.to_string()).ok()
}

pub fn non_zero<T: PartialEq + Default>(val: T) -> Option<T> {
    // if val == 0
    if val == T::default() {
        None
    } else {
        Some(val)
    }
}

#[macro_export]
macro_rules! impl_property {
    ($name:ident, Option<String>) => {
        pub fn $name(&self) -> Option<String> {
            crate::utils::string_from(self.inner.$name.as_ptr())
        }
    };
    ($name:ident, $ty:ty) => {
        #[inline]
        pub fn $name(&self) -> $ty {
            self.inner.$name
        }
    };
    ($name:ident, $prop:ident, Option<String>) => {
        pub fn $name(&self) -> Option<String> {
            crate::utils::string_from(self.inner.$prop.as_ptr())
        }
    };
    ($name:ident, $prop:ident, $ty:ty) => {
        #[inline]
        pub fn $name(&self) -> $ty {
            self.inner.$prop
        }
    };
}
