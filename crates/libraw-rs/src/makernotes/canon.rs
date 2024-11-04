use libraw_sys::*;

// pub ColorDataVer: libc::c_int,
// pub ColorDataSubVer: libc::c_int,
// pub SpecularWhiteLevel: libc::c_int,
// pub NormalWhiteLevel: libc::c_int,
// pub ChannelBlackLevel: [libc::c_int; 4usize],
// pub AverageBlackLevel: libc::c_int,
// pub multishot: [libc::c_uint; 4usize],
// pub MeteringMode: libc::c_short,
// pub SpotMeteringMode: libc::c_short,
// pub FlashMeteringMode: uchar,
// pub FlashExposureLock: libc::c_short,
// pub ExposureMode: libc::c_short,
// pub AESetting: libc::c_short,
// pub ImageStabilization: libc::c_short,
// pub FlashMode: libc::c_short,
// pub FlashActivity: libc::c_short,
// pub FlashBits: libc::c_short,
// pub ManualFlashOutput: libc::c_short,
// pub FlashOutput: libc::c_short,
// pub FlashGuideNumber: libc::c_short,
// pub ContinuousDrive: libc::c_short,
// pub SensorWidth: libc::c_short,
// pub SensorHeight: libc::c_short,
// pub AFMicroAdjMode: libc::c_int,
// pub AFMicroAdjValue: f32,
// pub MakernotesFlip: libc::c_short,
// pub RecordMode: libc::c_short,
// pub SRAWQuality: libc::c_short,
// pub wbi: libc::c_uint,
// pub RF_lensID: libc::c_short,
// pub AutoLightingOptimizer: libc::c_int,
// pub HighlightTonePriority: libc::c_int,
// pub Quality: libc::c_short,
// pub CanonLog: libc::c_int,
// pub DefaultCropAbsolute: libraw_area_t,
// pub RecommendedImageArea: libraw_area_t,
// pub LeftOpticalBlack: libraw_area_t,
// pub UpperOpticalBlack: libraw_area_t,
// pub ActiveArea: libraw_area_t,
// pub ISOgain: [libc::c_short; 2usize],

pub struct Canon {
    pub color_data_ver: i32,
    pub color_data_sub_ver: i32,
    pub specular_white_level: i32,
    pub normal_white_level: i32,
    pub channel_black_level: [i32; 4],
    pub average_black_level: i32,
    pub multishot: [u32; 4],
    pub metering_mode: i16,
    pub spot_metering_mode: i16,
    pub flash_metering_mode: u8,
    pub flash_exposure_lock: i16,
    pub exposure_mode: i16,
    pub ae_setting: i16,
    pub image_stabilization: i16,
    pub flash_mode: i16,
    pub flash_activity: i16,
    pub flash_bits: i16,
    pub manual_flash_output: i16,
    pub flash_output: i16,
    pub flash_guide_number: i16,
    pub continuous_drive: i16,
    pub sensor_width: i16,
    pub sensor_height: i16,
    pub af_micro_adj_mode: i32,
    pub af_micro_adj_value: f32,
    pub makernotes_flip: i16,
    pub record_mode: i16,
    pub sraw_quality: i16,
    pub wbi: u32,
    pub rf_lens_id: i16,
    pub auto_lighting_optimizer: i32,
    pub highlight_tone_priority: i32,
    pub quality: i16,
    pub canon_log: i32,
    pub default_crop_absolute: libraw_area_t,
    pub recommended_image_area: libraw_area_t,
    pub left_optical_black: libraw_area_t,
    pub upper_optical_black: libraw_area_t,
    pub active_area: libraw_area_t,
    pub iso_gain: [i16; 2],
}

impl From<libraw_canon_makernotes_t> for Canon {
    fn from(value: libraw_canon_makernotes_t) -> Self {
        Canon {
            color_data_ver: value.ColorDataVer,
            color_data_sub_ver: value.ColorDataSubVer,
            specular_white_level: value.SpecularWhiteLevel,
            normal_white_level: value.NormalWhiteLevel,
            channel_black_level: value.ChannelBlackLevel,
            average_black_level: value.AverageBlackLevel,
            multishot: value.multishot,
            metering_mode: value.MeteringMode,
            spot_metering_mode: value.SpotMeteringMode,
            flash_metering_mode: value.FlashMeteringMode,
            flash_exposure_lock: value.FlashExposureLock,
            exposure_mode: value.ExposureMode,
            ae_setting: value.AESetting,
            image_stabilization: value.ImageStabilization,
            flash_mode: value.FlashMode,
            flash_activity: value.FlashActivity,
            flash_bits: value.FlashBits,
            manual_flash_output: value.ManualFlashOutput,
            flash_output: value.FlashOutput,
            flash_guide_number: value.FlashGuideNumber,
            continuous_drive: value.ContinuousDrive,
            sensor_width: value.SensorWidth,
            sensor_height: value.SensorHeight,
            af_micro_adj_mode: value.AFMicroAdjMode,
            af_micro_adj_value: value.AFMicroAdjValue,
            makernotes_flip: value.MakernotesFlip,
            record_mode: value.RecordMode,
            sraw_quality: value.SRAWQuality,
            wbi: value.wbi,
            rf_lens_id: value.RF_lensID,
            auto_lighting_optimizer: value.AutoLightingOptimizer,
            highlight_tone_priority: value.HighlightTonePriority,
            quality: value.Quality,
            canon_log: value.CanonLog,
            default_crop_absolute: value.DefaultCropAbsolute,
            recommended_image_area: value.RecommendedImageArea,
            left_optical_black: value.LeftOpticalBlack,
            upper_optical_black: value.UpperOpticalBlack,
            active_area: value.ActiveArea,
            iso_gain: value.ISOgain,
        }
    }
}

impl Canon {}
