use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    #[error("invalid move")]
    Invalid,
}

#[cfg(feature = "wasm-bindgen")]
impl From<Error> for wasm_bindgen::JsValue {
    fn from(value: Error) -> Self {
        serde_wasm_bindgen::to_value(&value)
            .unwrap_or_else(|_| wasm_bindgen::JsValue::from_str("failed to generate error message"))
    }
}
