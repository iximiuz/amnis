use super::reader::Reader;
use crate::error::Result;

pub struct Input {
    reader: Box<dyn Reader>,
}

impl Input {
    pub fn new(reader: Box<dyn Reader>) -> Self {
        Self { reader }
    }

    // pub fn read(&mut self) -> Result<Point, ReadError> {
    //     let mut buf = Vec::new();
    //     self.reader.read(&mut buf)?;
    //     let point = self.decoder.decode(&buf)?;
    //     Ok(point)
    // }
}
