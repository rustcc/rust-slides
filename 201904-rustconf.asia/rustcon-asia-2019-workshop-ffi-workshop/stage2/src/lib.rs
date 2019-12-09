#![crate_type = "staticlib"]

#[macro_use]
mod macros;
pub mod png;

pub use png::png_encode_webp;

use std::fmt;

pub type ImageResult<T> = Result<T, ImageError>;

#[derive(Debug)]
pub enum ImageError {
    /// The Image is not formatted properly
    FormatError(String),

    /// The Decoder does not support this image format
    UnsupportedError(String),

    TranformError(String),

    ServiceError(String),

    NotFoundOrigin(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ImageError::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            ImageError::UnsupportedError(ref f) => write!(
                fmt,
                "The Decoder does not support the \
                 image format `{}`",
                f
            ),
            ImageError::TranformError(ref f) => write!(fmt, "Tranform error: {}", f),
            ImageError::ServiceError(ref f) => write!(fmt, "service error: {}", f),
            ImageError::NotFoundOrigin(ref f) => write!(fmt, "not found image: {}", f),
        }
    }
}

impl std::error::Error for ImageError {
    fn description(&self) -> &str {
        match *self {
            ImageError::FormatError(ref _e) => &"Format error",
            ImageError::UnsupportedError(ref _f) => {
                &"The Decoder does not support the image format"
            }
            ImageError::TranformError(ref _f) => &"Tranform error",
            ImageError::ServiceError(ref _f) => &"Service error",
            ImageError::NotFoundOrigin(ref _f) => &"not found image",
        }
    }
}
