use cesrinf::CesrParser;
use wasm_bindgen::prelude::*;

use crate::{
    domain::WrappedParsedData,
    error::{JsResult, Result},
};

#[wasm_bindgen(js_name = CesrParser)]
pub struct ParserWrapper(String);

#[wasm_bindgen(js_class = CesrParser)]
impl ParserWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(stream: String) -> ParserWrapper {
        ParserWrapper(stream)
    }

    pub fn parse(&self) -> Result<WrappedParsedData> {
        let cp = CesrParser::new(&self.0).as_js()?;
        let pd = cp.parse().as_js()?;
        Ok(pd.into())
    }
}
