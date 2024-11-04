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
    pub(crate) inner: libraw_makernotes_t,
}

impl Makernotes {
    #[inline]
    pub fn canon(&self) -> canon::Canon {
        canon::Canon {
            inner: self.inner.canon,
        }
    }
    #[inline]
    pub fn nikon(&self) -> nikon::Nikon {
        nikon::Nikon {
            inner: self.inner.nikon,
        }
    }
    #[inline]
    pub fn hasselblad(&self) -> hasselblad::Hasselblad {
        hasselblad::Hasselblad {
            inner: self.inner.hasselblad,
        }
    }
    #[inline]
    pub fn fuji(&self) -> fuji::Fuji {
        fuji::Fuji {
            inner: self.inner.fuji,
        }
    }
    #[inline]
    pub fn olympus(&self) -> olympus::Olympus {
        olympus::Olympus {
            inner: self.inner.olympus,
        }
    }
    #[inline]
    pub fn sony(&self) -> sony::Sony {
        sony::Sony {
            inner: self.inner.sony,
        }
    }
    #[inline]
    pub fn kodak(&self) -> kodak::Kodak {
        kodak::Kodak {
            inner: self.inner.kodak,
        }
    }
    #[inline]
    pub fn panasonic(&self) -> panasonic::Panasonic {
        panasonic::Panasonic {
            inner: self.inner.panasonic,
        }
    }
    #[inline]
    pub fn pentax(&self) -> pentax::Pentax {
        pentax::Pentax {
            inner: self.inner.pentax,
        }
    }
    #[inline]
    pub fn phaseone(&self) -> phaseone::PhaseOne {
        phaseone::PhaseOne {
            inner: self.inner.phaseone,
        }
    }
    #[inline]
    pub fn ricoh(&self) -> ricoh::Ricoh {
        ricoh::Ricoh {
            inner: self.inner.ricoh,
        }
    }
    #[inline]
    pub fn samsung(&self) -> samsung::Samsung {
        samsung::Samsung {
            inner: self.inner.samsung,
        }
    }
    #[inline]
    pub fn common(&self) -> common::Common {
        common::Common {
            inner: self.inner.common,
        }
    }
}
