use crate::{impl_property, makernotes_lens::MakernotesLens};

pub struct LensInfo {
    inner: libraw_sys::libraw_lensinfo_t,
}
impl LensInfo {
    pub(crate) fn new(inner: libraw_sys::libraw_lensinfo_t) -> Self {
        LensInfo { inner }
    }
    impl_property!(min_focal, MinFocal, Option<f32>);
    impl_property!(max_focal, MaxFocal, Option<f32>);
    impl_property!(max_ap4_min_focal, MaxAp4MinFocal, Option<f32>);
    impl_property!(max_ap4_max_focal, MaxAp4MaxFocal, Option<f32>);
    impl_property!(exif_max_ap, EXIF_MaxAp, Option<f32>);
    impl_property!(lens_make, LensMake, Option<String>);
    impl_property!(lens, Lens, Option<String>);
    impl_property!(lens_serial, LensSerial, Option<String>);
    impl_property!(internal_lens_serial, InternalLensSerial, Option<String>);
    impl_property!(
        focal_length_in_35mm_format,
        FocalLengthIn35mmFormat,
        libraw_sys::ushort
    );
    pub fn makernotes(&self) -> MakernotesLens {
        MakernotesLens::new(self.inner.makernotes)
    }
}
