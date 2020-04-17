use serde_json;

use crate::point::Point;

use super::{DecodeError, Decoder};

impl From<serde_json::error::Error> for DecodeError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::Format(Box::new(err))
    }
}

pub struct JsonDecoder {}

impl JsonDecoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for JsonDecoder {
    fn decode(&self, buf: &[u8]) -> Result<Point, DecodeError> {
        let v = serde_json::from_slice(buf)?;
        println!("JSON: {:?}", v);
        Ok(Point::new())
    }

    fn kind(&self) -> &str {
        "JsonDecoder"
    }
}
