pub mod counter;
pub mod indexer;
pub mod matter;

use matter::MatterCodeage;

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
    },
    Indexer,
}
