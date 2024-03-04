use std::io;

use clap::Parser;
use colored::Colorize;

use cesrinf::CesrParser;

fn main() -> io::Result<()> {
    // https://trustoverip.github.io/tswg-cesr-specification/#stream-parsing-rules
    // These rules ease parsing by determining the shape of the data from the first tritet.
    // NOTE: Assumption 1: assume only T domain

    let args = Args::parse();
    eprintln!("Parsing {}", args.stream.magenta());

    let parser = match CesrParser::new(&args.stream) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e.to_string().red());
            std::process::exit(1);
        }
    };
    let dat = match parser.parse() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e.to_string().red());
            std::process::exit(1);
        }
    };
    println!("{}", format!("{:#?}", dat).green());

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// CESR-encoded stream in the T (text) domain
    stream: String,
}
