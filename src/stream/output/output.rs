use super::writer::Writer;
use crate::error::Result;
use crate::stream::producer::{Inbound, Producer};
use crate::stream::Point;

pub struct Output {
    writer: Box<dyn Writer>,
}

impl Output {
    pub fn new(writer: Box<dyn Writer>) -> Self {
        Self { writer }
    }
}

impl Producer for Output {
    fn produce(&mut self, _buf: &mut dyn Inbound) -> Result<Vec<Point>> {
        // pub fn write(&mut self, sample: Point) -> Result<()> {
        //     let buf = self.encoder.encode(sample)?;
        //     self.writer.write(&buf)?;
        //     Ok(())
        // }
        println!("Output::produce()");
        Ok(Vec::new())
    }
}
