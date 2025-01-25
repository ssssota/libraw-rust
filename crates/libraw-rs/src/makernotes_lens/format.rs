use libraw_sys as sys;

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
