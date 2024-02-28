use js_sys::Error as JsError;
use wasm_bindgen::JsValue;

pub type Result<T> = core::result::Result<T, JsValue>;

pub(crate) trait JsResult<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_js(self) -> core::result::Result<T, JsError>;
}

impl<T> JsResult<T> for core::result::Result<T, cesrinf::error::Error> {
    fn as_js(self) -> core::result::Result<T, JsError> {
        self.map_err(|e| JsError::new(&e.to_string()))
    }
}
