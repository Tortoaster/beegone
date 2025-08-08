use wasm_bindgen::prelude::wasm_bindgen;

use crate::Piece;
use crate::inbound::wasm::bee::WasmBee;

#[wasm_bindgen(js_name = "Piece")]
#[derive(Copy, Clone)]
pub struct WasmPiece {
    /// If `None`, the piece is a wall.
    #[wasm_bindgen(readonly)]
    pub bee: Option<WasmBee>,
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

impl From<WasmPiece> for Piece {
    fn from(value: WasmPiece) -> Self {
        match value.bee {
            None => Piece::Wall,
            Some(bee) => Piece::Bee(bee.into()),
        }
    }
}
