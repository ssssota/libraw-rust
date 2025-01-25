use crate::{impl_property, makernotes_lens::MakernotesLens};
use libraw_sys as sys;

pub struct LensInfo {
    inner: sys::libraw_lensinfo_t,
}
impl LensInfo {
    pub(crate) fn new(inner: sys::libraw_lensinfo_t) -> Self {
        LensInfo { inner }
    }
    impl_property!(min_focal, MinFocal, f32);
    impl_property!(max_focal, MaxFocal, f32);
    impl_property!(max_ap4_min_focal, MaxAp4MinFocal, f32);
    impl_property!(max_ap4_max_focal, MaxAp4MaxFocal, f32);
    impl_property!(exif_max_ap, EXIF_MaxAp, f32);
    impl_property!(lens_make, LensMake, Option<String>);
    impl_property!(lens, Lens, Option<String>);
    impl_property!(lens_serial, LensSerial, Option<String>);
    impl_property!(internal_lens_serial, InternalLensSerial, Option<String>);
    impl_property!(
        focal_length_in_35mm_format,
        FocalLengthIn35mmFormat,
        sys::ushort
    );
    pub fn makernotes(&self) -> MakernotesLens {
        MakernotesLens::new(self.inner.makernotes)
    }
}
