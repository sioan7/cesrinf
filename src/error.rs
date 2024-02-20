use crate::domain::Msg;

#[derive(Debug)]
pub enum Error {
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
        decoded_msgs: Vec<Msg<'static>>,
        cause: Box<Error>,
    },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
