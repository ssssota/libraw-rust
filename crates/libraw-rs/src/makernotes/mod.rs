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

pub struct Makernotes {
    inner: libraw_sys::libraw_makernotes_t,
}

impl Makernotes {
    pub(crate) fn new(value: libraw_sys::libraw_makernotes_t) -> Self {
        Makernotes { inner: value }
    }

    #[inline]
    pub fn canon(&self) -> canon::Canon {
        canon::Canon::new(self.inner.canon)
    }
    #[inline]
    pub fn nikon(&self) -> nikon::Nikon {
        nikon::Nikon::new(self.inner.nikon)
    }
    #[inline]
    pub fn hasselblad(&self) -> hasselblad::Hasselblad {
        hasselblad::Hasselblad::new(self.inner.hasselblad)
    }
    #[inline]
    pub fn fuji(&self) -> fuji::Fuji {
        fuji::Fuji::new(self.inner.fuji)
    }
    #[inline]
    pub fn olympus(&self) -> olympus::Olympus {
        olympus::Olympus::new(self.inner.olympus)
    }
    #[inline]
    pub fn sony(&self) -> sony::Sony {
        sony::Sony::new(self.inner.sony)
    }
    #[inline]
    pub fn kodak(&self) -> kodak::Kodak {
        kodak::Kodak::new(self.inner.kodak)
    }
    #[inline]
    pub fn panasonic(&self) -> panasonic::Panasonic {
        panasonic::Panasonic::new(self.inner.panasonic)
    }
    #[inline]
    pub fn pentax(&self) -> pentax::Pentax {
        pentax::Pentax::new(self.inner.pentax)
    }
    #[inline]
    pub fn phaseone(&self) -> phaseone::PhaseOne {
        phaseone::PhaseOne::new(self.inner.phaseone)
    }
    #[inline]
    pub fn ricoh(&self) -> ricoh::Ricoh {
        ricoh::Ricoh::new(self.inner.ricoh)
    }
    #[inline]
    pub fn samsung(&self) -> samsung::Samsung {
        samsung::Samsung::new(self.inner.samsung)
    }
    #[inline]
    pub fn common(&self) -> common::Common {
        common::Common::new(self.inner.common)
    }
}
