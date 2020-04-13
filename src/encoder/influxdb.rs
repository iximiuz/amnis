// Implements InfluxDB line protocol
// see https://docs.influxdata.com/influxdb/latest/write_protocols/line_protocol_tutorial/

use crate::sample::Sample;

use super::{Encoder, Result};

pub struct LineProto {}

impl LineProto {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for LineProto {
    fn encode(&self, _sample: Sample) -> Result<Vec<u8>> {
        Ok(vec![48, 49, 50])
    }
}
