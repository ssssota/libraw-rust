use libraw_sys::*;

use crate::impl_property;

pub struct ImgOther {
    inner: libraw_imgother_t,
}

impl ImgOther {
    pub(crate) fn new(inner: libraw_imgother_t) -> Self {
        Self { inner }
    }

    impl_property!(iso_speed, f32);
    impl_property!(shutter, f32);
    impl_property!(aperture, f32);
    impl_property!(focal_len, f32);
    impl_property!(timestamp, time_t);
    impl_property!(shot_order, u32);
    impl_property!(gpsdata, [u32; 32usize]);
    impl_property!(parsed_gps, libraw_gps_info_t);
    impl_property!(desc, Option<String>);
    impl_property!(artist, Option<String>);
    impl_property!(analogbalance, [f32; 4usize]);
}
