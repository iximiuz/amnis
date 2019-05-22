use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

extern crate regex;
use regex::Regex;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    pattern: String,

    #[structopt(short = "n", long = "size", conflicts_with = "records")]
    bytes: Option<i32>,

    #[structopt(short = "r", long = "records", conflicts_with = "bytes")]
    records: Option<i32>,

    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

type LineNum = i64;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Record(String, LineNum, String);

// impl<'s> Ord for Record<'s> {
//     fn cmp(&self, other: &Record) -> Ordering {
//         (self.0, self.1).cmp(&(other.0, other.1))
//     }
// }

// select url, status, count(1) where status >= 400 group by date::(m), url, status;

// define buffer in terms of number of records, diff of record values, memory size?
// for each line:
//   extract key 
//   find first bucket record can be placed to
//   if bucket found:
//     push record to bucket
//   else:
//     create a new bucket
//     push record to it
//     append new bucket to heap of buckets
//   if at max capacity
//     pop record with least key value
//     output it
//     rearrange heap of buckets

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();

    let mut input: Box<BufRead> = match args.path {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut output = io::BufWriter::new(io::stdout());
    let re = Regex::new(&args.pattern)?;

    let mut buckets: BinaryHeap<VecDeque<Record>> = BinaryHeap::new();
    let mut line_no: LineNum = 0;
    loop {
        line_no += 1;

        let mut vec = Vec::new();
        let len = input.read_until(b'\n', &mut vec)?;
        if len == 0 {
            break;
        }

        let line = String::from_utf8(vec)?;
        let mut caps = re.captures_iter(&line);
        let key = match caps.next() {
            Some(cap) => match cap.get(1) {
                Some(m) => m.as_str().to_string(),
                None => {
                    eprintln!("pattern {} does't have a group", args.pattern);
                    break;
                }
            },
            None => {
                eprintln!("line {} pattern not found: {}", line_no, line);
                continue;
            }
        };

        if caps.next().is_some() {
            eprintln!("line {} more than 1 pattern match {}", line_no, line);
            continue;
        }

        if let Some(idx) = buckets.iter().enumerate().find(|(i, &b)| match b.back() {
            Some(Record(last_key, _, _)) => last_key <= &key,
            None => unreachable!("empty bucket is not allowed"),
        }) {
            buckets.peek_mut(idx).unwrap().push_back(Record(key, line_no, line));
        } else {
            let mut bucket = VecDeque::new();
            bucket.push_back(Record(key, line_no, line));
            buckets.push(bucket);
        }

        // let mut matches = HashMap::new();
        // for (i, caps) in re.captures_iter(&line).enumerate() {
        //     for (j, cap) in caps.iter().enumerate() {
        //         matches.insert(1000 * i + j, cap);
        //     }
        // }
        // if matches.len() != 0 {
        //     write!(output, "{:#?}", matches)?;
        //     output.flush()?;
        // }
    }
    Ok(())
}
