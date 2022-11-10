//! Error definitions.
use std::error::Error;
use std::{fmt, result};

/// A specialized Result type for this library.
pub type Result<T, E = EasySegmenterError> = result::Result<T, E>;

/// Errors in this library.
#[derive(Debug)]
pub enum EasySegmenterError {
    /// Contains [`InputError`].
    Input(InputError),

    /// The error variant for [`toml::de::Error`].
    TomlDecode(toml::de::Error),
}

impl fmt::Display for EasySegmenterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Input(e) => e.fmt(f),
            Self::TomlDecode(e) => e.fmt(f),
        }
    }
}

impl Error for EasySegmenterError {}

impl EasySegmenterError {
    pub(crate) fn input<S>(msg: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::Input(InputError {
            msg: msg.as_ref().to_string(),
        })
    }
}

/// Error used when the input argument is invalid.
#[derive(Debug)]
pub struct InputError {
    msg: String,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputError: {}", &self.msg)
    }
}

impl From<toml::de::Error> for EasySegmenterError {
    fn from(error: toml::de::Error) -> Self {
        Self::TomlDecode(error)
    }
}
