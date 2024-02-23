use cesrinfo::{domain::Msg, CesrParser};

pub fn parse(stream: &str) -> Vec<Msg> {
    CesrParser::new(stream).unwrap().parse().unwrap().msgs
}

/// A tree of codes.
#[derive(Debug, PartialEq)]
pub struct CTree<'a> {
    /// selector
    pub s: &'a str,
    /// start index
    pub i: usize,
    /// indexed messages
    pub m: Option<Vec<CTree<'a>>>,
}

pub fn flatten_codes_recursively(msgs: Vec<Msg>) -> Vec<CTree> {
    let mut trees = Vec::new();
    for msg in msgs {
        let (s, i) = match msg {
            Msg::Counter => todo!(),
            Msg::Matter { codeage, istart } => (codeage.selector, istart),
            Msg::Indexer { codeage, istart } => (codeage.selector, istart),
        };
        trees.push(CTree { s, i, m: None })
        // TODO: support indexed messages
    }
    trees
}
