use std::{error, fmt, io};

use crate::decoder::{Decoder, Error as DecodeError};
use crate::input::reader::Reader;
use crate::sample::Sample;

#[derive(Debug)]
pub enum Error {
    Decode(DecodeError),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Decode(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
        }
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Error {
        Error::Decode(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Input {
    reader: Box<dyn Reader>,
    decoder: Box<dyn Decoder>,
}

impl Input {
    pub fn new(reader: Box<dyn Reader>, decoder: Box<dyn Decoder>) -> Self {
        Self { reader, decoder }
    }

    pub fn read(&mut self) -> Result<Sample> {
        let mut buf = Vec::new();
        self.reader.read(&mut buf)?;
        let sample = self.decoder.decode(&buf)?;
        Ok(sample)
    }
}
