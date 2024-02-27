use cesrinf::CesrParser;
use ouroboros::self_referencing;
use wasm_bindgen::prelude::*;

use crate::error::{JsResult, Result};

#[self_referencing]
#[wasm_bindgen(js_name = CesrParser)]
pub struct ParserWrapper {
    owned_stream: String,
    #[borrows(owned_stream)]
    parser: CesrParser<'this>,
}

#[wasm_bindgen(js_class = CesrParser)]
impl ParserWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(stream: &str) -> Result<ParserWrapper> {
        let cp = CesrParser::new(stream).as_js()?;
        Ok(ParserWrapper(cp))
    }

    pub fn parse(&self) -> Result<String> {
        let pd = self.0.parse().as_js()?;
        Ok(pd)
    }
}
