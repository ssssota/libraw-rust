use crate::{impl_property, impl_serialize};

pub struct Ricoh {
    inner: libraw_sys::libraw_ricoh_makernotes_t,
}

impl Ricoh {
    pub fn new(value: libraw_sys::libraw_ricoh_makernotes_t) -> Self {
        Ricoh { inner: value }
    }

    impl_property!(af_status, AFStatus, u16);
    impl_property!(af_area_x_position, AFAreaXPosition, [u32; 2usize]);
    impl_property!(af_area_y_position, AFAreaYPosition, [u32; 2usize]);
    impl_property!(af_area_mode, AFAreaMode, u16);
    impl_property!(sensor_width, SensorWidth, u32);
    impl_property!(sensor_height, SensorHeight, u32);
    impl_property!(cropped_image_width, CroppedImageWidth, u32);
    impl_property!(cropped_image_height, CroppedImageHeight, u32);
    impl_property!(wide_adapter, WideAdapter, u16);
    impl_property!(crop_mode, CropMode, u16);
    impl_property!(nd_filter, NDFilter, u16);
    impl_property!(auto_bracketing, AutoBracketing, u16);
    impl_property!(macro_mode, MacroMode, u16);
    impl_property!(flash_mode, FlashMode, u16);
    impl_property!(flash_exposure_comp, FlashExposureComp, f64);
    impl_property!(manual_flash_output, ManualFlashOutput, f64);
}

impl_serialize!(
    Ricoh,
    [
        af_status,
        af_area_x_position,
        af_area_y_position,
        af_area_mode,
        sensor_width,
        sensor_height,
        cropped_image_width,
        cropped_image_height,
        wide_adapter,
        crop_mode,
        nd_filter,
        auto_bracketing,
        macro_mode,
        flash_mode,
        flash_exposure_comp,
        manual_flash_output
    ]
);
