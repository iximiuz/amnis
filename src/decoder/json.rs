use serde_json;

use crate::sample::Sample;

use super::{Decoder, Error, Result};

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::Format(Box::new(err))
    }
}

pub struct Json {}

impl Json {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for Json {
    fn decode(&self, buf: &[u8]) -> Result<Sample> {
        let v = serde_json::from_slice(buf)?;
        println!("JSON: {:?}", v);
        Ok(Sample::new())
    }
}
