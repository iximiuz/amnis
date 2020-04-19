use std::{error, fmt};

use crate::point::Point;

mod json;
pub use self::json::JsonDecoder;

mod regex;
pub use self::regex::RegexDecoder;

pub trait Decoder {
    fn decode(&self, buf: &[u8]) -> Result<Point, DecodeError>;
    fn kind(&self) -> &str;
}

#[derive(Debug)]
pub enum DecodeError {
    Format(Box<dyn error::Error>),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

enum Attribute {
    Field(DescriptorField),
    Label(DescriptorLabel),
    Timestamp(DescriptorTimestamp),
}

struct DescriptorTimestamp {
    name: String,
    format: String,
}

struct DescriptorLabel {
    name: String,
    data_type: String,
}

struct DescriptorField {
    name: String,
    data_type: String,
}
