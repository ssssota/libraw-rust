use std::{ffi::CStr, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPath,
    BadCrop,
    CancelledByCallback,
    DataError,
    FileUnsupported,
    InputClosed,
    IoError,
    MemPoolOverflow,
    NotImplemented,
    NoThumbnail,
    OutOfOrderCall,
    RequestForNonexistentImage,
    RequestForNonexistentThumbnail,
    TooBig,
    UnspecifiedError,
    UnsufficientMemory,
    UnsupportedThumbnail,
    Unknown(i32),
}

impl From<i32> for Error {
    fn from(code: i32) -> Self {
        match code {
            libraw_sys::LibRaw_errors_LIBRAW_BAD_CROP => Error::BadCrop,
            libraw_sys::LibRaw_errors_LIBRAW_CANCELLED_BY_CALLBACK => Error::CancelledByCallback,
            libraw_sys::LibRaw_errors_LIBRAW_DATA_ERROR => Error::DataError,
            libraw_sys::LibRaw_errors_LIBRAW_FILE_UNSUPPORTED => Error::FileUnsupported,
            libraw_sys::LibRaw_errors_LIBRAW_INPUT_CLOSED => Error::InputClosed,
            libraw_sys::LibRaw_errors_LIBRAW_IO_ERROR => Error::IoError,
            libraw_sys::LibRaw_errors_LIBRAW_MEMPOOL_OVERFLOW => Error::MemPoolOverflow,
            libraw_sys::LibRaw_errors_LIBRAW_NOT_IMPLEMENTED => Error::NotImplemented,
            libraw_sys::LibRaw_errors_LIBRAW_NO_THUMBNAIL => Error::NoThumbnail,
            libraw_sys::LibRaw_errors_LIBRAW_OUT_OF_ORDER_CALL => Error::OutOfOrderCall,
            libraw_sys::LibRaw_errors_LIBRAW_REQUEST_FOR_NONEXISTENT_IMAGE => {
                Error::RequestForNonexistentImage
            }
            libraw_sys::LibRaw_errors_LIBRAW_REQUEST_FOR_NONEXISTENT_THUMBNAIL => {
                Error::RequestForNonexistentThumbnail
            }
            libraw_sys::LibRaw_errors_LIBRAW_TOO_BIG => Error::TooBig,
            libraw_sys::LibRaw_errors_LIBRAW_UNSPECIFIED_ERROR => Error::UnspecifiedError,
            libraw_sys::LibRaw_errors_LIBRAW_UNSUFFICIENT_MEMORY => Error::UnsufficientMemory,
            libraw_sys::LibRaw_errors_LIBRAW_UNSUPPORTED_THUMBNAIL => Error::UnsupportedThumbnail,
            _ => Error::Unknown(code),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::InvalidPath => "Invalid path".to_string(),
            Error::BadCrop => error_string(libraw_sys::LibRaw_errors_LIBRAW_BAD_CROP),
            Error::CancelledByCallback => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_CANCELLED_BY_CALLBACK)
            }
            Error::DataError => error_string(libraw_sys::LibRaw_errors_LIBRAW_DATA_ERROR),
            Error::FileUnsupported => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_FILE_UNSUPPORTED)
            }
            Error::InputClosed => error_string(libraw_sys::LibRaw_errors_LIBRAW_INPUT_CLOSED),
            Error::IoError => error_string(libraw_sys::LibRaw_errors_LIBRAW_IO_ERROR),
            Error::MemPoolOverflow => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_MEMPOOL_OVERFLOW)
            }
            Error::NotImplemented => error_string(libraw_sys::LibRaw_errors_LIBRAW_NOT_IMPLEMENTED),
            Error::NoThumbnail => error_string(libraw_sys::LibRaw_errors_LIBRAW_NO_THUMBNAIL),
            Error::OutOfOrderCall => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_OUT_OF_ORDER_CALL)
            }
            Error::RequestForNonexistentImage => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_REQUEST_FOR_NONEXISTENT_IMAGE)
            }
            Error::RequestForNonexistentThumbnail => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_REQUEST_FOR_NONEXISTENT_THUMBNAIL)
            }
            Error::TooBig => error_string(libraw_sys::LibRaw_errors_LIBRAW_TOO_BIG),
            Error::UnspecifiedError => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_UNSPECIFIED_ERROR)
            }
            Error::UnsufficientMemory => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_UNSUFFICIENT_MEMORY)
            }
            Error::UnsupportedThumbnail => {
                error_string(libraw_sys::LibRaw_errors_LIBRAW_UNSUPPORTED_THUMBNAIL)
            }
            Error::Unknown(code) => format!("Unknown error: {}", code),
        }
    }
}

pub fn handle_error(code: i32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(Error::from(code))
    }
}

fn error_string(code: i32) -> String {
    let str = unsafe { CStr::from_ptr(libraw_sys::strerror(code)) };
    str.to_string_lossy().into_owned()
}
