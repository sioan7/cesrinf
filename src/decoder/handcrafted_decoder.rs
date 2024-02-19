use crate::tables::{indexer, matter};

use super::{is_digit, is_uppercase_letter, Msg, ParsedData};

pub fn decode(mut stream: &str) -> Result<ParsedData, String> {
    let mut matteri = vec![];
    loop {
        let possible_token = match token(stream, false) {
            Ok(t) => t,
            Err(e) => return Err(format!("error parsing token: {e}")),
        };

        match possible_token {
            Some((matter, remainder)) => {
                matteri.push(matter);
                stream = remainder;
            }
            None => break,
        }
    }

    Ok(ParsedData { stream, matteri })
}

pub fn token(stream: &str, indexed: bool) -> Result<Option<(Msg, &str)>, String> {
    if stream.is_empty() {
        return Ok(None);
    }

    if indexed {
        let selector = &stream[0..1];
        if let "A" | "B" | "C" | "D" = selector {
            let codeage = indexer::codeage(selector).unwrap();
            return Ok(None);
        }

        let selector = &stream[0..2];
        if let "0A" | "0B" | "2A" | "2B" | "2C" | "2D" | "3A" | "3B" = selector {
            return Ok(None);
        }

        return Err("selector of indexed token didn't match".to_owned());
    }

    let selector = &stream[0..1];
    match selector {
        x if is_uppercase_letter(x) => {
            let codeage = matter::codeage(x).unwrap();
            let remainder = &stream[codeage.fs..];
            Ok(Some((Msg::Matter { codeage }, remainder)))
        }
        x if is_digit(x) => Ok(None),
        "-" => Ok(None),
        "_" => Ok(None),
        _ => Err(format!("unrcognized selector {}", selector)),
    }
}
