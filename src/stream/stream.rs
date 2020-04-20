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

pub struct StreamId(u32);

pub trait Stream {
    fn id(&self) -> StreamId;
    fn process(&mut self) -> Result<()>;
}

pub trait Inlet {
    fn push(&mut self, origin: StreamId, point: Point);
}

pub trait Outlet {
    fn pipe(&mut self, inlet: Box<dyn Inlet>);
}
