use std::collections::HashMap;

use chrono::DateTime;
use regex::Regex;

use amnis::input::Input;
use amnis::output::Output;

// Nginx log stream use case:
//   Show request rate (per second/minute/etc)
//   Show request rate by status code
//   Show request rate by method
//   Show request rate by URL
//   Show request rate by (method, URL, status code)
//
//   Input: stream, bin interval, group by tuple
//   Read input line by line
//   Parse lines to hashmaps
//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("amnis 0.0.1");

    let mut input = BufReader::new(io::stdin());

    let bin_interval = 60 * 60;
    // let group_by = ("method", "URL", "status_code");

    let pattern = r#"^[^\[]+\[(?P<time>[^]]+)\]\s+"(?P<method>[A-Z]+)\s+(?P<url>.+)\s+HTTP.+"\s+(?P<status_code>\d+)\s+"#;
    let re = Regex::new(pattern)?;

    let mut line_no = 0;
    let mut prev_bin = 0;
    let mut agg: HashMap<String, i64> = HashMap::new();
    loop {
        let mut vec = Vec::new();
        let len = input.read_until(b'\n', &mut vec)?;
        if len == 0 {
            break;
        }
        line_no += 1;

        let line = String::from_utf8(vec)?;
        if line == "\n" {
            continue;
        }

        let caps = match re.captures(&line) {
            Some(caps) => caps,
            None => {
                eprintln!("line {} pattern not found: {}", line_no, line);
                continue;
            }
        };

        let time = match caps.name("time") {
            Some(x) => x.as_str(),
            None => {
                eprintln!("line {} does't have time", line_no);
                continue;
            }
        };
        let timestamp = DateTime::parse_from_str(time, "%d/%h/%Y:%H:%M:%S %z")?.timestamp();

        let method = match caps.name("method") {
            Some(x) => x.as_str(),
            None => "<no method>",
        };
        let status_code = match caps.name("status_code") {
            Some(x) => x.as_str(),
            None => "<no status code>",
        };
        let url = match caps.name("url") {
            Some(x) => x.as_str(),
            None => "<no url>",
        };

        // write!(
        //     output,
        //     "DEBUG: line {}: len={}, time={}, timestamp={}, method={}, status_code={}, URL={}\n",
        //     line_no, len, time, timestamp, method, status_code, url,
        // )?;

        let bin = bin(timestamp, bin_interval);
        if bin != prev_bin {
            prev_bin = bin;
            if !agg.is_empty() {
                write!(output, "{} {:?}\n\n", prev_bin, agg)?;
            }
            agg.clear();
        }

        *agg.entry("total".to_owned()).or_insert(0) += 1;
        *agg.entry(method.to_owned()).or_insert(0) += 1;
        *agg.entry(status_code.to_owned()).or_insert(0) += 1;
        *agg.entry(url.to_owned()).or_insert(0) += 1;

        output.flush()?;
    }

    write!(output, "{} {:?}\n", prev_bin, agg)?;
    Ok(())
}

fn bin(key: i64, interval: i64) -> i64 {
    return (key / interval) * interval;
}
