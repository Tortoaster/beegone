use wasm_bindgen::prelude::wasm_bindgen;
use crate::Color;
use crate::inbound::wasm::error::InvalidColor;

#[wasm_bindgen(js_name = "Color")]
#[derive(Copy, Clone)]
pub enum WasmColor {
    Light = "light",
    Dark = "dark",
}

impl From<Color> for WasmColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Light => WasmColor::Light,
            Color::Dark => WasmColor::Dark,
        }
    }
}

impl TryFrom<WasmColor> for Color {
    type Error = InvalidColor;

    fn try_from(value: WasmColor) -> Result<Self, Self::Error> {
        match value {
            WasmColor::Light => Ok(Color::Light),
            WasmColor::Dark => Ok(Color::Dark),
            _ => Err(InvalidColor),
        }
    }
}
