use crate::{impl_property, impl_serialize};

pub struct ColorData {
    inner: libraw_sys::libraw_colordata_t,
}

impl ColorData {
    pub(crate) fn new(inner: libraw_sys::libraw_colordata_t) -> Self {
        ColorData { inner }
    }
    impl_property!(curve, [u16; 65536]);
    impl_property!(cblack, [u32; 4104]);
    impl_property!(black, u32);
    impl_property!(data_maximum, u32);
    impl_property!(maximum, u32);
    #[cfg(any(windows, target_family = "wasm"))]
    impl_property!(linear_max, [i32; 4]);
    #[cfg(not(any(windows, target_family = "wasm")))]
    impl_property!(linear_max, [i64; 4]);
    impl_property!(fmaximum, f32);
    impl_property!(fnorm, f32);
    impl_property!(white, [[u16; 8]; 8]);
    impl_property!(cam_mul, [f32; 4]);
    impl_property!(pre_mul, [f32; 4]);
    impl_property!(cmatrix, [[f32; 4]; 3]);
    impl_property!(ccm, [[f32; 4]; 3]);
    impl_property!(rgb_cam, [[f32; 4]; 3]);
    impl_property!(cam_xyz, [[f32; 3]; 4]);
    impl_property!(phase_one_data, libraw_sys::ph1_t);
    impl_property!(flash_used, f32);
    impl_property!(canon_ev, f32);
    impl_property!(model2, Option<String>);
    impl_property!(unique_camera_model, UniqueCameraModel, Option<String>);
    impl_property!(localized_camera_model, LocalizedCameraModel, Option<String>);
    impl_property!(image_unique_id, ImageUniqueID, Option<String>);
    impl_property!(raw_data_unique_id, RawDataUniqueID, Option<String>);
    impl_property!(original_raw_file_name, OriginalRawFileName, Option<String>);
    impl_property!(profile, *mut std::ffi::c_void);
    impl_property!(profile_length, u32);
    impl_property!(black_stat, [u32; 8]);
    impl_property!(dng_color, [libraw_sys::libraw_dng_color_t; 2]);
    impl_property!(dng_levels, libraw_sys::libraw_dng_levels_t);
    impl_property!(wb_coeffs, WB_Coeffs, [[i32; 4]; 256]);
    impl_property!(wbct_coeffs, WBCT_Coeffs, [[f32; 5]; 64]);
    impl_property!(as_shot_wb_applied, i32);
    impl_property!(p1_color, P1_color, [libraw_sys::libraw_P1_color_t; 2]);
    impl_property!(raw_bps, u32);
    impl_property!(exif_color_space, ExifColorSpace, i32);
}

impl_serialize!(
    ColorData,
    [
        // curve,
        // cblack,
        black,
        data_maximum,
        maximum,
        linear_max,
        fmaximum,
        fnorm,
        white,
        cam_mul,
        pre_mul,
        cmatrix,
        ccm,
        rgb_cam,
        cam_xyz,
        // phase_one_data,
        flash_used,
        canon_ev,
        model2,
        unique_camera_model,
        localized_camera_model,
        image_unique_id,
        raw_data_unique_id,
        original_raw_file_name,
        // profile,
        // profile_length,
        black_stat,
        // dng_color,
        // dng_levels,
        // wb_coeffs,
        // wbct_coeffs,
        as_shot_wb_applied,
        // p1_color,
        raw_bps,
        exif_color_space
    ]
);
