use std::io::{self, BufReader};

use serde_json::Value as JsonValue;

use super::input::Input;
use super::reader;
use crate::decoder;
use crate::error::Result;

impl Input {
    pub fn from_json(json: &str) -> Result<Self> {
        let config = serde_json::from_str(json).map_err(|e| ("Malformed JSON", e))?;
        Ok(Self::new(create_reader(&config)?, create_decoder(&config)?))
    }
}

fn create_reader(_config: &JsonValue) -> Result<Box<dyn reader::Reader>> {
    Ok(Box::new(reader::LineReader::new(BufReader::new(
        io::stdin(),
    ))))
}

fn create_decoder(config: &JsonValue) -> Result<Box<dyn decoder::Decoder>> {
    use JsonValue::*;

    match config["decode"] {
        String(ref kind) => match kind.as_str() {
            "re" => Ok(Box::new(decoder::RegexDecoder::new("foo")?)),
            "json" => Ok(Box::new(decoder::JsonDecoder::new())),
            _ => Err("unsupported \"decode\" type".into()),
        },
        Null => Err("missing \"decode\" attribute".into()),
        _ => Err("\"decode\" attribute must be string".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_json() {
        match Input::from_json("foobar") {
            Err(e) => assert_eq!(
                e.to_string(),
                "Malformed JSON. Source error: expected ident at line 1 column 2"
            ),
            _ => panic!("error expected when creating an input from malformed JSON"),
        }
    }

    #[test]
    fn input_create_simple() -> Result<()> {
        //   -i '{"re": "(\d+)\s(\w)", "multiline": {"re": "\d4-\d2\d2", "negative": true}, "t:ts": "$1:%Y-%m-%dT%H:%I:%S", "l:num": "$1:i"}' \
        let input = Input::from_json(r#"{"decode": "re"}"#)?;
        assert_eq!(input.decoder_kind(), "RegexDecoder");
        assert_eq!(input.reader_kind(), "LineReader");
        Ok(())
    }

    #[test]
    fn decoder_create_regex() -> Result<()> {
        let d = create_decoder(&serde_json::from_str("{}").map_err(|e| From::from(e))?)?;
        println!("{}", d.kind());
        Ok(())
    }
}
