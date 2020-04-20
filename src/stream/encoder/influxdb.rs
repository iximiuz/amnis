// Implements InfluxDB line protocol
// see https://docs.influxdata.com/influxdb/latest/write_protocols/line_protocol_tutorial/

use crate::point::Point;

use super::{Encoder, Result};

pub struct LineProto {}

impl LineProto {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for LineProto {
    fn encode(&self, _sample: Point) -> Result<Vec<u8>> {
        Ok(vec![48, 49, 50])
    }
}
