use std::io::{self, BufReader, BufWriter};

use regex::bytes::Regex;
use structopt::StructOpt;

use amnis::decoder::{self, Decoder};
use amnis::encoder::{self, Encoder};
use amnis::input::{Input, LineReader};
use amnis::output::{LineWriter, Output};

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

// amnis -d 're:(\d+) (\w) (\d+) (\d{2})' -t timestamp:$1:%Y-%m-%dT%H:%I:%S -l word:$2 -l num:$3:i -f uid:$4 -e influxdb:line_proto
// amnis -d json -e influxdb:line_proto
// amnis -d prom -e json
//
// Sample may have multiple fields.
// Query: rate(metric_name.field_name)
// Type annotations: :i - int
//                   :f - float
//                   :j - json
//                   :s - string (default, optional)
//
//
// tail -f /var/log/app.log | amnis \
//   -i '{"re": "(\d+)\s(\w)", "multiline": {"re": "\d4-\d2\d2", "negative": true}, "t:ts": "$1:%Y-%m-%dT%H:%I:%S", "l:num": "$1:i"}' \
//   -s '{"name": "foo:bar", "query": "sum($input:0.someField)"}' \
//   -s '{"name": "baz:abc", "transform": "... jq-alike program ..."}' \
//   -s '{"name": "baz:qux", "query": "foo:bar - baz:abc.otherField"}' \
//   -o '{"encode": "json"}' \
//   -o '{"encode": "influxdb.line_proto", "streams": ["foo:bar", "baz:*"], "file": "/var/log/foobarbaz.log"}'

#[derive(Debug, StructOpt)]
#[structopt(name = "amnis", about = "amnis command line arguments")]
struct CliOpt {
    #[structopt(long = "decode", short = "d")]
    decode: String,

    #[structopt(long = "encode", short = "e")]
    encode: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = CliOpt::from_args();

    let mut input = Input::new(
        Box::new(LineReader::new(BufReader::new(io::stdin()))),
        create_decoder(&opt)?,
    );

    let mut output = Output::new(
        Box::new(LineWriter::new(BufWriter::new(io::stdout()))),
        create_encoder(&opt)?,
    );

    // let mut pipeline = Pipeline::new(input, [], output);
    // pipeline.run()
    loop {
        let sample = input.read()?;
        output.write(sample)?;
    }

    Ok(())
}

fn create_decoder(opt: &CliOpt) -> Result<Box<dyn Decoder>, Box<dyn std::error::Error>> {
    // if opt.decode == "json" {
    //     return decoder::Json();
    // }
    let pattern = r#"^[^\[]+\[(?P<time>[^]]+)\]\s+"(?P<method>[A-Z]+)\s+(?P<url>.+)\s+HTTP.+"\s+(?P<status_code>\d+)\s+"#;
    let re = Regex::new(pattern)?;

    Ok(Box::new(decoder::Regex::new(re)))
}

fn create_encoder(opt: &CliOpt) -> Result<Box<dyn Encoder>, Box<dyn std::error::Error>> {
    Ok(Box::new(encoder::influxdb::LineProto::new()))
}
