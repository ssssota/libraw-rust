use crate::utils::string_from;
use libraw_sys::*;

pub struct IParams {
    pub(crate) inner: libraw_iparams_t,
}

impl IParams {
    pub fn make(&self) -> String {
        string_from((self.inner).make.as_ptr())
    }
    pub fn model(&self) -> String {
        string_from((self.inner).model.as_ptr())
    }
    pub fn normalized_make(&self) -> String {
        string_from((self.inner).normalized_make.as_ptr())
    }
    pub fn normalized_model(&self) -> String {
        string_from((self.inner).normalized_model.as_ptr())
    }
    pub fn maker_index(&self) -> u32 {
        (self.inner).maker_index
    }
    pub fn software(&self) -> String {
        string_from((self.inner).software.as_ptr())
    }
    pub fn raw_count(&self) -> u32 {
        (self.inner).raw_count
    }
    pub fn is_foveon(&self) -> bool {
        (self.inner).is_foveon != 0
    }
    pub fn dng_version(&self) -> u32 {
        (self.inner).dng_version
    }
    pub fn colors(&self) -> i32 {
        (self.inner).colors
    }
}
