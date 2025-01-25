use std::ffi::CStr;

pub unsafe fn string_from(ptr: *const i8) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr)
        .to_str()
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

pub fn none_if_zero<T: PartialEq + Default>(val: T) -> Option<T> {
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
            unsafe { $crate::utils::string_from(self.inner.$name.as_ptr()) }
        }
    };
    ($name:ident, Option<$ty:ty>) => {
        #[inline]
        pub fn $name(&self) -> Option<$ty> {
            $crate::utils::none_if_zero(self.inner.$name)
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
            unsafe { $crate::utils::string_from(self.inner.$prop.as_ptr()) }
        }
    };
    ($name:ident, $prop:ident, Option<$ty:ty>) => {
        #[inline]
        pub fn $name(&self) -> Option<$ty> {
            $crate::utils::none_if_zero(self.inner.$prop)
        }
    };
    ($name:ident, $prop:ident, $ty:ty) => {
        #[inline]
        pub fn $name(&self) -> $ty {
            self.inner.$prop
        }
    };
}
