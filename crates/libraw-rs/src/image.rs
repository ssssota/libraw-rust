use std::borrow::Cow;

use libraw_sys as sys;

use crate::impl_property;

pub struct ProcessedImage {
    inner: sys::libraw_processed_image_t,
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        let ptr = &mut self.inner as *mut _;
        unsafe { sys::libraw_dcraw_clear_mem(ptr) };
    }
}

impl ProcessedImage {
    pub(crate) fn new(inner: sys::libraw_processed_image_t) -> Self {
        ProcessedImage { inner }
    }

    pub fn r#type(&self) -> ImageFormat {
        ImageFormat::from(self.inner.type_)
    }
    impl_property!(height, u16);
    impl_property!(width, u16);
    impl_property!(colors, u16);
    impl_property!(bits, u16);
    impl_property!(data_size, u32);
    pub fn data(&self) -> Cow<[u8]> {
        let data = &self.inner.data as *const _;
        let len = self.inner.data_size as usize;
        Cow::Borrowed(unsafe { std::slice::from_raw_parts(data, len) })
    }
}

pub enum ImageFormat {
    JPEG = sys::LibRaw_image_formats_LIBRAW_IMAGE_JPEG as isize,
    BITMAP = sys::LibRaw_image_formats_LIBRAW_IMAGE_BITMAP as isize,
}
impl From<sys::LibRaw_image_formats> for ImageFormat {
    fn from(value: sys::LibRaw_image_formats) -> Self {
        match value {
            sys::LibRaw_image_formats_LIBRAW_IMAGE_JPEG => ImageFormat::JPEG,
            sys::LibRaw_image_formats_LIBRAW_IMAGE_BITMAP => ImageFormat::BITMAP,
            _ => unreachable!(),
        }
    }
}
