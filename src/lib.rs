pub mod image;
pub mod iparams;
pub mod lensinfo;
pub mod result;
pub mod sizes;
pub mod utils;

use image::ProcessedImage;
use iparams::IParams;
use lensinfo::LensInfo;
use libraw_sys::*;
use result::{handle_error, Error, Result};
use sizes::Sizes;
use std::{
    ffi::{CStr, CString},
    path::Path,
};

#[derive(Debug)]
pub struct LibRaw {
    inner: *mut libraw_data_t,
}

impl LibRaw {
    pub fn version() -> &'static str {
        unsafe { CStr::from_ptr(libraw_version()).to_str().unwrap() }
    }

    #[inline]
    pub fn version_number() -> i32 {
        unsafe { libraw_versionNumber() }
    }

    #[inline]
    pub fn camera_count() -> i32 {
        unsafe { libraw_cameraCount() }
    }

    pub fn camera_list() -> Vec<String> {
        let mut list = Vec::new();
        let count = LibRaw::camera_count();
        let names = unsafe { libraw_cameraList() };
        for i in 0..count {
            let name = unsafe { names.offset(i as isize) };
            let name = unsafe { std::ffi::CStr::from_ptr(*name) };
            let name = name.to_string_lossy().into_owned();
            list.push(name);
        }
        list
    }

    pub fn open_file<P: AsRef<Path>>(path: &P) -> Result<Self> {
        let inner = unsafe { libraw_init(0) };
        let path = path.as_ref();
        let path = path.to_str().ok_or(Error::InvalidPath)?;
        let path = CString::new(path).map_err(|_| Error::InvalidPath)?;
        handle_error(unsafe { libraw_open_file(inner, path.as_ptr()) })?;
        Ok(LibRaw { inner })
    }

    pub fn open_buffer(buffer: &[u8]) -> Result<Self> {
        let inner = unsafe { libraw_init(0) };
        handle_error(unsafe {
            libraw_open_buffer(inner, buffer.as_ptr() as *const _, buffer.len())
        })?;
        Ok(LibRaw { inner })
    }

    pub fn unpack(&self) -> Result<()> {
        handle_error(unsafe { libraw_unpack(self.inner) })
    }

    pub fn raw2image(&self) -> Result<()> {
        handle_error(unsafe { libraw_raw2image(self.inner) })
    }

    pub fn dcraw_process(&self) -> Result<()> {
        handle_error(unsafe { libraw_dcraw_process(self.inner) })
    }

    pub fn dcraw_ppm_tiff_writer<P: AsRef<Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        let path = path.to_str().ok_or(Error::InvalidPath)?;
        let path = CString::new(path).map_err(|_| Error::InvalidPath)?;
        handle_error(unsafe { libraw_dcraw_ppm_tiff_writer(self.inner, path.as_ptr()) })
    }

    pub fn dcraw_thumb_writer<P: AsRef<Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        let path = path.to_str().ok_or(Error::InvalidPath)?;
        let path = CString::new(path).map_err(|_| Error::InvalidPath)?;
        handle_error(unsafe { libraw_dcraw_thumb_writer(self.inner, path.as_ptr()) })
    }

    pub fn dcraw_make_mem_image(&self) -> Result<ProcessedImage> {
        let ptr = std::ptr::null_mut();
        let image = unsafe { libraw_dcraw_make_mem_image(self.inner, ptr) };
        let image = ProcessedImage::new(image);
        handle_error(ptr as i32)?;
        Ok(image)
    }

    pub fn dcraw_make_mem_thumb(&self) -> Result<ProcessedImage> {
        let ptr = std::ptr::null_mut();
        let image = unsafe { libraw_dcraw_make_mem_thumb(self.inner, ptr) };
        let image = ProcessedImage::new(image);
        handle_error(ptr as i32)?;
        Ok(image)
    }

    pub fn idata(&self) -> IParams {
        IParams {
            inner: unsafe { (*self.inner).idata },
        }
    }

    pub fn sizes(&self) -> Sizes {
        Sizes {
            inner: unsafe { (*self.inner).sizes },
        }
    }

    pub fn lens(&self) -> LensInfo {
        LensInfo {
            inner: unsafe { (*self.inner).lens },
        }
    }

    pub fn makernotes(&self) -> libraw_makernotes_t {
        unsafe { (*self.inner).makernotes }
    }

    pub fn color(&self) -> libraw_colordata_t {
        unsafe { (*self.inner).color }
    }

    pub fn other(&self) -> libraw_imgother_t {
        unsafe { (*self.inner).other }
    }

    pub fn thumbnail(&self) -> libraw_thumbnail_t {
        unsafe { (*self.inner).thumbnail }
    }

    pub fn params(&self) -> libraw_output_params_t {
        unsafe { (*self.inner).params }
    }
}

impl Drop for LibRaw {
    fn drop(&mut self) {
        unsafe { libraw_close(self.inner) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(LibRaw::version(), "0.21.3-Release");
    }

    #[test]
    fn version_number() {
        assert_eq!(LibRaw::version_number(), 5379);
    }

    #[test]
    fn camera_list() {
        let cameras = LibRaw::camera_list();
        assert!(!cameras.is_empty());
        // Check for some known cameras
        assert!(cameras.contains(&"Adobe Digital Negative (DNG)".to_string()));
        assert!(cameras.contains(&"Canon EOS R3".to_string()));
        assert!(cameras.contains(&"Leica M11".to_string()));
        assert!(cameras.contains(&"Nikon Z fc".to_string()));
        assert!(cameras.contains(&"Sony ILCE-7M4 (A7 IV)".to_string()));
    }

    #[test]
    fn nikon_d90() {
        let buf = include_bytes!("../raw-samples/NEF/RAW_NIKON_D90.NEF");
        let libraw = LibRaw::open_buffer(buf).unwrap();
        let iparams = libraw.idata();
        assert_eq!(iparams.make(), "Nikon");
        assert_eq!(iparams.model(), "D90");
        assert_eq!(iparams.normalized_make(), "Nikon");
        assert_eq!(iparams.normalized_model(), "D90");
        assert_eq!(iparams.is_foveon(), false);

        let lensinfo = libraw.lens();
        assert_eq!(lensinfo.exif_max_ap(), 1.4142135);
    }
}
