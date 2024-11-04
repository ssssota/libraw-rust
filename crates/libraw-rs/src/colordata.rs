use libraw_sys::*;

use crate::utils::string_from;

pub struct ColorData {
    pub(crate) inner: libraw_colordata_t,
}

impl ColorData {
    pub fn curve(&self) -> [u16; 65536] {
        self.inner.curve
    }
    pub fn cblack(&self) -> [u32; 4104] {
        self.inner.cblack
    }
    pub fn black(&self) -> u32 {
        self.inner.black
    }
    pub fn data_maximum(&self) -> u32 {
        self.inner.data_maximum
    }
    pub fn maximum(&self) -> u32 {
        self.inner.maximum
    }
    pub fn linear_max(&self) -> [i64; 4] {
        self.inner.linear_max
    }
    pub fn fmaximum(&self) -> f32 {
        self.inner.fmaximum
    }
    pub fn fnorm(&self) -> f32 {
        self.inner.fnorm
    }
    pub fn white(&self) -> [[u16; 8]; 8] {
        self.inner.white
    }
    pub fn cam_mul(&self) -> [f32; 4] {
        self.inner.cam_mul
    }
    pub fn pre_mul(&self) -> [f32; 4] {
        self.inner.pre_mul
    }
    pub fn cmatrix(&self) -> [[f32; 4]; 3] {
        self.inner.cmatrix
    }
    pub fn ccm(&self) -> [[f32; 4]; 3] {
        self.inner.ccm
    }
    pub fn rgb_cam(&self) -> [[f32; 4]; 3] {
        self.inner.rgb_cam
    }
    pub fn cam_xyz(&self) -> [[f32; 3]; 4] {
        self.inner.cam_xyz
    }
    pub fn phase_one_data(&self) -> ph1_t {
        self.inner.phase_one_data
    }
    pub fn flash_used(&self) -> f32 {
        self.inner.flash_used
    }
    pub fn canon_ev(&self) -> f32 {
        self.inner.canon_ev
    }
    pub fn model2(&self) -> Option<String> {
        string_from(self.inner.model2.as_ptr())
    }
    pub fn unique_camera_model(&self) -> Option<String> {
        string_from(self.inner.UniqueCameraModel.as_ptr())
    }
    pub fn localized_camera_model(&self) -> Option<String> {
        string_from(self.inner.LocalizedCameraModel.as_ptr())
    }
    pub fn image_unique_id(&self) -> Option<String> {
        string_from(self.inner.ImageUniqueID.as_ptr())
    }
    pub fn raw_data_unique_id(&self) -> Option<String> {
        string_from(self.inner.RawDataUniqueID.as_ptr())
    }
    pub fn original_raw_file_name(&self) -> Option<String> {
        string_from(self.inner.OriginalRawFileName.as_ptr())
    }
    pub fn profile(&self) -> *mut std::ffi::c_void {
        self.inner.profile
    }
    pub fn profile_length(&self) -> u32 {
        self.inner.profile_length
    }
    pub fn black_stat(&self) -> [u32; 8] {
        self.inner.black_stat
    }
    pub fn dng_color(&self) -> [libraw_dng_color_t; 2] {
        self.inner.dng_color
    }
    pub fn dng_levels(&self) -> libraw_dng_levels_t {
        self.inner.dng_levels
    }
    pub fn wb_coeffs(&self) -> [[i32; 4]; 256] {
        self.inner.WB_Coeffs
    }
    pub fn wbct_coeffs(&self) -> [[f32; 5]; 64] {
        self.inner.WBCT_Coeffs
    }
    pub fn as_shot_wb_applied(&self) -> i32 {
        self.inner.as_shot_wb_applied
    }
    pub fn p1_color(&self) -> [libraw_P1_color_t; 2] {
        self.inner.P1_color
    }
    pub fn raw_bps(&self) -> u32 {
        self.inner.raw_bps
    }
    pub fn exif_color_space(&self) -> i32 {
        self.inner.ExifColorSpace
    }
}
