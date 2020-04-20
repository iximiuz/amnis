use std::rc::Rc;

use serde_json;

use crate::error::Result;
use crate::stream::{Inlet, Outlet, Point, Stream, StreamId};

pub struct JsonDecoder {
    buf: Option<Point>,
    upstreams: Vec<Rc<dyn Inlet>>,
}

impl JsonDecoder {
    pub fn new() -> Self {
        Self { buf: None }
    }
}

impl Inlet for JsonDecoder {
    fn push(&mut self, origin: StreamId, point: Point) {
        self.buf = Some(Point);
    }
}

impl Stream for JsonDecoder {
    fn process(&mut self) -> Result<()> {
        let v = serde_json::from_slice(buf)?;
        println!("JSON: {:?}", v);
        Ok(Point::new())
    }
}
