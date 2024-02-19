use crate::tables::codeage;

use super::{is_digit, is_uppercase_letter, Matter, ParsedData};

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

pub fn token(stream: &str, indexed: bool) -> Result<Option<(Matter, &str)>, String> {
    if indexed {
        let code = &stream[0..1];
        if let "A" | "B" | "C" | "D" = code {
            return Ok(None);
        }

        let code = &stream[0..2];
        if let "0A" | "0B" | "2A" | "2B" | "2C" | "2D" | "3A" | "3B" = code {
            return Ok(None);
        }

        return Err("code of indexed token didn't match".to_owned());
    }

    let code = &stream[0..1];
    match code {
        x if is_uppercase_letter(x) => {
            let codeage = codeage(x);
            Ok(None)
        }
        x if is_digit(x) => Ok(None),
        "-" => Ok(None),
        "_" => Ok(None),
        _ => Err(format!("unrcognized code {}", code)),
    }
}
