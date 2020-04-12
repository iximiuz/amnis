use std::io::{self, BufRead, BufReader, Write};

pub struct Output {}

impl Output {
    pub fn stdout() -> Self {
        let mut output = io::BufWriter::new(io::stdout());
    }
}
