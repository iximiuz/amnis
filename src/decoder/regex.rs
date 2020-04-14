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
    pub fn new(re: RE) -> Self {
        Self { re }
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
