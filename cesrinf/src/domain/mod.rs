pub mod counter;
pub mod indexer;
pub mod matter;

use matter::MatterCodeage;
use serde::Serialize;

use crate::error::Error;

use self::indexer::IndexerCodeage;

#[derive(Debug, Serialize)]
pub struct ParsedData {
    pub msgs: Vec<Msg>,
}

impl ParsedData {
    pub fn to_json_pretty(&self) -> Result<String, Error> {
        let res = if self.msgs.len() == 1 {
            let first_msg = self.msgs.first().unwrap();
            serde_json::to_string_pretty(first_msg)
        } else {
            serde_json::to_string_pretty(&self.msgs)
        };
        res.map_err(|e| Error::SerializationFailure(e.to_string()))
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Msg {
    Counter,
    Matter {
        codeage: MatterCodeage,
        istart: usize,
        indexed: Option<Vec<Msg>>,
    },
    Indexer {
        codeage: IndexerCodeage,
        istart: usize,
    },
}
