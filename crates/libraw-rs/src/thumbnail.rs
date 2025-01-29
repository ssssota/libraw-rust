use crate::{impl_display, impl_property};

#[derive(Debug)]
pub struct Thumbnail {
    inner: libraw_sys::libraw_thumbnail_t,
}

impl Thumbnail {
    pub(crate) fn new(inner: libraw_sys::libraw_thumbnail_t) -> Self {
        Thumbnail { inner }
    }

    pub fn tformat(&self) -> ThumbnailFormat {
        ThumbnailFormat::from(self.inner.tformat)
    }
    impl_property!(twidth, u16);
    impl_property!(theight, u16);
    impl_property!(tlength, u32);
    impl_property!(tcolors, i32);
    pub fn thumb(&self) -> *mut std::os::raw::c_char {
        self.inner.thumb
    }
}

impl_display!(Thumbnail, [tformat, twidth, theight, tlength, tcolors]);

#[derive(Debug, PartialEq)]
pub enum ThumbnailFormat {
    Unknown = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_UNKNOWN as isize,
    JPEG = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEG as isize,
    Bitmap = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP as isize,
    Bitmap16 = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP16 as isize,
    Layer = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_LAYER as isize,
    Rollei = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_ROLLEI as isize,
    H265 = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_H265 as isize,
    JPEGXL = libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEGXL as isize,
}
impl From<libraw_sys::LibRaw_thumbnail_formats> for ThumbnailFormat {
    fn from(value: libraw_sys::LibRaw_thumbnail_formats) -> Self {
        match value {
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEG => ThumbnailFormat::JPEG,
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP => ThumbnailFormat::Bitmap,
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP16 => {
                ThumbnailFormat::Bitmap16
            }
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_LAYER => ThumbnailFormat::Layer,
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_ROLLEI => ThumbnailFormat::Rollei,
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_H265 => ThumbnailFormat::H265,
            libraw_sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEGXL => ThumbnailFormat::JPEGXL,
            _ => ThumbnailFormat::Unknown,
        }
    }
}
