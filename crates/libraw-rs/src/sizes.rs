use crate::{impl_display, impl_property};

#[derive(Debug)]
pub struct Sizes {
    inner: libraw_sys::libraw_image_sizes_t,
}

#[derive(Debug, PartialEq)]
pub enum Rotation {
    Zero,
    OneEighty,
    NinetyCounterclockwise,
    NinetyClockwise,
    Unknown(i32),
}
impl From<i32> for Rotation {
    fn from(value: i32) -> Self {
        match value {
            0 => Rotation::Zero,
            3 => Rotation::OneEighty,
            5 => Rotation::NinetyCounterclockwise,
            6 => Rotation::NinetyClockwise,
            _ => Rotation::Unknown(value),
        }
    }
}

impl Sizes {
    pub(crate) fn new(inner: libraw_sys::libraw_image_sizes_t) -> Self {
        Sizes { inner }
    }
    impl_property!(raw_width, u16);
    impl_property!(raw_height, u16);
    impl_property!(width, u16);
    impl_property!(height, u16);
    impl_property!(top_margin, u16);
    impl_property!(left_margin, u16);
    impl_property!(iwidth, u16);
    impl_property!(iheight, u16);
    impl_property!(raw_pitch, u32);
    impl_property!(pixel_aspect, f64);
    impl_property!(raw_inset_crops, [libraw_sys::libraw_raw_inset_crop_t; 2]);
    pub fn flip(&self) -> Rotation {
        Rotation::from((self.inner).flip)
    }
}

impl_display!(
    Sizes,
    [
        raw_width,
        raw_height,
        width,
        height,
        top_margin,
        left_margin,
        iwidth,
        iheight,
        raw_pitch,
        pixel_aspect,
        raw_inset_crops,
        flip
    ]
);
