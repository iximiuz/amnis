use std::rc::Rc;

use serde_json;

use crate::error::Result;
use crate::stream::{Inlet, Outlet, Point, Stream, StreamId};

pub struct JsonDecoder {
    id: StreamId,
    buf: Option<Rc<Point>>,
    upstreams: Vec<Rc<dyn Inlet>>,
}

impl JsonDecoder {
    pub fn new(id: StreamId) -> Self {
        Self {
            id: id,
            buf: None,
            upstreams: Vec::new(),
        }
    }
}

impl Inlet for JsonDecoder {
    fn push(&mut self, _origin: StreamId, point: Rc<Point>) {
        self.buf = Some(point);
    }
}

impl Outlet for JsonDecoder {
    fn pipe(&mut self, inlet: Rc<dyn Inlet>) {
        self.upstreams.push(inlet);
    }
}

impl Stream for JsonDecoder {
    fn id(&self) -> &StreamId {
        &self.id
    }

    fn process(&mut self) -> Result<()> {
        // let v = serde_json::from_slice(buf)?;
        // println!("JSON: {:?}", v);
        // Ok(Point::new())
        Ok(())
    }
}
