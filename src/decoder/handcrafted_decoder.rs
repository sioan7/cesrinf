use crate::domain::{indexer, matter, Msg, ParsedData};

use super::{is_digit, is_uppercase_letter};

pub fn decode(stream: &str) -> Result<ParsedData, String> {
    let mut msgs = vec![];
    let mut next_tokens = stream;
    let mut next_token_start_idx: usize = 0;
    loop {
        let maybe_token = match token(next_tokens, next_token_start_idx, false) {
            Ok(t) => t,
            Err(e) => return Err(format!("error parsing token: {e}")),
        };

        match maybe_token {
            Some((msg, nts, ntsi)) => {
                msgs.push(msg);
                next_tokens = nts;
                next_token_start_idx = ntsi;
            }
            None => break,
        }
    }

    Ok(ParsedData { stream, msgs })
}

fn token(
    stream: &str,
    token_start_idx: usize,
    indexed: bool,
) -> Result<Option<(Msg, &str, usize)>, String> {
    if stream.is_empty() {
        return Ok(None);
    }

    if indexed {
        let selector = &stream[..1];
        if let "A" | "B" | "C" | "D" = selector {
            let codeage = indexer::codeage(selector).unwrap();
            let fs = codeage.fs;
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            return Ok(Some((
                Msg::Indexer {
                    codeage,
                    istart: token_start_idx,
                },
                next_tokens,
                next_token_start_idx,
            )));
        }

        if stream.len() < 2 {
            return Err(format!("invalid token `{}`", stream));
        }

        let selector = &stream[..2];
        if let "0A" | "0B" | "2A" | "2B" | "2C" | "2D" | "3A" | "3B" = selector {
            // TODO: fill with signatures from both indices
            let codeage = indexer::codeage(selector).unwrap();
            let fs = codeage.fs;
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            return Ok(Some((
                Msg::Indexer {
                    codeage,
                    istart: token_start_idx,
                },
                next_tokens,
                next_token_start_idx,
            )));
        }

        return Err("selector of indexed token didn't match".to_owned());
    }

    let universal_selector = &stream[..1];
    match universal_selector {
        x if is_uppercase_letter(x) => {
            let codeage = matter::codeage(x).unwrap();
            // TODO: calculate the variable size from ss
            let fs = codeage.fs.unwrap_or(stream.len());
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        "0" => {
            let selector = &stream[..1];
            let codeage = matter::codeage(selector).unwrap();
            let fs = codeage.fs.unwrap_or(stream.len());
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        "1" => {
            let selector = &stream[..4];
            let codeage = matter::codeage(selector).unwrap();
            let fs = codeage.fs.unwrap_or(stream.len());
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        x if is_digit(x) => Ok(None),
        "-" => {
            let selector = &stream[1..2];
            match selector {
                "-" => {
                    // TODO: this is the version
                }
                "0" => {}
                x if is_uppercase_letter(x) => {}
                _ => return Err(format!("unrecognized selector {}", &stream[..2])),
            }

            Ok(None)
        }
        "_" => {
            let codeage = matter::codeage(universal_selector).unwrap();
            let fs = codeage.fs.unwrap_or(stream.len());
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        _ => Err(format!(
            "unrecognized universal selector {}",
            universal_selector
        )),
    }
}
