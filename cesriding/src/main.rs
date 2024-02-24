use cesriding::basic_one_character_codes;
use colored::Colorize;

fn main() {
    let boccs = basic_one_character_codes();
    for c in boccs {
        println!("{}: {}", c.code.yellow(), c.material.green());
    }
}
