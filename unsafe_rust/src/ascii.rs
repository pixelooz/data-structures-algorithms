use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Ascii(Vec<u8>);

impl Ascii {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|byte| !byte.is_ascii()) {
            return Err(NotAsciiError(bytes));
        }
        Ok(Ascii(bytes))
    }
}

impl From<Ascii> for String {
    fn from(ascii: Ascii) -> Self {
        unsafe { String::from_utf8_unchecked(ascii.0) }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotAsciiError(pub Vec<u8>);

impl Display for NotAsciiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Error for NotAsciiError {}
