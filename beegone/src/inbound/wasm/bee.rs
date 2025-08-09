use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    inbound::wasm::{color::WasmColor, error::InvalidBee, species::WasmSpecies},
    Bee,
};

#[wasm_bindgen(js_name = "Bee")]
#[derive(Copy, Clone)]
pub struct WasmBee {
    #[wasm_bindgen(readonly)]
    pub color: WasmColor,
    #[wasm_bindgen(readonly)]
    pub species: WasmSpecies,
}

#[wasm_bindgen(js_class = "Bee")]
impl WasmBee {
    #[wasm_bindgen(constructor)]
    pub fn new(color: WasmColor, species: WasmSpecies) -> Self {
        Self { color, species }
    }
}

impl From<Bee> for WasmBee {
    fn from(value: Bee) -> Self {
        Self {
            color: value.color().into(),
            species: value.species().into(),
        }
    }
}

impl TryFrom<WasmBee> for Bee {
    type Error = InvalidBee;

    fn try_from(value: WasmBee) -> Result<Self, Self::Error> {
        Ok(Bee::new(value.color.try_into()?, value.species.try_into()?))
    }
}
