use libraw_sys::*;

use crate::impl_property;

pub struct ProcessedImage {
    inner: libraw_processed_image_t,
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        let ptr = &mut self.inner as *mut _;
        unsafe { libraw_dcraw_clear_mem(ptr) };
    }
}

impl ProcessedImage {
    pub(crate) fn new(inner: libraw_processed_image_t) -> Self {
        ProcessedImage { inner }
    }

    impl_property!(r#type, type_, LibRaw_image_formats);
    impl_property!(height, u16);
    impl_property!(width, u16);
    impl_property!(colors, u16);
    impl_property!(bits, u16);
    impl_property!(data_size, u32);
    impl_property!(data, [u8; 1usize]);
}

// pub type_: LibRaw_image_formats,
// pub height: ushort,
// pub width: ushort,
// pub colors: ushort,
// pub bits: ushort,
// pub data_size: libc::c_uint,
// pub data: [libc::c_uchar; 1usize],
