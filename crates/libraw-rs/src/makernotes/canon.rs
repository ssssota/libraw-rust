use crate::impl_property;

#[derive(Debug)]
pub struct Canon {
    inner: libraw_sys::libraw_canon_makernotes_t,
}
impl Canon {
    pub(crate) fn new(value: libraw_sys::libraw_canon_makernotes_t) -> Self {
        Canon { inner: value }
    }
    impl_property!(color_data_ver, ColorDataVer, i32);
    impl_property!(color_data_sub_ver, ColorDataSubVer, i32);
    impl_property!(specular_white_level, SpecularWhiteLevel, i32);
    impl_property!(normal_white_level, NormalWhiteLevel, i32);
    impl_property!(channel_black_level, ChannelBlackLevel, [i32; 4]);
    impl_property!(average_black_level, AverageBlackLevel, i32);
    impl_property!(multishot, multishot, [u32; 4]);
    impl_property!(metering_mode, MeteringMode, i16);
    impl_property!(spot_metering_mode, SpotMeteringMode, i16);
    impl_property!(flash_metering_mode, FlashMeteringMode, u8);
    impl_property!(flash_exposure_lock, FlashExposureLock, i16);
    impl_property!(exposure_mode, ExposureMode, i16);
    impl_property!(ae_setting, AESetting, i16);
    impl_property!(image_stabilization, ImageStabilization, i16);
    impl_property!(flash_mode, FlashMode, i16);
    impl_property!(flash_activity, FlashActivity, i16);
    impl_property!(flash_bits, FlashBits, i16);
    impl_property!(manual_flash_output, ManualFlashOutput, i16);
    impl_property!(flash_output, FlashOutput, i16);
    impl_property!(flash_guide_number, FlashGuideNumber, i16);
    impl_property!(continuous_drive, ContinuousDrive, i16);
    impl_property!(sensor_width, SensorWidth, i16);
    impl_property!(sensor_height, SensorHeight, i16);
    impl_property!(af_micro_adj_mode, AFMicroAdjMode, i32);
    impl_property!(af_micro_adj_value, AFMicroAdjValue, f32);
    impl_property!(makernotes_flip, MakernotesFlip, i16);
    impl_property!(record_mode, RecordMode, i16);
    impl_property!(sraw_quality, SRAWQuality, i16);
    impl_property!(wbi, wbi, u32);
    impl_property!(rf_lens_id, RF_lensID, i16);
    impl_property!(auto_lighting_optimizer, AutoLightingOptimizer, i32);
    impl_property!(highlight_tone_priority, HighlightTonePriority, i32);
    impl_property!(quality, Quality, i16);
    impl_property!(canon_log, CanonLog, i32);
    impl_property!(
        default_crop_absolute,
        DefaultCropAbsolute,
        libraw_sys::libraw_area_t
    );
    impl_property!(
        recommended_image_area,
        RecommendedImageArea,
        libraw_sys::libraw_area_t
    );
    impl_property!(
        left_optical_black,
        LeftOpticalBlack,
        libraw_sys::libraw_area_t
    );
    impl_property!(
        upper_optical_black,
        UpperOpticalBlack,
        libraw_sys::libraw_area_t
    );
    impl_property!(active_area, ActiveArea, libraw_sys::libraw_area_t);
    impl_property!(iso_gain, ISOgain, [i16; 2]);
}
