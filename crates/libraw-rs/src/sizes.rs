use libraw_sys::*;

pub struct Sizes {
    pub(crate) inner: libraw_sys::libraw_image_sizes_t,
}

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
    pub fn raw_width(&self) -> u16 {
        (self.inner).raw_width
    }
    pub fn raw_height(&self) -> u16 {
        (self.inner).raw_height
    }
    pub fn width(&self) -> u16 {
        (self.inner).width
    }
    pub fn height(&self) -> u16 {
        (self.inner).height
    }
    pub fn top_margin(&self) -> u16 {
        (self.inner).top_margin
    }
    pub fn left_margin(&self) -> u16 {
        (self.inner).left_margin
    }
    pub fn iwidth(&self) -> u16 {
        (self.inner).iwidth
    }
    pub fn iheight(&self) -> u16 {
        (self.inner).iheight
    }
    pub fn raw_pitch(&self) -> u32 {
        (self.inner).raw_pitch
    }
    pub fn pixel_aspect(&self) -> f64 {
        (self.inner).pixel_aspect
    }
    pub fn flip(&self) -> Rotation {
        Rotation::from((self.inner).flip)
    }
    pub fn raw_inset_crops(&self) -> [libraw_raw_inset_crop_t; 2] {
        (self.inner).raw_inset_crops
    }
}
