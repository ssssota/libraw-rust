use crate::{impl_property, impl_serialize};

pub struct Nikon {
    inner: libraw_sys::libraw_nikon_makernotes_t,
}

impl Nikon {
    pub(crate) fn new(value: libraw_sys::libraw_nikon_makernotes_t) -> Self {
        Nikon { inner: value }
    }
    impl_property!(exposure_bracket_value, ExposureBracketValue, f64);
    impl_property!(active_d_lighting, ActiveDLighting, u16);
    impl_property!(shooting_mode, ShootingMode, u16);
    impl_property!(image_stabilization, ImageStabilization, [u8; 7]);
    impl_property!(vibration_reduction, VibrationReduction, u8);
    impl_property!(vr_mode, VRMode, u8);
    impl_property!(flash_setting, FlashSetting, [i8; 13]);
    impl_property!(flash_type, FlashType, [i8; 20]);
    impl_property!(
        flash_exposure_compensation,
        FlashExposureCompensation,
        [u8; 4]
    );
    impl_property!(
        external_flash_exposure_comp,
        ExternalFlashExposureComp,
        [u8; 4]
    );
    impl_property!(
        flash_exposure_bracket_value,
        FlashExposureBracketValue,
        [u8; 4]
    );
    impl_property!(flash_mode, FlashMode, u8);
    impl_property!(flash_exposure_compensation2, FlashExposureCompensation2, i8);
    impl_property!(flash_exposure_compensation3, FlashExposureCompensation3, i8);
    impl_property!(flash_exposure_compensation4, FlashExposureCompensation4, i8);
    impl_property!(flash_source, FlashSource, u8);
    impl_property!(flash_firmware, FlashFirmware, [u8; 2]);
    impl_property!(external_flash_flags, ExternalFlashFlags, u8);
    impl_property!(flash_control_commander_mode, FlashControlCommanderMode, u8);
    impl_property!(
        flash_output_and_compensation,
        FlashOutputAndCompensation,
        u8
    );
    impl_property!(flash_focal_length, FlashFocalLength, u8);
    impl_property!(flash_gn_distance, FlashGNDistance, u8);
    impl_property!(flash_group_control_mode, FlashGroupControlMode, [u8; 4]);
    impl_property!(
        flash_group_output_and_compensation,
        FlashGroupOutputAndCompensation,
        [u8; 4]
    );
    impl_property!(flash_color_filter, FlashColorFilter, u8);
    impl_property!(nef_compression, NEFCompression, u16);
    impl_property!(exposure_mode, ExposureMode, i32);
    impl_property!(exposure_program, ExposureProgram, i32);
    impl_property!(nme_shots, nMEshots, i32);
    impl_property!(me_gain_on, MEgainOn, i32);
    impl_property!(me_wb, ME_WB, [f64; 4]);
    impl_property!(af_fine_tune, AFFineTune, u8);
    impl_property!(af_fine_tune_index, AFFineTuneIndex, u8);
    impl_property!(af_fine_tune_adj, AFFineTuneAdj, i8);
    impl_property!(lens_data_version, LensDataVersion, u32);
    impl_property!(flash_info_version, FlashInfoVersion, u32);
    impl_property!(color_balance_version, ColorBalanceVersion, u32);
    impl_property!(key, u8);
    impl_property!(nef_bit_depth, NEFBitDepth, [u16; 4]);
    impl_property!(high_speed_crop_format, HighSpeedCropFormat, u16);
    impl_property!(
        sensor_high_speed_crop,
        SensorHighSpeedCrop,
        libraw_sys::libraw_sensor_highspeed_crop_t
    );
    impl_property!(sensor_width, SensorWidth, u16);
    impl_property!(sensor_height, SensorHeight, u16);
    // impl_property!(active_d_lighting, Active_D_Lighting, u16);
    impl_property!(picture_control_version, PictureControlVersion, u32);
    impl_property!(picture_control_name, PictureControlName, [i8; 20]);
    impl_property!(picture_control_base, PictureControlBase, [i8; 20]);
    impl_property!(shot_info_version, ShotInfoVersion, u32);
    impl_property!(makernotes_flip, MakernotesFlip, i16);
    impl_property!(roll_angle, RollAngle, f64);
    impl_property!(pitch_angle, PitchAngle, f64);
    impl_property!(yaw_angle, YawAngle, f64);
}

impl_serialize!(
    Nikon,
    [
        exposure_bracket_value,
        active_d_lighting,
        shooting_mode,
        image_stabilization,
        vibration_reduction,
        vr_mode,
        flash_setting,
        flash_type,
        flash_exposure_compensation,
        external_flash_exposure_comp,
        flash_exposure_bracket_value,
        flash_mode,
        flash_exposure_compensation2,
        flash_exposure_compensation3,
        flash_exposure_compensation4,
        flash_source,
        flash_firmware,
        external_flash_flags,
        flash_control_commander_mode,
        flash_output_and_compensation,
        flash_focal_length,
        flash_gn_distance,
        flash_group_control_mode,
        flash_group_output_and_compensation,
        flash_color_filter,
        nef_compression,
        exposure_mode,
        exposure_program,
        nme_shots,
        me_gain_on,
        me_wb,
        af_fine_tune,
        af_fine_tune_index,
        af_fine_tune_adj,
        lens_data_version,
        flash_info_version,
        color_balance_version,
        key,
        nef_bit_depth,
        high_speed_crop_format,
        // sensor_high_speed_crop,
        sensor_width,
        sensor_height,
        active_d_lighting,
        picture_control_version,
        picture_control_name,
        picture_control_base,
        shot_info_version,
        makernotes_flip,
        roll_angle,
        pitch_angle,
        yaw_angle
    ]
);
