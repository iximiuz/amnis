// use serde_json;

use crate::error::Result;
use crate::stream::producer::{Inbound, Producer};
use crate::stream::Point;

pub struct JsonDecoder {}

impl JsonDecoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Producer for JsonDecoder {
    fn produce(&mut self, _buf: &mut dyn Inbound) -> Result<Vec<Point>> {
        // let v = serde_json::from_slice(buf)?;
        // println!("JSON: {:?}", v);
        // Ok(Point::new())
        println!("JsonDecoder::produce()");
        Ok(Vec::new())
    }
}
