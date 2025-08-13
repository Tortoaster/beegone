use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Error)]
pub enum BeegoneError {
    #[error("invalid color")]
    InvalidColor,
    #[error("invalid species")]
    InvalidSpecies,
    #[error("invalid action")]
    InvalidAction,
    #[error("invalid position")]
    InvalidPos,
}
