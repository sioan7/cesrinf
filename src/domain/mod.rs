pub mod counter;
pub mod indexer;
pub mod matter;

use matter::MatterCodeage;

use self::indexer::IndexerCodeage;

#[derive(Debug)]
pub struct ParsedData<'a> {
    pub stream: &'a str,
    pub msgs: Vec<Msg<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Msg<'a> {
    Counter,
    Matter {
        codeage: MatterCodeage<'a>,
        istart: usize,
        indexed: Option<Vec<Msg<'a>>>,
    },
    Indexer {
        codeage: IndexerCodeage<'a>,
        istart: usize,
    },
}
