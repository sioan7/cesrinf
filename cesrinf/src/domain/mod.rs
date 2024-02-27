pub mod counter;
pub mod indexer;
pub mod matter;

use matter::MatterCodeage;

use self::indexer::IndexerCodeage;

#[derive(Debug)]
pub struct ParsedData {
    pub msgs: Vec<Msg>,
}

#[derive(Debug, PartialEq)]
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
