use std::collections::{HashMap, HashSet};

use regex::bytes::Regex;

use super::{
    Attribute, DecodeError, Decoder, DescriptorField, DescriptorLabel, DescriptorTimestamp,
};
use crate::error::{Error, Result};
use crate::point::Point;

type Position = usize;

pub struct CaptureField {
    pos: Option<Position>,
    descriptor: DescriptorField,
}

impl CaptureField {
    pub fn new(name: String, data_type: String, pos: Option<Position>) -> Self {
        Self {
            pos,
            descriptor: DescriptorField { name, data_type },
        }
    }
}

pub struct CaptureLabel {
    pos: Option<Position>,
    descriptor: DescriptorLabel,
}

pub struct CaptureTimestamp {
    pos: Option<Position>,
    descriptor: DescriptorTimestamp,
}

pub struct RegexDecoder {
    re: Regex,
    captures: HashMap<Position, Attribute>,
}

impl RegexDecoder {
    pub fn new(
        re_pattern: &str,
        timestamp: Option<CaptureTimestamp>,
        labels: Vec<CaptureLabel>,
        fields: Vec<CaptureField>,
    ) -> Result<Self> {
        let re = Self::compile_regular_expression(re_pattern)?;
        // println!("{:?}", re.capture_names().collect::<Vec<_>>());
        Self::validate_capture_positions(timestamp, labels, fields, re.captures_len())?;

        for pos in &[1..re.captures_len()] {}

        Ok(Self {
            re,
            captures: HashMap::new(),
        })
    }

    fn compile_regular_expression(re_pattern: &str) -> Result<Regex> {
        let re = Regex::new(re_pattern).map_err(|e| ("Bad regex pattern", e))?;
        match re.captures_len() {
            0 | 1 => Err("regex must have at least one capture".into()),
            _ => Ok(re),
        }
    }

    fn validate_capture_positions(
        timestamp: Option<CaptureTimestamp>,
        labels: Vec<CaptureLabel>,
        fields: Vec<CaptureField>,
        max_capture: Position,
    ) -> Result<()> {
        let mut unique = HashSet::new();

        for pos in timestamp
            .map_or(Vec::new(), |cap| vec![cap.pos])
            .iter()
            .cloned()
            .chain(labels.iter().map(|cap| cap.pos))
            .chain(fields.iter().map(|cap| cap.pos))
            .filter_map(|pos| pos)
        {
            if pos > max_capture {
                return Err(format!(
                    "out of bound capture position {}; max allowed position is {}",
                    pos, max_capture
                )
                .into());
            }
            if !unique.insert(pos) {
                return Err(format!("ambiguous capture position {}", pos).into());
            }
        }

        Ok(())
    }
}

impl Decoder for RegexDecoder {
    fn decode(&self, buf: &[u8]) -> std::result::Result<Point, DecodeError> {
        let caps = match self.re.captures(buf) {
            None => return Err(DecodeError::Format(Box::new(Error::new("no match")))),
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
        let decoder = Regex::new(re).expect("lol");
    }
}
