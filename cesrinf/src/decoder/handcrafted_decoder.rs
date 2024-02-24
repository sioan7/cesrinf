use crate::{
    base64_conversions::TryFromBase64,
    domain::{indexer, matter, Msg, ParsedData},
    error::Error,
};

use super::{is_digit, is_uppercase_letter};

pub fn decode(stream: &str) -> Result<ParsedData, Error> {
    let mut msgs = vec![];
    let mut next_tokens = stream;
    let mut next_token_start_idx: usize = 0;
    loop {
        let maybe_token = match token(next_tokens, next_token_start_idx, false) {
            Ok(t) => t,
            Err(e) => {
                return Err(Error::DecodeFailure {
                    stream: stream.to_owned(),
                    decoded_msgs: msgs,
                    cause: Box::new(e),
                })
            }
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
) -> Result<Option<(Msg, &str, usize)>, Error> {
    if stream.is_empty() {
        return Ok(None);
    }

    if indexed {
        let selector = &stream[..=0];
        if let "A" | "B" | "C" | "D" = selector {
            let codeage = indexer::codeage(selector).ok_or_else(|| Error::SelectorNotFound {
                selector: selector.to_string(),
                stream_remainder: stream.to_owned(),
                token_start_idx,
            })?;
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
            return Err(Error::InvalidStream {
                stream_remainder: stream.to_owned(),
                token_start_idx,
            });
        }

        let selector = &stream[..=1];
        if let "0A" | "0B" | "2A" | "2B" | "2C" | "2D" | "3A" | "3B" = selector {
            // TODO: fill with signatures from both indices
            let codeage = indexer::codeage(selector).ok_or_else(|| Error::SelectorNotFound {
                selector: selector.to_string(),
                stream_remainder: stream.to_owned(),
                token_start_idx,
            })?;
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

        return Err(Error::InvalidSelector {
            stream_remainder: stream.to_owned(),
            token_start_idx,
        });
    }

    let universal_selector = &stream[..=0];
    match universal_selector {
        x if is_uppercase_letter(x) => {
            let codeage = matter::codeage(x).ok_or_else(|| Error::SelectorNotFound {
                selector: universal_selector.to_string(),
                stream_remainder: stream.to_owned(),
                token_start_idx,
            })?;
            let fs = codeage
                .fs
                .expect("fs is always defined for capital selectors");
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                    indexed: None,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        "0" => {
            let selector = &stream[..=1];
            let codeage = matter::codeage(selector).ok_or_else(|| Error::SelectorNotFound {
                selector: selector.to_string(),
                stream_remainder: stream.to_owned(),
                token_start_idx,
            })?;
            let fs = codeage
                .fs
                .expect("fs is always defined for leading '0' selectors");
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                    indexed: None,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        "1" => {
            let selector = &stream[..=3];
            let codeage = matter::codeage(selector).ok_or_else(|| Error::SelectorNotFound {
                selector: selector.to_string(),
                stream_remainder: stream.to_owned(),
                token_start_idx,
            })?;
            let fs = codeage
                .fs
                .expect("fs is always defined for leading '1' selectors");
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                    indexed: None,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        x if is_digit(x) => {
            // TODO: implement
            Ok(None)
        }
        "-" => {
            let selector = &stream[1..=1];
            match selector {
                "-" => {
                    // TODO: this is the version
                    let selector = &stream[..=1];
                    let codeage =
                        matter::codeage(selector).ok_or_else(|| Error::SelectorNotFound {
                            selector: selector.to_string(),
                            stream_remainder: stream.to_owned(),
                            token_start_idx,
                        })?;
                    let fs = codeage
                        .fs
                        .expect("fs is always defined for the '--' selector");
                    let next_tokens = &stream[fs..];
                    let next_token_start_idx = token_start_idx + fs;
                    Ok(Some((
                        Msg::Matter {
                            codeage,
                            istart: token_start_idx,
                            indexed: None,
                        },
                        next_tokens,
                        next_token_start_idx,
                    )))
                }
                x if x == "0" || is_uppercase_letter(x) => {
                    let selector = if x == "0" {
                        &stream[..=2]
                    } else {
                        &stream[..=1]
                    };
                    let codeage =
                        matter::codeage(selector).ok_or_else(|| Error::SelectorNotFound {
                            selector: selector.to_string(),
                            stream_remainder: stream.to_owned(),
                            token_start_idx,
                        })?;
                    // TODO: implement
                    let fs = codeage
                        .fs
                        .expect("fs is always defined for leading '-' selectors");
                    let count_start_idx = selector.len();
                    let count = &stream[count_start_idx..(count_start_idx + codeage.ss)];
                    let count =
                        usize::try_from_base64(count).map_err(|_| Error::InvalidCountStream {
                            selector: selector.to_owned(),
                            count_stream: count.to_owned(),
                            token_start_idx,
                        })?;

                    let mut msgs: Vec<Msg<'_>> = Vec::with_capacity(count);
                    let mut nts = &stream[fs..];
                    let mut ntsi = token_start_idx + fs;
                    for _ in 0..count {
                        let is_indexed =
                            matches!(selector, "-A" | "-0A" | "-B" | "-0B" | "-C" | "-0C");
                        let tk = token(nts, ntsi, is_indexed)?;
                        let Some((msg, next_tokens, next_token_start_idx)) = tk else {
                            todo!("return error because of premature termination")
                        };
                        msgs.push(msg);
                        nts = next_tokens;
                        ntsi = next_token_start_idx;
                    }

                    Ok(Some((
                        Msg::Matter {
                            codeage,
                            istart: token_start_idx,
                            indexed: Some(msgs),
                        },
                        nts,
                        ntsi,
                    )))
                }
                _ => Err(Error::InvalidSelector {
                    stream_remainder: stream.to_owned(),
                    token_start_idx,
                }),
            }
        }
        "_" => {
            let codeage =
                matter::codeage(universal_selector).ok_or_else(|| Error::SelectorNotFound {
                    selector: universal_selector.to_string(),
                    stream_remainder: stream.to_owned(),
                    token_start_idx,
                })?;
            let fs = codeage.fs.expect("fs is always defined for '_' selector");
            let next_tokens = &stream[fs..];
            let next_token_start_idx = token_start_idx + fs;
            Ok(Some((
                Msg::Matter {
                    codeage,
                    istart: token_start_idx,
                    indexed: None,
                },
                next_tokens,
                next_token_start_idx,
            )))
        }
        _ => Err(Error::InvalidSelector {
            stream_remainder: stream.to_owned(),
            token_start_idx,
        }),
    }
}
