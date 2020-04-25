use super::reader::Reader;
use crate::error::Result;
use crate::stream::producer::{Inbound, Producer};
use crate::stream::Point;

pub struct Input {
    reader: Box<dyn Reader>,
}

impl Input {
    pub fn new(reader: Box<dyn Reader>) -> Self {
        Self { reader }
    }
}

impl Producer for Input {
    fn produce(&mut self, _buf: &mut dyn Inbound) -> Result<Vec<Point>> {
        println!("Input::produce()");
        let mut buf = Vec::new();
        self.reader.read(&mut buf).expect("panic!");
        Ok(vec![Point::new()])
    }
}
