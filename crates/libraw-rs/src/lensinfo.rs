use crate::utils::{non_zero, string_from};
use libraw_sys as sys;

pub struct LensInfo {
    pub(crate) inner: sys::libraw_lensinfo_t,
}

#[derive(Debug, PartialEq)]
pub enum CameraFormat {
    Unknown(u32),
    FormatAPSC,
    FormatFF,
    FormatMF,
    FormatAPSH,
    Format1INCH,
    Format1div2p3INCH,
    Format1div1p7INCH,
    FormatFT,
    FormatCROP645,
    FormatLeicaS,
    Format645,
    Format66,
    Format69,
    FormatLF,
    FormatLeicaDMR,
    Format67,
    FormatSigmaAPSC,
    FormatSigmaMerrill,
    FormatSigmaAPSH,
    Format3648,
    Format68,
    FormatTheLastOne,
}
impl From<u32> for CameraFormat {
    fn from(value: u32) -> Self {
        match value {
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_APSC => CameraFormat::FormatAPSC,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_FF => CameraFormat::FormatFF,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_MF => CameraFormat::FormatMF,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_APSH => CameraFormat::FormatAPSH,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_1INCH => CameraFormat::Format1INCH,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_1div2p3INCH => CameraFormat::Format1div2p3INCH,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_1div1p7INCH => CameraFormat::Format1div1p7INCH,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_FT => CameraFormat::FormatFT,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_CROP645 => CameraFormat::FormatCROP645,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_LeicaS => CameraFormat::FormatLeicaS,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_645 => CameraFormat::Format645,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_66 => CameraFormat::Format66,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_69 => CameraFormat::Format69,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_LF => CameraFormat::FormatLF,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_Leica_DMR => CameraFormat::FormatLeicaDMR,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_67 => CameraFormat::Format67,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_SigmaAPSC => CameraFormat::FormatSigmaAPSC,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_SigmaMerrill => {
                CameraFormat::FormatSigmaMerrill
            }
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_SigmaAPSH => CameraFormat::FormatSigmaAPSH,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_3648 => CameraFormat::Format3648,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_68 => CameraFormat::Format68,
            sys::LibRaw_camera_formats_LIBRAW_FORMAT_TheLastOne => CameraFormat::FormatTheLastOne,
            _ => CameraFormat::Unknown(value),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum CameraMount {
    Unknown(u32),
    Alpa,
    C,
    Canon_EF_M,
    Canon_EF_S,
    Canon_EF,
    Canon_RF,
    Contax_N,
    Contax645,
    FT,
    mFT,
    Fuji_GF,
    Fuji_GX,
    Fuji_X,
    Hasselblad_H,
    Hasselblad_V,
    Hasselblad_XCD,
    Leica_M,
    Leica_R,
    Leica_S,
    Leica_SL,
    Leica_TL,
    LPS_L,
    Mamiya67,
    Mamiya645,
    Minolta_A,
    Nikon_CX,
    Nikon_F,
    Nikon_Z,
    PhaseOne_iXM_MV,
    PhaseOne_iXM_RS,
    PhaseOne_iXM,
    Pentax_645,
    Pentax_K,
    Pentax_Q,
    RicohModule,
    Rollei_bayonet,
    Samsung_NX_M,
    Samsung_NX,
    Sigma_X3F,
    Sony_E,
    LF,
    DigitalBack,
    FixedLens,
    IL_UM,
    TheLastOne,
}
impl From<u32> for CameraMount {
    fn from(value: u32) -> Self {
        match value {
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Alpa => CameraMount::Alpa,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_C => CameraMount::C,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_EF_M => CameraMount::Canon_EF_M,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_EF_S => CameraMount::Canon_EF_S,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_EF => CameraMount::Canon_EF,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_RF => CameraMount::Canon_RF,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Contax_N => CameraMount::Contax_N,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Contax645 => CameraMount::Contax645,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_FT => CameraMount::FT,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_mFT => CameraMount::mFT,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Fuji_GF => CameraMount::Fuji_GF,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Fuji_GX => CameraMount::Fuji_GX,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Fuji_X => CameraMount::Fuji_X,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Hasselblad_H => CameraMount::Hasselblad_H,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Hasselblad_V => CameraMount::Hasselblad_V,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Hasselblad_XCD => CameraMount::Hasselblad_XCD,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_M => CameraMount::Leica_M,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_R => CameraMount::Leica_R,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_S => CameraMount::Leica_S,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_SL => CameraMount::Leica_SL,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_TL => CameraMount::Leica_TL,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_LPS_L => CameraMount::LPS_L,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Mamiya67 => CameraMount::Mamiya67,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Mamiya645 => CameraMount::Mamiya645,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Minolta_A => CameraMount::Minolta_A,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Nikon_CX => CameraMount::Nikon_CX,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Nikon_F => CameraMount::Nikon_F,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Nikon_Z => CameraMount::Nikon_Z,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_PhaseOne_iXM_MV => CameraMount::PhaseOne_iXM_MV,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_PhaseOne_iXM_RS => CameraMount::PhaseOne_iXM_RS,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_PhaseOne_iXM => CameraMount::PhaseOne_iXM,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Pentax_645 => CameraMount::Pentax_645,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Pentax_K => CameraMount::Pentax_K,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Pentax_Q => CameraMount::Pentax_Q,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_RicohModule => CameraMount::RicohModule,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Rollei_bayonet => CameraMount::Rollei_bayonet,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Samsung_NX_M => CameraMount::Samsung_NX_M,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Samsung_NX => CameraMount::Samsung_NX,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Sigma_X3F => CameraMount::Sigma_X3F,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Sony_E => CameraMount::Sony_E,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_LF => CameraMount::LF,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_DigitalBack => CameraMount::DigitalBack,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_FixedLens => CameraMount::FixedLens,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_IL_UM => CameraMount::IL_UM,
            sys::LibRaw_camera_mounts_LIBRAW_MOUNT_TheLastOne => CameraMount::TheLastOne,
            _ => CameraMount::Unknown(value),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum FocalType {
    Undefined,
    Unknown(i16),
    Zoom,
    Fixed,
}
impl From<i16> for FocalType {
    fn from(value: i16) -> Self {
        match value {
            -1 => FocalType::Undefined,
            1 => FocalType::Zoom,
            2 => FocalType::Fixed,
            _ => FocalType::Unknown(value),
        }
    }
}

impl LensInfo {
    pub fn lens_make(&self) -> Option<String> {
        string_from((self.inner).LensMake.as_ptr())
    }
    pub fn lens(&self) -> Option<String> {
        string_from((self.inner).makernotes.Lens.as_ptr())
    }
    pub fn exif_max_ap(&self) -> f32 {
        (self.inner).EXIF_MaxAp
    }
    pub fn lens_id(&self) -> u64 {
        (self.inner).makernotes.LensID
    }
    pub fn cam_id(&self) -> u64 {
        (self.inner).makernotes.CamID
    }
    pub fn lens_format(&self) -> CameraFormat {
        CameraFormat::from((self.inner).makernotes.LensFormat as u32)
    }
    pub fn camera_format(&self) -> CameraFormat {
        CameraFormat::from((self.inner).makernotes.CameraFormat as u32)
    }
    pub fn lens_mount(&self) -> CameraMount {
        CameraMount::from((self.inner).makernotes.LensMount as u32)
    }
    pub fn camera_mount(&self) -> CameraMount {
        CameraMount::from((self.inner).makernotes.CameraMount as u32)
    }
    pub fn body(&self) -> Option<String> {
        string_from((self.inner).makernotes.body.as_ptr())
    }
    pub fn focal_type(&self) -> FocalType {
        FocalType::from((self.inner).makernotes.FocalType)
    }
    pub fn lens_features_pre(&self) -> Option<String> {
        string_from((self.inner).makernotes.LensFeatures_pre.as_ptr())
    }
    pub fn lens_features_suf(&self) -> Option<String> {
        string_from((self.inner).makernotes.LensFeatures_suf.as_ptr())
    }
    pub fn min_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MinFocal)
    }
    pub fn max_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MaxFocal)
    }
    pub fn max_ap4_min_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MaxAp4MinFocal)
    }
    pub fn max_ap4_max_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MaxAp4MaxFocal)
    }
    pub fn min_ap4_min_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MinAp4MinFocal)
    }
    pub fn min_ap4_max_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MinAp4MaxFocal)
    }
    pub fn max_ap(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MaxAp)
    }
    pub fn min_ap(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MinAp)
    }
    pub fn cur_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.CurFocal)
    }
    pub fn cur_ap(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.CurAp)
    }
    pub fn max_ap4_cur_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MaxAp4CurFocal)
    }
    pub fn min_ap4_cur_focal(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.MinAp4CurFocal)
    }
    pub fn lens_f_stop(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.LensFStops)
    }
    pub fn teleconverter_id(&self) -> Option<u64> {
        non_zero((self.inner).makernotes.TeleconverterID)
    }
    pub fn adapter_id(&self) -> Option<u64> {
        non_zero((self.inner).makernotes.AdapterID)
    }
    pub fn attachment_id(&self) -> Option<u64> {
        non_zero((self.inner).makernotes.AttachmentID)
    }
    pub fn teleconverter(&self) -> Option<String> {
        string_from((self.inner).makernotes.Teleconverter.as_ptr())
    }
    pub fn adapter(&self) -> Option<String> {
        string_from((self.inner).makernotes.Adapter.as_ptr())
    }
    pub fn attachment(&self) -> Option<String> {
        string_from((self.inner).makernotes.Attachment.as_ptr())
    }
    pub fn focal_units(&self) -> u16 {
        (self.inner).makernotes.FocalUnits
    }
    pub fn focal_length_in_35mm_format(&self) -> Option<f32> {
        non_zero((self.inner).makernotes.FocalLengthIn35mmFormat)
    }
}
