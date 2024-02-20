use colored::Colorize;

use crate::domain::Msg;

#[derive(Debug)]
pub enum Error<'a> {
    EmptyStream,
    UnsupportedCodeCodex(String),
    InvalidSelector {
        stream_remainder: String,
        token_start_idx: usize,
    },
    InvalidStream {
        stream_remainder: String,
        token_start_idx: usize,
    },
    DecodeFailure {
        stream: String,
        decoded_msgs: Vec<Msg<'a>>,
        cause: Box<Error<'a>>,
    },
}

impl<'a> std::error::Error for Error<'a> {}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            EmptyStream => write!(f, "empty stream"),
            UnsupportedCodeCodex(msg) => write!(f, "{msg}"),
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
        }
    }
}
