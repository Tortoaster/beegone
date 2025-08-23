use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    inbound::wasm::{bee::WasmBee, error::BeegoneError},
    Piece,
};

#[wasm_bindgen(js_name = "Piece", inspectable)]
#[derive(Copy, Clone)]
pub struct WasmPiece {
    /// If `None`, the piece is a wall.
    #[wasm_bindgen(readonly)]
    pub bee: Option<WasmBee>,
}

#[wasm_bindgen(js_class = "Piece")]
impl WasmPiece {
    #[wasm_bindgen(js_name = "bee")]
    pub fn new_bee(bee: WasmBee) -> Self {
        Self { bee: Some(bee) }
    }

    #[wasm_bindgen(js_name = "wall")]
    pub fn new_wall() -> Self {
        Self { bee: None }
    }
}

impl From<Piece> for WasmPiece {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Wall => WasmPiece { bee: None },
            Piece::Bee(bee) => WasmPiece {
                bee: Some(bee.into()),
            },
        }
    }
}

impl TryFrom<WasmPiece> for Piece {
    type Error = BeegoneError;

    fn try_from(value: WasmPiece) -> Result<Self, Self::Error> {
        match value.bee {
            None => Ok(Piece::Wall),
            Some(bee) => Ok(Piece::Bee(bee.try_into()?)),
        }
    }
}
