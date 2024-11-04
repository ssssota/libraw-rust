use std::borrow::Cow;

use crate::utils::string_from;
use libraw_sys::*;

pub struct IParams<'a> {
    pub make: Option<String>,
    pub model: Option<String>,
    pub software: Option<String>,
    pub normalized_make: Option<String>,
    pub normalized_model: Option<String>,
    pub maker_index: u32,
    pub raw_count: u32,
    pub dng_version: u32,
    pub is_foveon: bool,
    pub colors: i32,
    pub filters: u32,
    pub xtrans: [[i8; 6usize]; 6usize],
    pub xtrans_abs: [[i8; 6usize]; 6usize],
    pub cdesc: Option<String>,
    pub xmplen: u32,
    pub xmpdata: Option<Cow<'a, [i8]>>,
}

impl From<libraw_iparams_t> for IParams<'_> {
    fn from(value: libraw_iparams_t) -> Self {
        IParams {
            make: string_from(value.make.as_ptr()),
            model: string_from(value.model.as_ptr()),
            software: string_from(value.software.as_ptr()),
            normalized_make: string_from(value.normalized_make.as_ptr()),
            normalized_model: string_from(value.normalized_model.as_ptr()),
            maker_index: value.maker_index,
            raw_count: value.raw_count,
            dng_version: value.dng_version,
            is_foveon: value.is_foveon != 0,
            colors: value.colors,
            filters: value.filters,
            xtrans: value.xtrans,
            xtrans_abs: value.xtrans_abs,
            cdesc: string_from(value.cdesc.as_ptr()),
            xmplen: value.xmplen,
            xmpdata: if value.xmpdata.is_null() {
                None
            } else {
                Some(unsafe {
                    Cow::Borrowed(std::slice::from_raw_parts(
                        value.xmpdata,
                        value.xmplen as usize,
                    ))
                })
            },
        }
    }
}
