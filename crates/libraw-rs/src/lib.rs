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
        assert_eq!(cameras.len(), 1222);
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

        assert_eq!(
            format!("{}", libraw.other()),
            "ImgOther { iso_speed: 100.0, shutter: 0.016666668, aperture: 3.5, focal_len: 50.0, timestamp: 1234262827, shot_order: 0, gpsdata: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], parsed_gps: libraw_gps_info_t { latitude: [0.0, 0.0, 0.0], longitude: [0.0, 0.0, 0.0], gpstimestamp: [0.0, 0.0, 0.0], altitude: 0.0, altref: 0, latref: 0, longref: 0, gpsstatus: 0, gpsparsed: 1 }, desc: None, artist: None, analogbalance: [0.0, 0.0, 0.0, 0.0] }"
        );

        assert_eq!(
            format!("{}", libraw.sizes()),
            "Sizes { raw_width: 4352, raw_height: 2868, width: 4310, height: 2868, top_margin: 0, left_margin: 0, iwidth: 4310, iheight: 2868, raw_pitch: 0, pixel_aspect: 1.0, raw_inset_crops: [libraw_raw_inset_crop_t { cleft: 65535, ctop: 65535, cwidth: 0, cheight: 0 }, libraw_raw_inset_crop_t { cleft: 65535, ctop: 65535, cwidth: 0, cheight: 0 }], flip: Zero }"
        );

        assert_eq!(
            format!("{}", libraw.thumbnail()),
            "Thumbnail { tformat: Unknown, twidth: 4288, theight: 2848, tlength: 1382859, tcolors: 0 }"
        );
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

        assert_eq!(
            format!("{}", libraw.other()),
            "ImgOther { iso_speed: 100.0, shutter: 0.01, aperture: 6.3, focal_len: 25.0, timestamp: 1266203455, shot_order: 0, gpsdata: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], parsed_gps: libraw_gps_info_t { latitude: [0.0, 0.0, 0.0], longitude: [0.0, 0.0, 0.0], gpstimestamp: [0.0, 0.0, 0.0], altitude: 0.0, altref: 0, latref: 0, longref: 0, gpsstatus: 0, gpsparsed: 0 }, desc: None, artist: Some(\"Mike Gemuende\"), analogbalance: [0.0, 0.0, 0.0, 0.0] }"
        );

        assert_eq!(
            format!("{}", libraw.sizes()),
            "Sizes { raw_width: 3948, raw_height: 2622, width: 3906, height: 2602, top_margin: 18, left_margin: 42, iwidth: 3906, iheight: 2602, raw_pitch: 0, pixel_aspect: 1.0, raw_inset_crops: [libraw_raw_inset_crop_t { cleft: 52, ctop: 24, cwidth: 3888, cheight: 2591 }, libraw_raw_inset_crop_t { cleft: 65535, ctop: 65535, cwidth: 0, cheight: 0 }], flip: Zero }"
        );

        assert_eq!(
            format!("{}", libraw.thumbnail()),
            "Thumbnail { tformat: Unknown, twidth: 3888, theight: 2592, tlength: 2373662, tcolors: 0 }"
        );
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

        assert_eq!(
            format!("{}", libraw.other()),
            "ImgOther { iso_speed: 160.0, shutter: 12.0, aperture: 4.0, focal_len: 50.0, timestamp: 1186060429, shot_order: 0, gpsdata: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], parsed_gps: libraw_gps_info_t { latitude: [0.0, 0.0, 0.0], longitude: [0.0, 0.0, 0.0], gpstimestamp: [0.0, 0.0, 0.0], altitude: 0.0, altref: 0, latref: 0, longref: 0, gpsstatus: 0, gpsparsed: 0 }, desc: None, artist: None, analogbalance: [0.0, 0.0, 0.0, 0.0] }"
        );

        assert_eq!(
            format!("{}", libraw.sizes()),
            "Sizes { raw_width: 3920, raw_height: 2638, width: 3920, height: 2638, top_margin: 0, left_margin: 0, iwidth: 3920, iheight: 2638, raw_pitch: 0, pixel_aspect: 1.0, raw_inset_crops: [libraw_raw_inset_crop_t { cleft: 2, ctop: 2, cwidth: 3916, cheight: 2634 }, libraw_raw_inset_crop_t { cleft: 65535, ctop: 65535, cwidth: 0, cheight: 0 }], flip: Zero }"
        );

        assert_eq!(
            format!("{}", libraw.thumbnail()),
            "Thumbnail { tformat: Unknown, twidth: 320, theight: 240, tlength: 65280, tcolors: 0 }"
        );
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

        assert_eq!(
            format!("{}", libraw.other()),
            "ImgOther { iso_speed: 100.0, shutter: 0.0025, aperture: 6.3, focal_len: 14.0, timestamp: 1228889193, shot_order: 0, gpsdata: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], parsed_gps: libraw_gps_info_t { latitude: [0.0, 0.0, 0.0], longitude: [0.0, 0.0, 0.0], gpstimestamp: [0.0, 0.0, 0.0], altitude: 0.0, altref: 0, latref: 0, longref: 0, gpsstatus: 0, gpsparsed: 0 }, desc: None, artist: None, analogbalance: [0.0, 0.0, 0.0, 0.0] }",
        );

        assert_eq!(
            format!("{}", libraw.sizes()),
            "Sizes { raw_width: 4060, raw_height: 3016, width: 4016, height: 3016, top_margin: 0, left_margin: 0, iwidth: 4016, iheight: 3016, raw_pitch: 0, pixel_aspect: 1.0, raw_inset_crops: [libraw_raw_inset_crop_t { cleft: 8, ctop: 4, cwidth: 4000, cheight: 3000 }, libraw_raw_inset_crop_t { cleft: 65535, ctop: 65535, cwidth: 0, cheight: 0 }], flip: Zero }"
        );

        assert_eq!(
            format!("{}", libraw.thumbnail()),
            "Thumbnail { tformat: Unknown, twidth: 1920, theight: 1440, tlength: 687616, tcolors: 0 }"
        );
    }
}
