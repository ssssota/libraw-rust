use crate::{impl_property, impl_serialize};

pub struct Samsung {
    inner: libraw_sys::libraw_samsung_makernotes_t,
}

impl Samsung {
    pub fn new(value: libraw_sys::libraw_samsung_makernotes_t) -> Self {
        Samsung { inner: value }
    }

    impl_property!(image_size_full, ImageSizeFull, [u32; 4usize]);
    impl_property!(image_size_crop, ImageSizeCrop, [u32; 4usize]);
    impl_property!(color_space, ColorSpace, [i32; 2usize]);
    impl_property!(key, [u32; 11usize]);
    impl_property!(digital_gain, DigitalGain, f64);
    impl_property!(device_type, DeviceType, i32);
    impl_property!(lens_firmware, LensFirmware, [i8; 32usize]);
}

impl_serialize!(
    Samsung,
    [
        image_size_full,
        image_size_crop,
        color_space,
        key,
        digital_gain,
        device_type,
        lens_firmware
    ]
);
