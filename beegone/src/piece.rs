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
    use wasm_bindgen::prelude::*;

    use crate::Bee;

    impl wasm_bindgen::convert::IntoWasmAbi for super::Piece {
        type Abi = <Piece as wasm_bindgen::convert::IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            match Piece::try_from(self) {
                Ok(value) => value.into_abi(),
                Err(_) => wasm_bindgen::throw_str("failed to serialize enum"),
            }
        }
    }

    impl wasm_bindgen::convert::FromWasmAbi for super::Piece {
        type Abi = <Piece as wasm_bindgen::convert::FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            match Piece::from_abi(js).try_into() {
                Ok(value) => value,
                Err(_) => wasm_bindgen::throw_str("failed to deserialize enum"),
            }
        }
    }

    impl wasm_bindgen::convert::OptionFromWasmAbi for super::Piece {
        #[inline]
        fn is_none(val: &Self::Abi) -> bool {
            Piece::is_none(val)
        }
    }

    impl wasm_bindgen::convert::OptionIntoWasmAbi for super::Piece {
        #[inline]
        fn none() -> Self::Abi {
            Piece::none()
        }
    }

    impl wasm_bindgen::describe::WasmDescribe for super::Piece {
        fn describe() {
            Piece::describe()
        }
    }

    #[wasm_bindgen]
    pub struct Piece {
        tag: PieceTag,
        content: PieceContent,
    }

    #[wasm_bindgen]
    #[derive(Copy, Clone)]
    pub enum PieceTag {
        Bee,
        Wall,
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "[Bee] | undefined")]
        type PieceContent;
    }

    impl TryFrom<Piece> for super::Piece {
        type Error = serde_wasm_bindgen::Error;

        fn try_from(value: Piece) -> Result<Self, Self::Error> {
            match value.tag {
                PieceTag::Bee => {
                    let (param0,) = serde_wasm_bindgen::from_value::<(Bee,)>(value.content.obj)?;
                    Ok(super::Piece::Bee(param0))
                }
                PieceTag::Wall => Ok(super::Piece::Wall),
            }
        }
    }

    impl TryFrom<super::Piece> for Piece {
        type Error = serde_wasm_bindgen::Error;

        fn try_from(value: super::Piece) -> Result<Self, Self::Error> {
            let tag = match value {
                super::Piece::Bee(_) => PieceTag::Bee,
                super::Piece::Wall => PieceTag::Wall,
            };

            let content = match value {
                super::Piece::Bee(param0) => PieceContent {
                    obj: serde_wasm_bindgen::to_value(&(param0,))?,
                },
                super::Piece::Wall => PieceContent {
                    obj: JsValue::undefined(),
                },
            };

            Ok(Piece { tag, content })
        }
    }
}
