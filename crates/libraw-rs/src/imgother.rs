use std::string;

use libraw_sys::*;

use crate::utils::string_from;

pub struct ImgOther {
    pub iso_speed: f32,
    pub shutter: f32,
    pub aperture: f32,
    pub focal_len: f32,
    pub timestamp: time_t,
    pub shot_order: u32,
    pub gpsdata: [u32; 32usize],
    pub parsed_gps: libraw_gps_info_t,
    pub desc: Option<String>,
    pub artist: Option<String>,
    pub analogbalance: [f32; 4usize],
}

impl From<libraw_imgother_t> for ImgOther {
    fn from(value: libraw_imgother_t) -> Self {
        ImgOther {
            iso_speed: value.iso_speed,
            shutter: value.shutter,
            aperture: value.aperture,
            focal_len: value.focal_len,
            timestamp: value.timestamp,
            shot_order: value.shot_order,
            gpsdata: value.gpsdata,
            parsed_gps: value.parsed_gps,
            desc: string_from(value.desc.as_ptr()),
            artist: string_from(value.artist.as_ptr()),
            analogbalance: value.analogbalance,
        }
    }
}
