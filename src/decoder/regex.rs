use regex::bytes::Regex as RE;

use crate::sample::Sample;

use super::{Decoder, Error, Result};

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
            None => return Err(Error {}),
            Some(x) => x,
        };
        Err(Error {})
    }
}
