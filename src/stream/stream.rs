use std::rc::Rc;

use super::point::Point;
use crate::error::Result;

// Example:
//   input implements Outlet
//       input.pipe(decoder);
//   decoder implements Inlet + Outlet
//       decoder.pipe(encoder);
//   encoder implements Inlet + Outlet
//       encoder.pipe(output);
//   output implements Inlet

pub trait Inlet {
    fn push(&mut self, origin: StreamId, point: Rc<Point>);
}

pub trait Outlet {
    fn pipe(&mut self, inlet: Rc<dyn Inlet>);
}

pub trait Inbound {
    fn get(origin: StreamId) -> Option<Rc<Point>>;
}

pub trait Producer {
    fn produce(&mut self, buf: &mut dyn Inbound) -> Result<Option<Point>>;
}

pub struct StreamId(u32);

pub struct Stream {
    id: StreamId,
    producer: Box<dyn Producer>,
    buffer: Buffer,
    upstreams: Vec<Rc<dyn Inlet>>,
}

impl Stream {
    pub fn new(id: StreamId, producer: Box<dyn Producer>) -> Self {
        Self {
            id: id,
            producer: producer,
            buffer: Buffer::new(),
        }
    }

    pub fn id(&self) -> &StreamId {
        self.id
    }

    pub fn produce(&mut self) -> Result<()> {
        self.producer.produce(&self.buffer)
    }
}

impl Inlet for Stream {
    fn push(&mut self, _: StreamId, point: Rc<Point>) {
        self.buf = Some(point);
    }
}

impl Outlet for Stream {
    fn pipe(&mut self, inlet: Rc<dyn Inlet>) {
        self.upstreams.push(inlet);
    }
}

struct Buffer {}

impl Buffer {
    fn new() -> Self {
        Self {}
    }

    fn add(&mut self, _point: Rc<Point>) {}
}

impl Inbound for Buffer {
    fn get(&mut self, origin: StreamId) -> Option<Rc<Point>> {
        None
    }
}
