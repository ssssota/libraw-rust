use std::borrow::Cow;

use crate::impl_property;

#[derive(Debug)]
pub struct ProcessedImage {
    inner: libraw_sys::libraw_processed_image_t,
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        let ptr = &mut self.inner as *mut _;
        unsafe { libraw_sys::libraw_dcraw_clear_mem(ptr) };
    }
}

impl ProcessedImage {
    pub(crate) fn new(inner: libraw_sys::libraw_processed_image_t) -> Self {
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
    JPEG = libraw_sys::LibRaw_image_formats_LIBRAW_IMAGE_JPEG as isize,
    BITMAP = libraw_sys::LibRaw_image_formats_LIBRAW_IMAGE_BITMAP as isize,
}
impl From<libraw_sys::LibRaw_image_formats> for ImageFormat {
    fn from(value: libraw_sys::LibRaw_image_formats) -> Self {
        match value {
            libraw_sys::LibRaw_image_formats_LIBRAW_IMAGE_JPEG => ImageFormat::JPEG,
            libraw_sys::LibRaw_image_formats_LIBRAW_IMAGE_BITMAP => ImageFormat::BITMAP,
            _ => unreachable!(),
        }
    }
}
