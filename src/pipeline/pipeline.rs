use crate::stream::Stream;
use crate::error::{Result};

struct Pipeline {
    queue: Vec<Box<dyn Stream>>;
}

impl Pipeline {
    pub fn run(&mut self) -> Result<()> {
        loop {
            for s in self.queue.iter() {
                s.process();
            }
        }
    }
}
