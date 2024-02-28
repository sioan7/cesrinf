use cesrinf::{domain::Msg, CesrParser};

pub fn parse(stream: &str) -> Vec<Msg> {
    let parser = match CesrParser::new(stream) {
        Ok(x) => x,
        Err(e) => panic!("{:#?}", e),
    };
    let pd = match parser.parse() {
        Ok(x) => x,
        Err(e) => panic!("{:#?}", e),
    };
    pd.msgs
}

/// A tree of codes.
#[derive(Debug, PartialEq)]
pub struct CTree {
    /// selector
    pub s: &'static str,
    /// start index
    pub i: usize,
    /// indexed messages
    pub m: Option<Vec<CTree>>,
}

pub fn flatten_codes_recursively(msgs: Vec<Msg>) -> Vec<CTree> {
    let mut trees = Vec::new();
    for msg in msgs {
        let (s, i, m) = match msg {
            Msg::Counter => todo!(),
            Msg::Matter {
                codeage,
                istart,
                indexed,
            } => (codeage.selector, istart, indexed),
            Msg::Indexer { codeage, istart } => (codeage.selector, istart, None),
        };
        trees.push(CTree {
            s,
            i,
            m: m.map(flatten_codes_recursively),
        })
    }
    trees
}
