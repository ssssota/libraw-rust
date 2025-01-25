use libraw_sys::*;

use crate::impl_property;

pub struct Sony {
    inner: libraw_sony_info_t,
}

impl Sony {
    pub fn new(value: libraw_sony_info_t) -> Self {
        Sony { inner: value }
    }

    impl_property!(camera_type, CameraType, u16);
    impl_property!(sony0x9400_version, Sony0x9400_version, u8);
    impl_property!(sony0x9400_release_mode2, Sony0x9400_ReleaseMode2, u8);
    impl_property!(
        sony0x9400_sequence_image_number,
        Sony0x9400_SequenceImageNumber,
        u32
    );
    impl_property!(sony0x9400_sequence_length1, Sony0x9400_SequenceLength1, u8);
    impl_property!(
        sony0x9400_sequence_file_number,
        Sony0x9400_SequenceFileNumber,
        u32
    );
    impl_property!(sony0x9400_sequence_length2, Sony0x9400_SequenceLength2, u8);
    impl_property!(af_area_mode_setting, AFAreaModeSetting, u8);
    impl_property!(af_area_mode, AFAreaMode, u16);
    impl_property!(flexible_spot_position, FlexibleSpotPosition, [u16; 2usize]);
    impl_property!(af_point_selected, AFPointSelected, u8);
    impl_property!(af_point_selected_0x201e, AFPointSelected_0x201e, u8);
    impl_property!(n_af_points_used, nAFPointsUsed, i16);
    impl_property!(af_points_used, AFPointsUsed, [u8; 10usize]);
    impl_property!(af_tracking, AFTracking, u8);
    impl_property!(af_type, AFType, u8);
    impl_property!(focus_location, FocusLocation, [u16; 4usize]);
    impl_property!(focus_position, FocusPosition, u16);
    impl_property!(af_micro_adj_value, AFMicroAdjValue, i8);
    impl_property!(af_micro_adj_on, AFMicroAdjOn, i8);
    impl_property!(
        af_micro_adj_registered_lenses,
        AFMicroAdjRegisteredLenses,
        u8
    );
    impl_property!(variable_low_pass_filter, VariableLowPassFilter, u16);
    impl_property!(
        long_exposure_noise_reduction,
        LongExposureNoiseReduction,
        u32
    );
    impl_property!(high_iso_noise_reduction, HighISONoiseReduction, u16);
    impl_property!(hdr, HDR, [u16; 2usize]);
    impl_property!(group2010, group2010, u16);
    impl_property!(group9050, group9050, u16);
    impl_property!(len_group9050, len_group9050, u16);
    impl_property!(real_iso_offset, real_iso_offset, u16);
    impl_property!(metering_mode_offset, MeteringMode_offset, u16);
    impl_property!(exposure_program_offset, ExposureProgram_offset, u16);
    impl_property!(release_mode2_offset, ReleaseMode2_offset, u16);
    impl_property!(minolta_cam_id, MinoltaCamID, u32);
    impl_property!(firmware, firmware, f32);
    impl_property!(image_count3_offset, ImageCount3_offset, u16);
    impl_property!(image_count3, ImageCount3, u32);
    impl_property!(
        electronic_front_curtain_shutter,
        ElectronicFrontCurtainShutter,
        u32
    );
    impl_property!(metering_mode2, MeteringMode2, u16);
    impl_property!(sony_date_time, SonyDateTime, [i8; 20usize]);
    impl_property!(shot_number_since_power_up, ShotNumberSincePowerUp, u32);
    impl_property!(pixel_shift_group_prefix, PixelShiftGroupPrefix, u16);
    impl_property!(pixel_shift_group_id, PixelShiftGroupID, u32);
    impl_property!(n_shots_in_pixel_shift_group, nShotsInPixelShiftGroup, i8);
    impl_property!(num_in_pixel_shift_group, numInPixelShiftGroup, i8);
    impl_property!(prd_image_height, prd_ImageHeight, u16);
    impl_property!(prd_image_width, prd_ImageWidth, u16);
    impl_property!(prd_total_bps, prd_Total_bps, u16);
    impl_property!(prd_active_bps, prd_Active_bps, u16);
    impl_property!(prd_storage_method, prd_StorageMethod, u16);
    impl_property!(prd_bayer_pattern, prd_BayerPattern, u16);
    impl_property!(sony_raw_file_type, SonyRawFileType, u16);
    impl_property!(raw_file_type, RAWFileType, u16);
    impl_property!(raw_size_type, RawSizeType, u16);
    impl_property!(quality, Quality, u32);
    impl_property!(file_format, FileFormat, u16);
    impl_property!(meta_version, MetaVersion, Option<String>);
    impl_property!(aspect_ratio, AspectRatio, f32);
}
