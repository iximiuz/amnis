use std::{error, fmt};

use regex::bytes::Regex as RE;

use crate::sample::Sample;

use super::{Decoder, Error, Result};

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

pub struct Regex {
    re: RE,
}

impl Regex {
    pub fn new(re: &str) -> std::result::Result<Self, Box<dyn error::Error>> {
        let compiled_re = RE::new(re)?;
        let tmp: Vec<_> = compiled_re.capture_names().collect();
        println!("{:?}", tmp);
        println!("{:?}", compiled_re.capture_locations());
        Ok(Self { re: compiled_re })
    }
}

impl Decoder for Regex {
    fn decode(&self, buf: &[u8]) -> Result<Sample> {
        let caps = match self.re.captures(buf) {
            None => return Err(Error::Format(Box::new(StringError("no match")))),
            Some(x) => x,
        };
        println!("REGEX: {:?}", caps);
        Ok(Sample::new())
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
