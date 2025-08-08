use std::collections::BTreeMap;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Board, Pos};
use crate::inbound::wasm::piece::WasmPiece;

#[wasm_bindgen(js_name = "Board")]
#[derive(Clone)]
pub struct WasmBoard {
    pieces: BTreeMap<Pos, WasmPiece>,
}

#[wasm_bindgen(js_class = "Board")]
impl WasmBoard {
    pub fn get(&self, pos: &Pos) -> Option<WasmPiece> {
        self.pieces.get(pos).cloned()
    }

    pub fn positions() -> Vec<Pos> {
        Board::positions().collect()
    }
}

impl From<Board> for WasmBoard {
    fn from(value: Board) -> Self {
        Self {
            pieces: value
                .pieces()
                .iter()
                .map(|(&k, &v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<WasmBoard> for Board {
    fn from(value: WasmBoard) -> Self {
        Board::new_with_pieces(
            value
                .pieces
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        )
    }
}
