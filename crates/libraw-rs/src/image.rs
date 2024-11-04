use libraw_sys::*;

pub struct ProcessedImage {
    inner: *mut libraw_processed_image_t,
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        unsafe { libraw_dcraw_clear_mem(self.inner) };
    }
}

impl ProcessedImage {
    pub fn new(inner: *mut libraw_processed_image_t) -> Self {
        ProcessedImage { inner }
    }
}
