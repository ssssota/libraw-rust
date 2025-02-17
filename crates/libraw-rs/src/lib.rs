pub mod colordata;
pub mod image;
pub mod imgother;
pub mod iparams;
pub mod lensinfo;
pub mod makernotes;
pub mod makernotes_lens;
pub mod result;
pub mod sizes;
pub mod thumbnail;
mod utils;

use colordata::ColorData;
use image::ProcessedImage;
use imgother::ImgOther;
use iparams::IParams;
use lensinfo::LensInfo;
use makernotes::Makernotes;
use result::{handle_error, Result};
use sizes::Sizes;
use std::ffi::CStr;
use thumbnail::Thumbnail;

#[derive(Debug)]
pub struct LibRaw {
    inner: *mut libraw_sys::libraw_data_t,
}

pub fn version() -> &'static str {
    unsafe {
        CStr::from_ptr(libraw_sys::libraw_version())
            .to_str()
            .unwrap()
    }
}

#[inline]
pub fn version_number() -> i32 {
    unsafe { libraw_sys::libraw_versionNumber() }
}

#[inline]
pub fn camera_count() -> i32 {
    unsafe { libraw_sys::libraw_cameraCount() }
}

pub fn camera_list() -> Vec<&'static str> {
    let mut list = Vec::new();
    let count = camera_count();
    let names = unsafe { libraw_sys::libraw_cameraList() };
    for i in 0..count {
        let name = unsafe { names.offset(i as isize) };
        let name = unsafe { CStr::from_ptr(*name) };
        let name = name.to_str().unwrap();
        list.push(name);
    }
    list
}

impl LibRaw {
    #[cfg(not(target_family = "wasm"))]
    pub fn open_file<P: AsRef<std::path::Path>>(path: &P) -> Result<Self> {
        let inner = unsafe { libraw_sys::libraw_init(0) };
        let path = path.as_ref();
        let path = path.to_str().ok_or(result::Error::InvalidPath)?;
        let path = std::ffi::CString::new(path).map_err(|_| result::Error::InvalidPath)?;
        handle_error(unsafe { libraw_sys::libraw_open_file(inner, path.as_ptr()) })?;
        Ok(LibRaw { inner })
    }

    pub fn open_buffer(buffer: &[u8]) -> Result<Self> {
        let inner = unsafe { libraw_sys::libraw_init(0) };
        handle_error(unsafe {
            libraw_sys::libraw_open_buffer(inner, buffer.as_ptr() as *const _, buffer.len())
        })?;
        Ok(LibRaw { inner })
    }

    pub fn unpack(&self) -> Result<()> {
        handle_error(unsafe { libraw_sys::libraw_unpack(self.inner) })
    }

    pub fn raw2image(&self) -> Result<()> {
        handle_error(unsafe { libraw_sys::libraw_raw2image(self.inner) })
    }

    pub fn dcraw_process(&self) -> Result<()> {
        handle_error(unsafe { libraw_sys::libraw_dcraw_process(self.inner) })
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn dcraw_ppm_tiff_writer<P: AsRef<std::path::Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        let path = path.to_str().ok_or(result::Error::InvalidPath)?;
        let path = std::ffi::CString::new(path).map_err(|_| result::Error::InvalidPath)?;
        handle_error(unsafe { libraw_sys::libraw_dcraw_ppm_tiff_writer(self.inner, path.as_ptr()) })
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn dcraw_thumb_writer<P: AsRef<std::path::Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        let path = path.to_str().ok_or(result::Error::InvalidPath)?;
        let path = std::ffi::CString::new(path).map_err(|_| result::Error::InvalidPath)?;
        handle_error(unsafe { libraw_sys::libraw_dcraw_thumb_writer(self.inner, path.as_ptr()) })
    }

    pub fn dcraw_make_mem_image(&self) -> Result<ProcessedImage> {
        let ptr = std::ptr::null_mut();
        let image = unsafe { *libraw_sys::libraw_dcraw_make_mem_image(self.inner, ptr) };
        handle_error(ptr as i32)?;
        let image = ProcessedImage::new(image);
        Ok(image)
    }

    pub fn dcraw_make_mem_thumb(&self) -> Result<ProcessedImage> {
        let ptr = std::ptr::null_mut();
        let image = unsafe { *libraw_sys::libraw_dcraw_make_mem_thumb(self.inner, ptr) };
        handle_error(ptr as i32)?;
        let image = ProcessedImage::new(image);
        Ok(image)
    }

    pub fn idata(&self) -> IParams {
        IParams::new(unsafe { (*self.inner).idata })
    }

    pub fn sizes(&self) -> Sizes {
        Sizes::new(unsafe { (*self.inner).sizes })
    }

    pub fn lens(&self) -> LensInfo {
        LensInfo::new(unsafe { (*self.inner).lens })
    }

    pub fn makernotes(&self) -> Makernotes {
        Makernotes::new(unsafe { (*self.inner).makernotes })
    }

    pub fn color(&self) -> ColorData {
        ColorData::new(unsafe { (*self.inner).color })
    }

    pub fn other(&self) -> ImgOther {
        ImgOther::new(unsafe { (*self.inner).other })
    }

    pub fn thumbnail(&self) -> Thumbnail {
        Thumbnail::new(unsafe { (*self.inner).thumbnail })
    }

    pub fn params(&self) -> libraw_sys::libraw_output_params_t {
        unsafe { (*self.inner).params }
    }
}

impl Drop for LibRaw {
    fn drop(&mut self) {
        unsafe { libraw_sys::libraw_close(self.inner) };
    }
}

#[cfg(test)]
mod tests {
    use makernotes_lens::mount::CameraMount;

    use super::*;

    #[test]
    fn version() {
        assert_eq!(super::version(), "0.22.0-Devel202403");
    }

    #[test]
    fn version_number() {
        assert_eq!(super::version_number(), 5632);
    }

    #[test]
    fn camera_list() {
        let cameras = super::camera_list();
        assert_eq!(cameras.len(), 1221);
        // Check for some known cameras
        assert!(cameras.contains(&"Adobe Digital Negative (DNG)"));
        assert!(cameras.contains(&"Canon EOS R3"));
        assert!(cameras.contains(&"Leica M11"));
        assert!(cameras.contains(&"Nikon Z fc"));
        assert!(cameras.contains(&"Sony ILCE-7M4 (A7 IV)"));
    }

    #[test]
    fn nikon_d90() {
        let buf = include_bytes!("../../../raw-samples/NEF/RAW_NIKON_D90.NEF");
        let libraw = LibRaw::open_buffer(buf).unwrap();
        let iparams = libraw.idata();
        assert_eq!(iparams.make(), Some("Nikon".to_string()));
        assert_eq!(iparams.model(), Some("D90".to_string()));
        assert_eq!(iparams.normalized_make(), Some("Nikon".to_string()));
        assert_eq!(iparams.normalized_model(), Some("D90".to_string()));
        assert!(!iparams.is_foveon());

        let lensinfo = libraw.lens();
        assert_eq!(lensinfo.exif_max_ap(), Some(1.4142135));
        assert_eq!(lensinfo.focal_length_in_35mm_format(), 75);
        let lens_makernotes = lensinfo.makernotes();
        assert_eq!(lens_makernotes.lens_mount(), CameraMount::Nikon_F);
        assert_eq!(lens_makernotes.camera_mount(), CameraMount::Nikon_F);

        let makernotes = libraw.makernotes();
        assert_eq!(makernotes.common().real_iso(), Some(100.0));

        let other = libraw.other();
        assert_eq!(other.focal_len(), 50.0);
        assert_eq!(other.iso_speed(), 100.0);
        assert_eq!(other.shutter(), 1.0 / 60.0);
        assert_eq!(other.aperture(), 3.5);

        let sizes = libraw.sizes();
        assert_eq!(sizes.raw_height(), 2868);
        assert_eq!(sizes.raw_width(), 4352);
    }

    #[test]
    fn canon_400d1() {
        let buf = include_bytes!("../../../raw-samples/CR2/sample_canon_400d1.cr2");
        let libraw = LibRaw::open_buffer(buf).unwrap();
        let iparams = libraw.idata();
        assert_eq!(iparams.make(), Some("Canon".to_string()));
        assert_eq!(iparams.model(), Some("EOS 400D".to_string()));
        assert_eq!(iparams.normalized_make(), Some("Canon".to_string()));
        assert_eq!(iparams.normalized_model(), Some("EOS 400D".to_string()));
        assert!(!iparams.is_foveon());

        let lensinfo = libraw.lens();
        let lens_makernotes = lensinfo.makernotes();
        assert_eq!(lens_makernotes.lens_mount(), CameraMount::Canon_EF);
        assert_eq!(lens_makernotes.camera_mount(), CameraMount::Canon_EF);

        let makernotes = libraw.makernotes();
        assert_eq!(makernotes.common().real_iso(), Some(100.0));

        let other = libraw.other();
        assert_eq!(other.focal_len(), 25.0);
        assert_eq!(other.iso_speed(), 100.0);
        assert_eq!(other.shutter(), 1.0 / 100.0);
        assert_eq!(other.aperture(), 6.3);

        let sizes = libraw.sizes();
        assert_eq!(sizes.raw_height(), 2622);
        assert_eq!(sizes.raw_width(), 3948);
    }

    #[test]
    fn leica_m8() {
        let buf = include_bytes!("../../../raw-samples/DNG/RAW_LEICA_M8.DNG");
        let libraw = LibRaw::open_buffer(buf).unwrap();
        let iparams = libraw.idata();
        assert_eq!(iparams.make(), Some("Leica".to_string()));
        assert_eq!(iparams.model(), Some("M8".to_string()));
        assert_eq!(iparams.normalized_make(), Some("Leica".to_string()));
        assert_eq!(iparams.normalized_model(), Some("M8".to_string()));
        assert!(!iparams.is_foveon());

        let makernotes = libraw.makernotes();
        assert_eq!(makernotes.common().real_iso(), None);

        let other = libraw.other();
        assert_eq!(other.focal_len(), 50.0);
        assert_eq!(other.iso_speed(), 160.0);
        assert_eq!(other.shutter(), 12.0);
        assert_eq!(other.aperture(), 4.0);

        let sizes = libraw.sizes();
        assert_eq!(sizes.raw_height(), 2638);
        assert_eq!(sizes.raw_width(), 3920);
    }

    #[test]
    fn panasonic_g1() {
        let buf = include_bytes!("../../../raw-samples/RW2/RAW_PANASONIC_G1.RW2");
        let libraw = LibRaw::open_buffer(buf).unwrap();
        let iparams = libraw.idata();
        assert_eq!(iparams.make(), Some("Panasonic".to_string()));
        assert_eq!(iparams.model(), Some("DMC-G1".to_string()));
        assert_eq!(iparams.normalized_make(), Some("Panasonic".to_string()));
        assert_eq!(iparams.normalized_model(), Some("DMC-G1".to_string()));
        assert!(!iparams.is_foveon());

        let lensinfo = libraw.lens();
        assert_eq!(lensinfo.exif_max_ap(), Some(3.4982677));

        let makernotes = libraw.makernotes();
        assert_eq!(makernotes.common().real_iso(), None);

        let other = libraw.other();
        assert_eq!(other.focal_len(), 14.0);
        assert_eq!(other.iso_speed(), 100.0);
        assert_eq!(other.shutter(), 1.0 / 400.0);
        assert_eq!(other.aperture(), 6.3);

        let sizes = libraw.sizes();
        assert_eq!(sizes.raw_height(), 3016);
        assert_eq!(sizes.raw_width(), 4060);
    }
}
