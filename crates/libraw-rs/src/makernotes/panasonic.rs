use crate::impl_property;

#[derive(Debug)]
pub struct Panasonic {
    inner: libraw_sys::libraw_panasonic_makernotes_t,
}

impl Panasonic {
    pub fn new(value: libraw_sys::libraw_panasonic_makernotes_t) -> Self {
        Panasonic { inner: value }
    }

    impl_property!(compression, Compression, u16);
    impl_property!(black_level_dim, BlackLevelDim, u16);
    impl_property!(black_level, BlackLevel, [f32; 8usize]);
    impl_property!(multishot, Multishot, u32);
    impl_property!(gamma, gamma, f32);
    impl_property!(high_iso_multiplier, HighISOMultiplier, [i32; 3usize]);
    impl_property!(focus_step_near, FocusStepNear, i16);
    impl_property!(focus_step_count, FocusStepCount, i16);
    impl_property!(zoom_position, ZoomPosition, u32);
    impl_property!(lens_manufacturer, LensManufacturer, u32);
}
