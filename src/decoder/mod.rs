use crate::tables::matter::MatterCodeage;

pub mod handcrafted_decoder;

pub struct ParsedData<'a> {
    pub stream: &'a str,
    pub matteri: Vec<Msg<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Msg<'a> {
    Counter,
    Matter { codeage: MatterCodeage<'a> },
    Indexer,
}

pub fn is_uppercase_letter(x: &str) -> bool {
    matches!(
        x,
        "A" | "B"
            | "C"
            | "D"
            | "E"
            | "F"
            | "G"
            | "H"
            | "I"
            | "J"
            | "K"
            | "L"
            | "M"
            | "N"
            | "O"
            | "P"
            | "Q"
            | "R"
            | "S"
            | "T"
            | "U"
            | "V"
            | "W"
            | "X"
            | "Y"
            | "Z"
    )
}

pub fn is_lowercase_letter(x: &str) -> bool {
    matches!(
        x,
        "a" | "b"
            | "c"
            | "d"
            | "e"
            | "f"
            | "g"
            | "h"
            | "i"
            | "j"
            | "k"
            | "l"
            | "m"
            | "n"
            | "o"
            | "p"
            | "q"
            | "r"
            | "s"
            | "t"
            | "u"
            | "v"
            | "w"
            | "x"
            | "y"
            | "z"
    )
}

pub fn is_digit(x: &str) -> bool {
    matches!(x, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
}
