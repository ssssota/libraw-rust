pub mod focal_type;
pub mod format;
pub mod mount;

use std::ffi::c_ulonglong;

use libraw_sys::libraw_makernotes_lens_t;

use crate::impl_property;

pub struct MakernotesLens {
    inner: libraw_makernotes_lens_t,
}

impl MakernotesLens {
    pub(crate) fn new(inner: libraw_makernotes_lens_t) -> Self {
        MakernotesLens { inner }
    }
    impl_property!(lens_id, LensID, c_ulonglong);
    impl_property!(lens, Lens, Option<String>);
    #[inline]
    pub fn lens_format(&self) -> format::CameraFormat {
        format::CameraFormat::from(self.inner.LensFormat as u32)
    }
    #[inline]
    pub fn lens_mount(&self) -> mount::CameraMount {
        mount::CameraMount::from(self.inner.LensMount as u32)
    }
    impl_property!(cam_id, CamID, c_ulonglong);
    #[inline]
    pub fn camera_format(&self) -> format::CameraFormat {
        format::CameraFormat::from(self.inner.CameraFormat as u32)
    }
    #[inline]
    pub fn camera_mount(&self) -> mount::CameraMount {
        mount::CameraMount::from(self.inner.CameraMount as u32)
    }
    impl_property!(body, Option<String>);
    #[inline]
    pub fn focal_type(&self) -> focal_type::FocalType {
        focal_type::FocalType::from(self.inner.FocalType)
    }
    impl_property!(lens_features_pre, LensFeatures_pre, Option<String>);
    impl_property!(lens_features_suf, LensFeatures_suf, Option<String>);
    impl_property!(min_focal, MinFocal, Option<f32>);
    impl_property!(max_focal, MaxFocal, Option<f32>);
    impl_property!(max_ap4_min_focal, MaxAp4MinFocal, Option<f32>);
    impl_property!(max_ap4_max_focal, MaxAp4MaxFocal, Option<f32>);
    impl_property!(min_ap4_min_focal, MinAp4MinFocal, Option<f32>);
    impl_property!(min_ap4_max_focal, MinAp4MaxFocal, Option<f32>);
    impl_property!(max_ap, MaxAp, Option<f32>);
    impl_property!(min_ap, MinAp, Option<f32>);
    impl_property!(cur_focal, CurFocal, Option<f32>);
    impl_property!(cur_ap, CurAp, Option<f32>);
    impl_property!(max_ap4_cur_focal, MaxAp4CurFocal, Option<f32>);
    impl_property!(min_ap4_cur_focal, MinAp4CurFocal, Option<f32>);
    impl_property!(min_focus_distance, MinFocusDistance, Option<f32>);
    impl_property!(focus_range_index, FocusRangeIndex, Option<f32>);
    impl_property!(lens_f_stops, LensFStops, Option<f32>);
    impl_property!(teleconverter_id, TeleconverterID, c_ulonglong);
    impl_property!(teleconverter, Teleconverter, Option<String>);
    impl_property!(adapter_id, AdapterID, c_ulonglong);
    impl_property!(adapter, Adapter, Option<String>);
    impl_property!(attachment_id, AttachmentID, c_ulonglong);
    impl_property!(attachment, Attachment, Option<String>);
    impl_property!(focal_units, FocalUnits, u16);
    impl_property!(focal_length_in_35mm_format, FocalLengthIn35mmFormat, f32);
}
