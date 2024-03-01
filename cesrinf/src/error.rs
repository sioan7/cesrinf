use colored::Colorize;

use crate::domain::Msg;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyStream,
    UnsupportedCodeCodex(String),
    InvalidCountChar(char),
    InvalidCountNum(usize),
    InvalidCountStream {
        selector: String,
        count_stream: String,
        token_start_idx: usize,
    },
    InvalidSelector {
        stream_remainder: String,
        token_start_idx: usize,
    },
    InvalidStream {
        stream_remainder: String,
        token_start_idx: usize,
    },
    SelectorNotFound {
        selector: String,
        stream_remainder: String,
        token_start_idx: usize,
    },
    DecodeFailure {
        stream: String,
        decoded_msgs: Vec<Msg>,
        cause: Box<Error>,
    },
    SerializationFailure(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            EmptyStream => write!(f, "empty stream"),
            UnsupportedCodeCodex(msg) => write!(f, "{msg}"),
            InvalidCountChar(c) => write!(f, "invalid count char `{}`", c.to_string().red()),
            InvalidCountNum(c) => write!(f, "invalid count number `{}`", c.to_string().red()),
            InvalidCountStream {
                selector,
                count_stream,
                token_start_idx,
            } => write!(
                f,
                "invalid count stream `{}` for selector `{}` at index `{}`",
                count_stream.red(),
                selector.yellow(),
                token_start_idx.to_string().yellow(),
            ),
            InvalidSelector {
                stream_remainder,
                token_start_idx,
            } => write!(
                f,
                "invalid selector at index {}: `{}`",
                token_start_idx.to_string().yellow(),
                stream_remainder.red(),
            ),
            InvalidStream {
                stream_remainder,
                token_start_idx,
            } => write!(
                f,
                "invalid remainig stream at index {}: `{}`",
                token_start_idx.to_string().yellow(),
                stream_remainder.red(),
            ),
            SelectorNotFound {
                selector,
                stream_remainder,
                token_start_idx,
            } => write!(
                f,
                "wrong `{}` selector or not supported\nat index {} with remaining stream `{}`",
                selector.red(),
                token_start_idx.to_string().yellow(),
                stream_remainder.red(),
            ),
            DecodeFailure {
                stream,
                decoded_msgs,
                cause,
            } => write!(
                f,
                "error decoding stream `{}`\ndue to {}\ncould decode the following:\n{:#?}",
                stream.red(),
                cause,
                decoded_msgs,
            ),
            SerializationFailure(s) => {
                write!(f, "serialization failue: {s}")
            }
        }
    }
}
