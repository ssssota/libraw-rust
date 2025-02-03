use crate::{impl_property, impl_serialize};

pub struct Olympus {
    inner: libraw_sys::libraw_olympus_makernotes_t,
}

impl Olympus {
    pub fn new(value: libraw_sys::libraw_olympus_makernotes_t) -> Self {
        Olympus { inner: value }
    }

    impl_property!(camera_type2, CameraType2, [i8; 6]);
    impl_property!(valid_bits, ValidBits, u16);
    impl_property!(sensor_calibration, SensorCalibration, [i32; 2]);
    impl_property!(drive_mode, DriveMode, [u16; 5]);
    impl_property!(color_space, ColorSpace, u16);
    impl_property!(focus_mode, FocusMode, [u16; 2]);
    impl_property!(auto_focus, AutoFocus, u16);
    impl_property!(af_point, AFPoint, u16);
    impl_property!(af_areas, AFAreas, [u32; 64]);
    impl_property!(af_point_selected, AFPointSelected, [f64; 5]);
    impl_property!(af_result, AFResult, u16);
    impl_property!(af_fine_tune, AFFineTune, u8);
    impl_property!(af_fine_tune_adj, AFFineTuneAdj, [i16; 3]);
    impl_property!(special_mode, SpecialMode, [u32; 3]);
    impl_property!(zoom_step_count, ZoomStepCount, u16);
    impl_property!(focus_step_count, FocusStepCount, u16);
    impl_property!(focus_step_infinity, FocusStepInfinity, u16);
    impl_property!(focus_step_near, FocusStepNear, u16);
    impl_property!(focus_distance, FocusDistance, f64);
    impl_property!(aspect_frame, AspectFrame, [u16; 4]);
    impl_property!(stacked_image, StackedImage, [u32; 2]);
    pub fn is_live_nd(&self) -> bool {
        self.inner.isLiveND != 0
    }
    impl_property!(live_nd_factor, LiveNDfactor, u32);
    impl_property!(panorama_mode, Panorama_mode, u16);
    impl_property!(panorama_frame_num, Panorama_frameNum, u16);
}

impl_serialize!(
    Olympus,
    [
        camera_type2,
        valid_bits,
        sensor_calibration,
        drive_mode,
        color_space,
        focus_mode,
        auto_focus,
        af_point,
        // af_areas,
        af_point_selected,
        af_result,
        af_fine_tune,
        af_fine_tune_adj,
        special_mode,
        zoom_step_count,
        focus_step_count,
        focus_step_infinity,
        focus_step_near,
        focus_distance,
        aspect_frame,
        stacked_image,
        is_live_nd,
        live_nd_factor,
        panorama_mode,
        panorama_frame_num
    ]
);
