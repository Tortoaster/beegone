use wasm_bindgen::prelude::wasm_bindgen;

use crate::{inbound::wasm::error::BeegoneError, Species};

#[wasm_bindgen(js_name = "Species")]
#[derive(Copy, Clone)]
pub enum WasmSpecies {
    Drone = "drone",
    Worker = "worker",
    Nurse = "nurse",
    Builder = "builder",
    Explorer = "explorer",
    Queen = "queen",
    Guard = "guard",
}

impl From<Species> for WasmSpecies {
    fn from(value: Species) -> Self {
        match value {
            Species::Drone => WasmSpecies::Drone,
            Species::Worker => WasmSpecies::Worker,
            Species::Nurse => WasmSpecies::Nurse,
            Species::Builder => WasmSpecies::Builder,
            Species::Explorer => WasmSpecies::Explorer,
            Species::Queen => WasmSpecies::Queen,
            Species::Guard => WasmSpecies::Guard,
        }
    }
}

impl TryFrom<WasmSpecies> for Species {
    type Error = BeegoneError;

    fn try_from(value: WasmSpecies) -> Result<Self, Self::Error> {
        match value {
            WasmSpecies::Drone => Ok(Species::Drone),
            WasmSpecies::Worker => Ok(Species::Worker),
            WasmSpecies::Nurse => Ok(Species::Nurse),
            WasmSpecies::Builder => Ok(Species::Builder),
            WasmSpecies::Explorer => Ok(Species::Explorer),
            WasmSpecies::Queen => Ok(Species::Queen),
            WasmSpecies::Guard => Ok(Species::Guard),
            _ => Err(BeegoneError::InvalidSpecies),
        }
    }
}
