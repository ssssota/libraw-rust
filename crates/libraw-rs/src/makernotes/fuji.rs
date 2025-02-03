use crate::{impl_property, impl_serialize};

pub struct Fuji {
    inner: libraw_sys::libraw_fuji_info_t,
}

impl Fuji {
    pub fn new(value: libraw_sys::libraw_fuji_info_t) -> Self {
        Fuji { inner: value }
    }

    impl_property!(expo_mid_point_shift, ExpoMidPointShift, f32);
    impl_property!(dynamic_range, DynamicRange, u16);
    impl_property!(film_mode, FilmMode, u16);
    impl_property!(dynamic_range_setting, DynamicRangeSetting, u16);
    impl_property!(development_dynamic_range, DevelopmentDynamicRange, u16);
    impl_property!(auto_dynamic_range, AutoDynamicRange, u16);
    impl_property!(d_range_priority, DRangePriority, u16);
    impl_property!(d_range_priority_auto, DRangePriorityAuto, u16);
    impl_property!(d_range_priority_fixed, DRangePriorityFixed, u16);
    impl_property!(fuji_model, FujiModel, [i8; 33]);
    impl_property!(fuji_model2, FujiModel2, [i8; 33]);
    impl_property!(brightness_compensation, BrightnessCompensation, f32);
    impl_property!(focus_mode, FocusMode, u16);
    impl_property!(af_mode, AFMode, u16);
    impl_property!(focus_pixel, FocusPixel, [u16; 2]);
    impl_property!(priority_settings, PrioritySettings, u16);
    impl_property!(focus_settings, FocusSettings, u32);
    impl_property!(af_c_settings, AF_C_Settings, u32);
    impl_property!(focus_warning, FocusWarning, u16);
    impl_property!(image_stabilization, ImageStabilization, [u16; 3]);
    impl_property!(flash_mode, FlashMode, u16);
    impl_property!(wb_preset, WB_Preset, u16);
    impl_property!(shutter_type, ShutterType, u16);
    impl_property!(exr_mode, ExrMode, u16);
    impl_property!(r#macro, Macro, u16);
    impl_property!(rating, Rating, u32);
    impl_property!(crop_mode, CropMode, u16);
    impl_property!(serial_signature, SerialSignature, [i8; 13]);
    impl_property!(sensor_id, SensorID, [i8; 5]);
    impl_property!(raf_version, RAFVersion, [i8; 5]);
    impl_property!(raf_data_generation, RAFDataGeneration, i32);
    impl_property!(raf_data_version, RAFDataVersion, u16);
    impl_property!(is_tsnerdts, isTSNERDTS, i32);
    impl_property!(drive_mode, DriveMode, i16);
    impl_property!(black_level, BlackLevel, [u16; 9]);
    impl_property!(raf_data_image_size_table, RAFData_ImageSizeTable, [u32; 32]);
    impl_property!(auto_bracketing, AutoBracketing, i32);
    impl_property!(sequence_number, SequenceNumber, i32);
    impl_property!(series_length, SeriesLength, i32);
    impl_property!(pixel_shift_offset, PixelShiftOffset, [f32; 2]);
    impl_property!(image_count, ImageCount, i32);
}

impl_serialize!(
    Fuji,
    [
        expo_mid_point_shift,
        dynamic_range,
        film_mode,
        dynamic_range_setting,
        development_dynamic_range,
        auto_dynamic_range,
        d_range_priority,
        d_range_priority_auto,
        d_range_priority_fixed,
        // fuji_model,
        // fuji_model2,
        brightness_compensation,
        focus_mode,
        af_mode,
        focus_pixel,
        priority_settings,
        focus_settings,
        af_c_settings,
        focus_warning,
        image_stabilization,
        flash_mode,
        wb_preset,
        shutter_type,
        exr_mode,
        r#macro,
        rating,
        crop_mode,
        serial_signature,
        sensor_id,
        raf_version,
        raf_data_generation,
        raf_data_version,
        is_tsnerdts,
        drive_mode,
        black_level,
        raf_data_image_size_table,
        auto_bracketing,
        sequence_number,
        series_length,
        pixel_shift_offset,
        image_count
    ]
);
