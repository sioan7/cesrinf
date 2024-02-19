use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // https://trustoverip.github.io/tswg-cesr-specification/#stream-parsing-rules
    // These rules ease parsing by determining the shape of the data from the first tritet.
    // NOTE: Assumption 1: assume only T domain

    Ok(())
}
