use super::reader::Reader;
use crate::error::Result;
use crate::stream::{Inbound, Point, Producer};

pub struct Input {
    reader: Box<dyn Reader>,
}

impl Input {
    pub fn new(reader: Box<dyn Reader>) -> Self {
        Self { reader }
    }
}

impl Producer for Input {
    fn produce(&mut self, _buf: &mut dyn Inbound) -> Result<Option<Point>> {
        //     let mut buf = Vec::new();
        //     self.reader.read(&mut buf)?;
        //     let point = self.decoder.decode(&buf)?;
        Ok(None)
    }
}
