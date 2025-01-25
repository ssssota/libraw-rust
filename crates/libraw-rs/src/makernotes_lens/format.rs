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
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_APSC => CameraFormat::FormatAPSC,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_FF => CameraFormat::FormatFF,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_MF => CameraFormat::FormatMF,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_APSH => CameraFormat::FormatAPSH,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_1INCH => CameraFormat::Format1INCH,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_1div2p3INCH => {
                CameraFormat::Format1div2p3INCH
            }
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_1div1p7INCH => {
                CameraFormat::Format1div1p7INCH
            }
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_FT => CameraFormat::FormatFT,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_CROP645 => CameraFormat::FormatCROP645,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_LeicaS => CameraFormat::FormatLeicaS,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_645 => CameraFormat::Format645,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_66 => CameraFormat::Format66,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_69 => CameraFormat::Format69,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_LF => CameraFormat::FormatLF,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_Leica_DMR => {
                CameraFormat::FormatLeicaDMR
            }
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_67 => CameraFormat::Format67,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_SigmaAPSC => {
                CameraFormat::FormatSigmaAPSC
            }
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_SigmaMerrill => {
                CameraFormat::FormatSigmaMerrill
            }
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_SigmaAPSH => {
                CameraFormat::FormatSigmaAPSH
            }
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_3648 => CameraFormat::Format3648,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_68 => CameraFormat::Format68,
            libraw_sys::LibRaw_camera_formats_LIBRAW_FORMAT_TheLastOne => {
                CameraFormat::FormatTheLastOne
            }
            _ => CameraFormat::Unknown(value),
        }
    }
}
