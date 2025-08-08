use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Bee, Color, Species};

#[wasm_bindgen(js_name = "Bee")]
#[derive(Copy, Clone)]
pub struct WasmBee {
    #[wasm_bindgen(readonly)]
    pub color: Color,
    #[wasm_bindgen(readonly)]
    pub species: Species,
}

#[wasm_bindgen(js_class = "Bee")]
impl WasmBee {
    #[wasm_bindgen(constructor)]
    pub fn new(color: Color, species: Species) -> Self {
        Self { color, species }
    }
}

impl From<Bee> for WasmBee {
    fn from(value: Bee) -> Self {
        Self {
            color: value.color(),
            species: value.species(),
        }
    }
}

impl From<WasmBee> for Bee {
    fn from(value: WasmBee) -> Self {
        Bee::new(value.color, value.species)
    }
}
