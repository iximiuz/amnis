use std::io::{self, BufReader};

use serde_json::Value;

use super::input::Input;
use super::reader::LineReader;
use crate::decoder;

impl Input {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let val: Value = serde_json::from_str(json)?;
        println!("{}", val["decode"]);
        Ok(Self::new(
            Box::new(LineReader::new(BufReader::new(io::stdin()))),
            Box::new(decoder::RegexDecoder::new("foo")?),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed() {
        match Input::from_json("foobar") {
            Err(err) => assert_eq!(err.to_string(), "expected ident at line 1 column 2"),
            _ => panic!("error expected when creating an input from malformed JSON"),
        }
    }

    #[test]
    fn decoder_regex() -> Result<(), Box<dyn std::error::Error>> {
        //   -i '{"re": "(\d+)\s(\w)", "multiline": {"re": "\d4-\d2\d2", "negative": true}, "t:ts": "$1:%Y-%m-%dT%H:%I:%S", "l:num": "$1:i"}' \
        let input = Input::from_json(r#"{"decode": "re"}"#)?;
        assert_eq!(input.decoder_kind(), "RegexDecoder");
        assert_eq!(input.reader_kind(), "LineReader");
        Ok(())
    }
}
