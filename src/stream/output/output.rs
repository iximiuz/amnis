use super::writer::Writer;

pub struct Output {
    writer: Box<dyn Writer>,
}

impl Output {
    pub fn new(writer: Box<dyn Writer>) -> Self {
        Self { writer }
    }

    // pub fn write(&mut self, sample: Point) -> Result<()> {
    //     let buf = self.encoder.encode(sample)?;
    //     self.writer.write(&buf)?;
    //     Ok(())
    // }
}
