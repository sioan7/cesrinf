use cesrinf::domain::{Msg, ParsedData};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone, js_name = "ParsedData")]
#[derive(Clone, Debug)]
pub struct WrappedParsedData {
    pub msgs: Vec<WrappedMsg>,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum MsgType {
    Matter = "matter",
    Indexer = "indexer",
}

#[wasm_bindgen(getter_with_clone, js_name = "Msg")]
#[derive(Clone, Debug)]
pub struct WrappedMsg {
    pub msg_type: MsgType,
    pub codeage: WrappedCodeage,
    pub istart: usize,
    pub indexed: Option<Vec<WrappedMsg>>,
}

#[wasm_bindgen(getter_with_clone, js_name = "Codeage")]
#[derive(Clone, Debug)]
pub struct WrappedCodeage {
    pub selector: String,
    pub description: String,
    pub hs: usize,
    pub ss: usize,
    pub fs: Option<usize>,
    pub cs: usize,
    pub vs: Option<usize>,
    pub ls: usize,
    pub ps: usize,
    pub os: Option<usize>,
    pub ms: Option<usize>,
}

impl From<ParsedData> for WrappedParsedData {
    fn from(value: ParsedData) -> Self {
        WrappedParsedData {
            msgs: value.msgs.into_iter().map(From::from).collect(),
        }
    }
}

impl From<Msg> for WrappedMsg {
    fn from(value: Msg) -> Self {
        match value {
            Msg::Counter => unimplemented!("counter message type is not yet implemented"),
            Msg::Matter {
                codeage,
                istart,
                indexed,
            } => WrappedMsg {
                msg_type: MsgType::Matter,
                codeage: WrappedCodeage {
                    selector: codeage.selector.to_owned(),
                    description: codeage.description.to_owned(),
                    hs: codeage.hs,
                    ss: codeage.ss,
                    fs: codeage.fs,
                    cs: codeage.cs(),
                    vs: codeage.vs(),
                    ls: codeage.ls(),
                    ps: codeage.ps(),
                    os: None,
                    ms: None,
                },
                istart,
                indexed: indexed.map(|x| x.into_iter().map(From::from).collect()),
            },
            Msg::Indexer { codeage, istart } => WrappedMsg {
                msg_type: MsgType::Indexer,
                codeage: WrappedCodeage {
                    selector: codeage.selector.to_owned(),
                    description: codeage.description.to_owned(),
                    hs: codeage.hs,
                    ss: codeage.ss,
                    fs: Some(codeage.fs),
                    cs: codeage.cs(),
                    vs: Some(codeage.vs()),
                    ls: codeage.ls(),
                    ps: codeage.ps(),
                    os: Some(codeage.os),
                    ms: Some(codeage.ms()),
                },
                istart,
                indexed: None,
            },
        }
    }
}
