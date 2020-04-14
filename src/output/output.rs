use std::{error, fmt, io};

use crate::encoder::{Encoder, Error as EncodeError};
use crate::sample::Sample;

use super::writer::Writer;

#[derive(Debug)]
pub enum Error {
    Encode(EncodeError),
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
            Error::Encode(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
        }
    }
}

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Error {
        Error::Encode(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Output {
    writer: Box<dyn Writer>,
    encoder: Box<dyn Encoder>,
}

impl Output {
    pub fn new(writer: Box<dyn Writer>, encoder: Box<dyn Encoder>) -> Self {
        Self { writer, encoder }
    }

    pub fn write(&mut self, sample: Sample) -> Result<()> {
        let buf = self.encoder.encode(sample)?;
        self.writer.write(&buf)?;
        Ok(())
    }
}
