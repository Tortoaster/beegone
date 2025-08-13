use wasm_bindgen::prelude::wasm_bindgen;

use crate::{inbound::wasm::error::BeegoneError, Pos};

#[wasm_bindgen(js_name = "Pos")]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum WasmPos {
    A4 = "A4",
    A5 = "A5",
    A6 = "A6",
    A7 = "A7",
    B3 = "B3",
    B4 = "B4",
    B5 = "B5",
    B6 = "B6",
    B7 = "B7",
    C2 = "C2",
    C3 = "C3",
    C4 = "C4",
    C5 = "C5",
    C6 = "C6",
    C7 = "C7",
    D1 = "D1",
    D2 = "D2",
    D3 = "D3",
    D4 = "D4",
    D5 = "D5",
    D6 = "D6",
    D7 = "D7",
    E1 = "E1",
    E2 = "E2",
    E3 = "E3",
    E4 = "E4",
    E5 = "E5",
    E6 = "E6",
    F1 = "F1",
    F2 = "F2",
    F3 = "F3",
    F4 = "F4",
    F5 = "F5",
    G1 = "G1",
    G2 = "G2",
    G3 = "G3",
    G4 = "G4",
}

impl TryFrom<Pos> for WasmPos {
    type Error = BeegoneError;

    fn try_from(value: Pos) -> Result<Self, Self::Error> {
        match value {
            Pos { q: -3, r: 0 } => Ok(WasmPos::A4),
            Pos { q: -3, r: 1 } => Ok(WasmPos::A5),
            Pos { q: -3, r: 2 } => Ok(WasmPos::A6),
            Pos { q: -3, r: 3 } => Ok(WasmPos::A7),
            Pos { q: -2, r: -1 } => Ok(WasmPos::B3),
            Pos { q: -2, r: 0 } => Ok(WasmPos::B4),
            Pos { q: -2, r: 1 } => Ok(WasmPos::B5),
            Pos { q: -2, r: 2 } => Ok(WasmPos::B6),
            Pos { q: -2, r: 3 } => Ok(WasmPos::B7),
            Pos { q: -1, r: -2 } => Ok(WasmPos::C2),
            Pos { q: -1, r: -1 } => Ok(WasmPos::C3),
            Pos { q: -1, r: 0 } => Ok(WasmPos::C4),
            Pos { q: -1, r: 1 } => Ok(WasmPos::C5),
            Pos { q: -1, r: 2 } => Ok(WasmPos::C6),
            Pos { q: -1, r: 3 } => Ok(WasmPos::C7),
            Pos { q: 0, r: -3 } => Ok(WasmPos::D1),
            Pos { q: 0, r: -2 } => Ok(WasmPos::D2),
            Pos { q: 0, r: -1 } => Ok(WasmPos::D3),
            Pos { q: 0, r: 0 } => Ok(WasmPos::D4),
            Pos { q: 0, r: 1 } => Ok(WasmPos::D5),
            Pos { q: 0, r: 2 } => Ok(WasmPos::D6),
            Pos { q: 0, r: 3 } => Ok(WasmPos::D7),
            Pos { q: 1, r: -3 } => Ok(WasmPos::E1),
            Pos { q: 1, r: -2 } => Ok(WasmPos::E2),
            Pos { q: 1, r: -1 } => Ok(WasmPos::E3),
            Pos { q: 1, r: 0 } => Ok(WasmPos::E4),
            Pos { q: 1, r: 1 } => Ok(WasmPos::E5),
            Pos { q: 1, r: 2 } => Ok(WasmPos::E6),
            Pos { q: 2, r: -3 } => Ok(WasmPos::F1),
            Pos { q: 2, r: -2 } => Ok(WasmPos::F2),
            Pos { q: 2, r: -1 } => Ok(WasmPos::F3),
            Pos { q: 2, r: 0 } => Ok(WasmPos::F4),
            Pos { q: 2, r: 1 } => Ok(WasmPos::F5),
            Pos { q: 3, r: -3 } => Ok(WasmPos::G1),
            Pos { q: 3, r: -2 } => Ok(WasmPos::G2),
            Pos { q: 3, r: -1 } => Ok(WasmPos::G3),
            Pos { q: 3, r: 0 } => Ok(WasmPos::G4),
            _ => Err(BeegoneError::InvalidPos),
        }
    }
}

impl TryFrom<WasmPos> for Pos {
    type Error = BeegoneError;

    fn try_from(value: WasmPos) -> Result<Self, Self::Error> {
        match value {
            WasmPos::A4 => Ok(Pos { q: -3, r: 0 }),
            WasmPos::A5 => Ok(Pos { q: -3, r: 1 }),
            WasmPos::A6 => Ok(Pos { q: -3, r: 2 }),
            WasmPos::A7 => Ok(Pos { q: -3, r: 3 }),
            WasmPos::B3 => Ok(Pos { q: -2, r: -1 }),
            WasmPos::B4 => Ok(Pos { q: -2, r: 0 }),
            WasmPos::B5 => Ok(Pos { q: -2, r: 1 }),
            WasmPos::B6 => Ok(Pos { q: -2, r: 2 }),
            WasmPos::B7 => Ok(Pos { q: -2, r: 3 }),
            WasmPos::C2 => Ok(Pos { q: -1, r: -2 }),
            WasmPos::C3 => Ok(Pos { q: -1, r: -1 }),
            WasmPos::C4 => Ok(Pos { q: -1, r: 0 }),
            WasmPos::C5 => Ok(Pos { q: -1, r: 1 }),
            WasmPos::C6 => Ok(Pos { q: -1, r: 2 }),
            WasmPos::C7 => Ok(Pos { q: -1, r: 3 }),
            WasmPos::D1 => Ok(Pos { q: 0, r: -3 }),
            WasmPos::D2 => Ok(Pos { q: 0, r: -2 }),
            WasmPos::D3 => Ok(Pos { q: 0, r: -1 }),
            WasmPos::D4 => Ok(Pos { q: 0, r: 0 }),
            WasmPos::D5 => Ok(Pos { q: 0, r: 1 }),
            WasmPos::D6 => Ok(Pos { q: 0, r: 2 }),
            WasmPos::D7 => Ok(Pos { q: 0, r: 3 }),
            WasmPos::E1 => Ok(Pos { q: 1, r: -3 }),
            WasmPos::E2 => Ok(Pos { q: 1, r: -2 }),
            WasmPos::E3 => Ok(Pos { q: 1, r: -1 }),
            WasmPos::E4 => Ok(Pos { q: 1, r: 0 }),
            WasmPos::E5 => Ok(Pos { q: 1, r: 1 }),
            WasmPos::E6 => Ok(Pos { q: 1, r: 2 }),
            WasmPos::F1 => Ok(Pos { q: 2, r: -3 }),
            WasmPos::F2 => Ok(Pos { q: 2, r: -2 }),
            WasmPos::F3 => Ok(Pos { q: 2, r: -1 }),
            WasmPos::F4 => Ok(Pos { q: 2, r: 0 }),
            WasmPos::F5 => Ok(Pos { q: 2, r: 1 }),
            WasmPos::G1 => Ok(Pos { q: 3, r: -3 }),
            WasmPos::G2 => Ok(Pos { q: 3, r: -2 }),
            WasmPos::G3 => Ok(Pos { q: 3, r: -1 }),
            WasmPos::G4 => Ok(Pos { q: 3, r: 0 }),
            WasmPos::__Invalid => Err(BeegoneError::InvalidPos),
        }
    }
}

#[wasm_bindgen]
pub fn x(pos: WasmPos) -> Result<f32, BeegoneError> {
    Ok(Pos::try_from(pos)?.x())
}

#[wasm_bindgen]
pub fn y(pos: WasmPos) -> Result<f32, BeegoneError> {
    Ok(Pos::try_from(pos)?.y())
}

#[wasm_bindgen]
pub fn distance(from: WasmPos, to: WasmPos) -> Result<i32, BeegoneError> {
    Ok((Pos::try_from(from)? - Pos::try_from(to)?).distance())
}
