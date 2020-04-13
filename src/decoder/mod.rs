use std::{error, fmt};

use crate::sample::Sample;

mod regex;
pub use self::regex::Regex;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Decoder {
    fn decode(&self, buf: &[u8]) -> Result<Sample>;
}
