use std::{error, fmt, io};

use crate::decoder::{DecodeError, Decoder};
use crate::input::reader::Reader;
use crate::point::Point;

pub struct Input {
    reader: Box<dyn Reader>,
    decoder: Box<dyn Decoder>,
}

impl Input {
    pub fn new(reader: Box<dyn Reader>, decoder: Box<dyn Decoder>) -> Self {
        Self { reader, decoder }
    }

    pub fn read(&mut self) -> Result<Point, ReadError> {
        let mut buf = Vec::new();
        self.reader.read(&mut buf)?;
        let point = self.decoder.decode(&buf)?;
        Ok(point)
    }

    pub fn decoder_kind(&self) -> &str {
        self.decoder.kind()
    }

    pub fn reader_kind(&self) -> &str {
        self.reader.kind()
    }
}

#[derive(Debug)]
pub enum ReadError {
    Decode(DecodeError),
    Io(io::Error),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ReadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Decode(ref e) => Some(e),
            Self::Io(ref e) => Some(e),
        }
    }
}

impl From<DecodeError> for ReadError {
    fn from(err: DecodeError) -> Self {
        Self::Decode(err)
    }
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
