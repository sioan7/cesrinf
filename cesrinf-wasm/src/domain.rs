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

#[derive(Debug, PartialEq)]
pub struct MatterCodeage<'a> {
    pub selector: &'a str,
    pub description: &'a str,
    pub hs: usize,
    pub ss: usize,
    pub fs: Option<usize>,
}

impl<'a> MatterCodeage<'a> {
    pub fn cs(&self) -> usize {
        todo!()
    }

    pub fn vs(&self) -> Option<usize> {
        todo!()
    }

    pub fn ls(&self) -> usize {
        todo!()
    }

    pub fn ps(&self) -> usize {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct IndexerCodeage<'a> {
    pub selector: &'a str,
    pub description: &'a str,
    pub hs: usize,
    pub ss: usize,
    pub os: usize,
    pub fs: usize,
}

impl<'a> IndexerCodeage<'a> {
    pub fn cs(&self) -> usize {
        todo!()
    }

    pub fn vs(&self) -> usize {
        todo!()
    }

    pub fn ms(&self) -> usize {
        todo!()
    }

    pub fn ls(&self) -> usize {
        todo!()
    }

    pub fn ps(&self) -> usize {
        todo!()
    }
}
