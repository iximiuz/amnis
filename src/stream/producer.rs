use std::rc::Rc;

use super::point::Point;
use super::stream::{Stream, StreamId};
use crate::error::Result;

pub trait Producer {
    fn produce(&mut self, buf: &mut dyn Inbound) -> Result<Vec<Point>>;
}

pub trait Inbound {
    fn get(&mut self, origin: StreamId) -> Option<Rc<Point>>;
}

pub struct ProducerStream {
    id: StreamId,
    producer: Box<dyn Producer>,
    inbound: Buffer,
    outbound: Buffer,
}

impl ProducerStream {
    pub fn new<S: Into<StreamId>>(id: S, producer: Box<dyn Producer>) -> Self {
        Self {
            id: id.into(),
            producer: producer,
            inbound: Buffer::new(),
            outbound: Buffer::new(),
        }
    }
}

impl Stream for ProducerStream {
    fn id(&self) -> &StreamId {
        &self.id
    }

    fn write(&mut self, origin: StreamId, point: Rc<Point>) {
        self.inbound.enque(origin, point);
    }

    fn read(&mut self) -> Result<Option<Point>> {
        if self.outbound.size() == 0 {
            for point in self.producer.produce(&mut self.inbound)? {
                self.outbound.enque(self.id, Rc::new(point));
            }
        }
        Ok(self.outbound.deque())
    }
}

struct Buffer {}

impl Buffer {
    fn new() -> Self {
        Self {}
    }

    fn enque(&mut self, _origin: StreamId, _point: Rc<Point>) {}

    fn deque(&mut self) -> Option<Point> {
        None
    }

    fn size(&self) -> usize {
        return 0;
    }
}

impl Inbound for Buffer {
    fn get(&mut self, _origin: StreamId) -> Option<Rc<Point>> {
        None
    }
}
