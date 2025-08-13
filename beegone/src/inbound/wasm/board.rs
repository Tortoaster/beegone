use std::collections::BTreeMap;

use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    inbound::wasm::{error::BeegoneError, piece::WasmPiece, pos::WasmPos},
    Board,
};

#[wasm_bindgen(js_name = "Board")]
#[derive(Clone)]
pub struct WasmBoard {
    pieces: BTreeMap<WasmPos, WasmPiece>,
}

#[wasm_bindgen(js_class = "Board")]
impl WasmBoard {
    pub fn get(&self, pos: WasmPos) -> Result<Option<WasmPiece>, BeegoneError> {
        Ok(Board::try_from(self.clone())?
            .get(&pos.try_into()?)
            .map(Into::into))
    }

    #[wasm_bindgen(unchecked_return_type = "Pos[]")]
    pub fn positions() -> Result<Array, BeegoneError> {
        let positions = Board::positions()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<WasmPos>, _>>()?;
        Ok(positions.into_iter().map(JsValue::from).collect())
    }
}

impl TryFrom<Board> for WasmBoard {
    type Error = BeegoneError;

    fn try_from(value: Board) -> Result<Self, Self::Error> {
        Ok(Self {
            pieces: value
                .pieces()
                .iter()
                .map(|(&k, &v)| Ok((k.try_into()?, v.into())))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<WasmBoard> for Board {
    type Error = BeegoneError;

    fn try_from(value: WasmBoard) -> Result<Self, Self::Error> {
        Ok(Board::new_with_pieces(
            value
                .pieces
                .into_iter()
                .map(|(k, v)| Ok((k.try_into()?, v.try_into()?)))
                .collect::<Result<_, BeegoneError>>()?,
        ))
    }
}
