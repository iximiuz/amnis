use std::rc::Rc;

use super::point::Point;
use crate::error::Result;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct StreamId(i32);

impl From<i32> for StreamId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl StreamId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

pub trait Stream {
    fn id(&self) -> &StreamId;

    fn write(&mut self, origin: StreamId, point: Rc<Point>);

    fn read(&mut self) -> Result<Option<Point>>;
}
