use std::io;

use clap::Parser;

use cesrinf::CesrParser;

fn main() -> io::Result<()> {
    // https://trustoverip.github.io/tswg-cesr-specification/#stream-parsing-rules
    // These rules ease parsing by determining the shape of the data from the first tritet.
    // NOTE: Assumption 1: assume only T domain

    let args = Args::parse();

    let parser = CesrParser::new(&args.stream).unwrap();
    let dat = parser.parse().unwrap();
    println!("{:#?}", dat);

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// CESR-encoded stream in the T (text) domain
    stream: String,
}
