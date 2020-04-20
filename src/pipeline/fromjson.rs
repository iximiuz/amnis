use std::io::{self, BufReader};

use serde_json::{Map, Value as JsonValue};

use super::pipeline::Pipeline;
use crate::error::Result;
// use crate::stream::decoder;
// use crate::stream::input::{reader, Input};

impl Pipeline {
    pub fn from_json(input: &str, streams: &[&str], outputs: &[&str]) -> Result<Self> {
        // let config = json_parse(json)?;
        // Ok(Self::new(create_reader(&config)?, create_decoder(&config)?))
        Ok(Self::new())
    }
}

// type JsonConfig = Map<String, JsonValue>;
//
// fn json_parse(json: &str) -> Result<JsonConfig> {
//     use JsonValue::*;
//
//     match serde_json::from_str(json) {
//         Ok(Object(obj)) => Ok(obj),
//         Err(err) => Err(("Malformed JSON", err).into()),
//         _ => Err("JSON object expected".into()),
//     }
// }
//
// fn json_get_str_attr<'a>(config: &'a JsonConfig, attr: &str) -> Result<&'a str> {
//     match config.get(attr) {
//         Some(JsonValue::String(val)) => Ok(val),
//         None => Err(format!(r#"missing "{}" attribute"#, attr).into()),
//         _ => Err(format!(r#""{}" attribute must be string"#, attr).into()),
//     }
// }
//
// fn create_reader(_config: &JsonConfig) -> Result<Box<dyn reader::Reader>> {
//     Ok(Box::new(reader::LineReader::new(BufReader::new(
//         io::stdin(),
//     ))))
// }
//
// fn create_decoder(config: &JsonConfig) -> Result<Box<dyn decoder::Decoder>> {
//     match json_get_str_attr(config, "decode")? {
//         "re" => create_decoder_re(config),
//         "json" => create_decoder_json(config),
//         _ => Err(r#"unsupported "decode" type"#.into()),
//     }
// }
//
// fn create_decoder_re(config: &JsonConfig) -> Result<Box<dyn decoder::Decoder>> {
//     // TODO: parse timestamp field
//     // TODO: parse all label fields
//     // TODO: parse all regular fields
//     Ok(Box::new(decoder::RegexDecoder::new(
//         json_get_str_attr(config, "re")?,
//         None,
//         vec![],
//         vec![],
//     )?))
// }
//
// fn create_decoder_json(_config: &JsonConfig) -> Result<Box<dyn decoder::Decoder>> {
//     Ok(Box::new(decoder::JsonDecoder::new()))
// }
//
// #[cfg(test)]
// mod tests {
//     use serde_json::json;
//
//     use super::*;
//
//     #[test]
//     fn input_create_simple() {
//         //   -i '{"re": "(\d+)\s(\w)", "multiline": {"re": "\d4-\d2\d2", "negative": true}, "t:ts": "$1:%Y-%m-%dT%H:%I:%S", "l:num": "$1:i"}' \
//         Input::from_json(r#"{"decode": "re", "re": "(.*)"}"#).expect("Input creation must succeed");
//     }
//
//     #[test]
//     fn input_create_malformed_json() {
//         match Input::from_json("foobar") {
//             Err(e) => assert_eq!(
//                 e.to_string(),
//                 "Malformed JSON. Source error: expected ident at line 1 column 2"
//             ),
//             _ => panic!("error expected when creating an input from malformed JSON"),
//         }
//     }
//
//     #[test]
//     fn decoder_create_regex() {
//         create_decoder(&cfg(json!({"decode": "re", "re": "(.*)"})))
//             .expect("Decoder creating must succeed");
//     }
//
//     fn cfg(c: JsonValue) -> JsonConfig {
//         match c {
//             JsonValue::Object(obj) => obj,
//             _ => unreachable!(),
//         }
//     }
// }
