#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CameraMount {
    Unknown(libraw_sys::LibRaw_camera_mounts),
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
impl From<libraw_sys::LibRaw_camera_mounts> for CameraMount {
    fn from(value: libraw_sys::LibRaw_camera_mounts) -> Self {
        match value {
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Alpa => CameraMount::Alpa,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_C => CameraMount::C,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_EF_M => CameraMount::Canon_EF_M,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_EF_S => CameraMount::Canon_EF_S,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_EF => CameraMount::Canon_EF,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Canon_RF => CameraMount::Canon_RF,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Contax_N => CameraMount::Contax_N,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Contax645 => CameraMount::Contax645,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_FT => CameraMount::FT,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_mFT => CameraMount::mFT,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Fuji_GF => CameraMount::Fuji_GF,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Fuji_GX => CameraMount::Fuji_GX,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Fuji_X => CameraMount::Fuji_X,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Hasselblad_H => CameraMount::Hasselblad_H,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Hasselblad_V => CameraMount::Hasselblad_V,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Hasselblad_XCD => {
                CameraMount::Hasselblad_XCD
            }
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_M => CameraMount::Leica_M,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_R => CameraMount::Leica_R,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_S => CameraMount::Leica_S,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_SL => CameraMount::Leica_SL,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Leica_TL => CameraMount::Leica_TL,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_LPS_L => CameraMount::LPS_L,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Mamiya67 => CameraMount::Mamiya67,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Mamiya645 => CameraMount::Mamiya645,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Minolta_A => CameraMount::Minolta_A,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Nikon_CX => CameraMount::Nikon_CX,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Nikon_F => CameraMount::Nikon_F,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Nikon_Z => CameraMount::Nikon_Z,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_PhaseOne_iXM_MV => {
                CameraMount::PhaseOne_iXM_MV
            }
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_PhaseOne_iXM_RS => {
                CameraMount::PhaseOne_iXM_RS
            }
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_PhaseOne_iXM => CameraMount::PhaseOne_iXM,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Pentax_645 => CameraMount::Pentax_645,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Pentax_K => CameraMount::Pentax_K,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Pentax_Q => CameraMount::Pentax_Q,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_RicohModule => CameraMount::RicohModule,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Rollei_bayonet => {
                CameraMount::Rollei_bayonet
            }
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Samsung_NX_M => CameraMount::Samsung_NX_M,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Samsung_NX => CameraMount::Samsung_NX,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Sigma_X3F => CameraMount::Sigma_X3F,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_Sony_E => CameraMount::Sony_E,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_LF => CameraMount::LF,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_DigitalBack => CameraMount::DigitalBack,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_FixedLens => CameraMount::FixedLens,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_IL_UM => CameraMount::IL_UM,
            libraw_sys::LibRaw_camera_mounts_LIBRAW_MOUNT_TheLastOne => CameraMount::TheLastOne,
            _ => CameraMount::Unknown(value),
        }
    }
}
