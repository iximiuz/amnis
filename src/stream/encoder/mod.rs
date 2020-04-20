pub mod influxdb;

use std::{error, fmt};

use crate::point::Point;

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

pub trait Encoder {
    fn encode(&self, sample: Point) -> Result<Vec<u8>>;
}
