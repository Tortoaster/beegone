use std::ops::Not;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    #[default]
    Light,
    Dark,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Dark => Color::Light,
            Color::Light => Color::Dark,
        }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Species {
    Drone,
    Worker,
    Nurse,
    Explorer,
    Builder,
    Guard,
    Queen,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Bee {
    pub color: Color,
    pub species: Species,
}

impl Bee {
    pub const fn new(color: Color, species: Species) -> Self {
        Bee { color, species }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "tag", content = "content"))]
pub enum Piece {
    Bee(Bee),
    Wall,
}

impl Piece {
    pub const DARK_DRONE: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Drone));
    pub const DARK_WORKER: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Worker));
    pub const DARK_NURSE: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Nurse));
    pub const DARK_EXPLORER: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Explorer));
    pub const DARK_BUILDER: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Builder));
    pub const DARK_GUARD: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Guard));
    pub const DARK_QUEEN: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Queen));

    pub const LIGHT_DRONE: Piece = Piece::Bee(Bee::new(Color::Light, Species::Drone));
    pub const LIGHT_WORKER: Piece = Piece::Bee(Bee::new(Color::Light, Species::Worker));
    pub const LIGHT_NURSE: Piece = Piece::Bee(Bee::new(Color::Light, Species::Nurse));
    pub const LIGHT_EXPLORER: Piece = Piece::Bee(Bee::new(Color::Light, Species::Explorer));
    pub const LIGHT_BUILDER: Piece = Piece::Bee(Bee::new(Color::Light, Species::Builder));
    pub const LIGHT_GUARD: Piece = Piece::Bee(Bee::new(Color::Light, Species::Guard));
    pub const LIGHT_QUEEN: Piece = Piece::Bee(Bee::new(Color::Light, Species::Queen));

    pub fn can_capture(&self, other: &Piece) -> bool {
        match self {
            Piece::Bee(bee) if bee.species != Species::Queen => match other {
                Piece::Bee(other) => bee.color != other.color && bee.species >= other.species,
                Piece::Wall => bee.species == Species::Builder,
            },
            _ => false,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Piece::Bee(bee) => Some(bee.color),
            Piece::Wall => None,
        }
    }
}

// Boilerplate to allow non-C-style enum `Piece`
#[cfg(feature = "wasm-bindgen")]
mod wasm {
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };

    use crate::Piece;

    #[wasm_bindgen(typescript_custom_section)]
    const PIECE: &str = r"export type Piece =
    | { tag: 'Bee'; content: [Color, Species] }
    | { tag: 'Wall'; content: undefined };";

    impl FromWasmAbi for Piece {
        type Abi = <JsValue as FromWasmAbi>::Abi;

        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            match serde_wasm_bindgen::from_value(JsValue::from_abi(js)) {
                Ok(value) => value,
                Err(_) => wasm_bindgen::throw_str("failed to deserialize enum"),
            }
        }
    }

    impl IntoWasmAbi for Piece {
        type Abi = <JsValue as IntoWasmAbi>::Abi;

        #[inline]
        fn into_abi(self) -> Self::Abi {
            match serde_wasm_bindgen::to_value(&self) {
                Ok(value) => value.into_abi(),
                Err(_) => wasm_bindgen::throw_str("failed to serialize enum"),
            }
        }
    }

    impl OptionFromWasmAbi for Piece {
        fn is_none(abi: &Self::Abi) -> bool {
            unsafe { JsValue::from_abi(*abi).is_null() }
        }
    }

    impl OptionIntoWasmAbi for Piece {
        #[inline]
        fn none() -> Self::Abi {
            JsValue::NULL.into_abi()
        }
    }

    impl WasmDescribe for Piece {
        fn describe() {
            JsValue::describe()
        }
    }
}
