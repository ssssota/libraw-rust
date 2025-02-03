use crate::{impl_property, impl_serialize};

pub struct Pentax {
    inner: libraw_sys::libraw_pentax_makernotes_t,
}

impl Pentax {
    pub fn new(value: libraw_sys::libraw_pentax_makernotes_t) -> Self {
        Pentax { inner: value }
    }

    impl_property!(drive_mode, DriveMode, [u8; 4usize]);
    impl_property!(focus_mode, FocusMode, [u16; 2usize]);
    impl_property!(af_point_selected, AFPointSelected, [u16; 2usize]);
    impl_property!(af_point_selected_area, AFPointSelected_Area, u16);
    impl_property!(af_points_in_focus_version, AFPointsInFocus_version, i32);
    impl_property!(af_points_in_focus, AFPointsInFocus, u32);
    impl_property!(focus_position, FocusPosition, u16);
    impl_property!(af_adjustment, AFAdjustment, i16);
    impl_property!(af_point_mode, AFPointMode, u8);
    impl_property!(multi_exposure, MultiExposure, u8);
    impl_property!(quality, Quality, u16);
}

impl_serialize!(
    Pentax,
    [
        drive_mode,
        focus_mode,
        af_point_selected,
        af_point_selected_area,
        af_points_in_focus_version,
        af_points_in_focus,
        focus_position,
        af_adjustment,
        af_point_mode,
        multi_exposure,
        quality
    ]
);
