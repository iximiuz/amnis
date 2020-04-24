use crate::error::Result;
use crate::stream::Stream;

pub struct Pipeline {
    queue: Vec<Box<Stream>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            for s in self.queue.iter_mut() {
                s.produce();
            }
        }
    }
}
