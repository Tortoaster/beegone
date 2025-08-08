use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Error)]
#[error("invalid color")]
pub struct InvalidColor;

#[wasm_bindgen]
#[derive(Debug, Error)]
#[error("invalid species")]
pub struct InvalidSpecies;

#[wasm_bindgen]
#[derive(Debug, Error)]
pub enum InvalidBee {
    #[error("invalid color")]
    InvalidColor,
    #[error("invalid species")]
    InvalidSpecies,
}

impl From<InvalidColor> for InvalidBee {
    fn from(_: InvalidColor) -> Self {
        InvalidBee::InvalidColor
    }
}

impl From<InvalidSpecies> for InvalidBee {
    fn from(_: InvalidSpecies) -> Self {
        InvalidBee::InvalidSpecies
    }
}

#[wasm_bindgen]
#[derive(Debug, Error)]
pub enum PerformError {
    #[error("invalid color")]
    InvalidColor,
    #[error("invalid species")]
    InvalidSpecies,
    #[error("invalid action")]
    InvalidAction,
}

impl From<InvalidColor> for PerformError {
    fn from(_: InvalidColor) -> Self {
        PerformError::InvalidColor
    }
}

impl From<InvalidSpecies> for PerformError {
    fn from(_: InvalidSpecies) -> Self {
        PerformError::InvalidSpecies
    }
}

impl From<InvalidBee> for PerformError {
    fn from(value: InvalidBee) -> Self {
        match value {
            InvalidBee::InvalidColor => PerformError::InvalidColor,
            InvalidBee::InvalidSpecies => PerformError::InvalidSpecies,
        }
    }
}
