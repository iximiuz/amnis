use std::{error, fmt};

use regex::bytes::Regex;

use super::{DecodeError, Decoder};
use crate::point::Point;

#[derive(Debug)]
struct StringError(&'static str);

impl error::Error for StringError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub struct RegexDecoder {
    re: Regex,
}

impl RegexDecoder {
    pub fn new(re: &str) -> std::result::Result<Self, Box<dyn error::Error>> {
        let compiled_re = Regex::new(re)?;
        let tmp: Vec<_> = compiled_re.capture_names().collect();
        println!("{:?}", tmp);
        println!("{:?}", compiled_re.capture_locations());
        Ok(Self { re: compiled_re })
    }
}

impl Decoder for RegexDecoder {
    fn decode(&self, buf: &[u8]) -> Result<Point, DecodeError> {
        let caps = match self.re.captures(buf) {
            None => return Err(DecodeError::Format(Box::new(StringError("no match")))),
            Some(x) => x,
        };
        println!("REGEX: {:?}", caps);
        Ok(Point::new())
    }

    fn kind(&self) -> &str {
        "RegexDecoder"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let re = r#"^[^\[]+\[(?P<time>[^]]+)\]\s+"([A-Z]+)\s+(?P<url>.+)\s+HTTP.+"\s+(?P<status_code>\d+)\s+"#;
        let decoder = Regex::new(re);
        assert_eq!(2 + 2, 4);
    }
}
