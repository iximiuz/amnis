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
type Key = String;
type Line = String;

struct Record(Key, LineNum, Line);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Bucket((Key, LineNum), VecDeque<Record>);

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

    let mut buckets = BinaryHeap::new();
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

        if let Some(Bucket(_, records)) = buckets.iter().find(|&b| match b.back() {
            Some() => last_key <= &key,
            None => unreachable!("empty bucket is not allowed"),
        }) {
            buckets
                .peek_mut(idx)
                .unwrap()
                .push_back(Record(key, line_no, line));
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

// use std::collections::HashMap;
// use chrono::DateTime;

// let bin_interval = 60 * 60;
// let group_by = ("method", "URL", "status_code");

// let mut line_no = 0;
// let mut prev_bin = 0;
// let mut agg: HashMap<String, i64> = HashMap::new();
// loop {
//     let mut vec = Vec::new();
//     let len = input.read_until(b'\n', &mut vec)?;
//     if len == 0 {
//         break;
//     }
//     line_no += 1;

//     let line = String::from_utf8(vec)?;
//     if line == "\n" {
//         continue;
//     }

//     let caps = match re.captures(&line) {
//         Some(caps) => caps,
//         None => {
//             eprintln!("line {} pattern not found: {}", line_no, line);
//             continue;
//         }
//     };

//     let time = match caps.name("time") {
//         Some(x) => x.as_str(),
//         None => {
//             eprintln!("line {} does't have time", line_no);
//             continue;
//         }
//     };
//     let timestamp = DateTime::parse_from_str(time, "%d/%h/%Y:%H:%M:%S %z")?.timestamp();

//     let method = match caps.name("method") {
//         Some(x) => x.as_str(),
//         None => "<no method>",
//     };
//     let status_code = match caps.name("status_code") {
//         Some(x) => x.as_str(),
//         None => "<no status code>",
//     };
//     let url = match caps.name("url") {
//         Some(x) => x.as_str(),
//         None => "<no url>",
//     };

//     // write!(
//     //     output,
//     //     "DEBUG: line {}: len={}, time={}, timestamp={}, method={}, status_code={}, URL={}\n",
//     //     line_no, len, time, timestamp, method, status_code, url,
//     // )?;

//     let bin = bin(timestamp, bin_interval);
//     if bin != prev_bin {
//         prev_bin = bin;
//         if !agg.is_empty() {
//             write!(output, "{} {:?}\n\n", prev_bin, agg)?;
//         }
//         agg.clear();
//     }

//     *agg.entry("total".to_owned()).or_insert(0) += 1;
//     *agg.entry(method.to_owned()).or_insert(0) += 1;
//     *agg.entry(status_code.to_owned()).or_insert(0) += 1;
//     *agg.entry(url.to_owned()).or_insert(0) += 1;

//     output.flush()?;
// }

// write!(output, "{} {:?}\n", prev_bin, agg)?;

// fn bin(key: i64, interval: i64) -> i64 {
//     return (key / interval) * interval;
// }
