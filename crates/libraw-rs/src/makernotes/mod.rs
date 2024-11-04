pub mod canon;
pub mod common;
pub mod fuji;
pub mod hasselblad;
pub mod kodak;
pub mod nikon;
pub mod olympus;
pub mod panasonic;
pub mod pentax;
pub mod phaseone;
pub mod ricoh;
pub mod samsung;
pub mod sony;

use libraw_sys::*;
pub struct Makernotes {
    pub canon: libraw_canon_makernotes_t,
    pub nikon: libraw_nikon_makernotes_t,
    pub hasselblad: libraw_hasselblad_makernotes_t,
    pub fuji: libraw_fuji_info_t,
    pub olympus: libraw_olympus_makernotes_t,
    pub sony: libraw_sony_info_t,
    pub kodak: libraw_kodak_makernotes_t,
    pub panasonic: libraw_panasonic_makernotes_t,
    pub pentax: libraw_pentax_makernotes_t,
    pub phaseone: libraw_p1_makernotes_t,
    pub ricoh: libraw_ricoh_makernotes_t,
    pub samsung: libraw_samsung_makernotes_t,
    pub common: libraw_metadata_common_t,
}

impl From<libraw_makernotes_t> for Makernotes {
    fn from(value: libraw_makernotes_t) -> Self {
        Makernotes {
            canon: value.canon,
            nikon: value.nikon,
            hasselblad: value.hasselblad,
            fuji: value.fuji,
            olympus: value.olympus,
            sony: value.sony,
            kodak: value.kodak,
            panasonic: value.panasonic,
            pentax: value.pentax,
            phaseone: value.phaseone,
            ricoh: value.ricoh,
            samsung: value.samsung,
            common: value.common,
        }
    }
}
