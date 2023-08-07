use std::iter;

use crate::{piece::Piece, pos::Pos};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "tag", content = "content"))]
pub enum Action {
    Move(Pos, Pos),
    Spawn(Pos, Piece),
}

pub enum Actions<'a> {
    None,
    Some {
        stage: usize,
        steps: Box<dyn Iterator<Item = Action> + 'a>,
        leaps: Box<dyn Iterator<Item = Action> + 'a>,
        captures: Box<dyn Iterator<Item = Action> + 'a>,
        specials: SpecialActions,
    },
}

impl<'a> Actions<'a> {
    pub fn new(
        steps: impl Iterator<Item = Action> + 'a,
        leaps: impl Iterator<Item = Action> + 'a,
        captures: impl Iterator<Item = Action> + 'a,
        specials: SpecialActions,
    ) -> Self {
        Actions::Some {
            stage: 0,
            steps: Box::new(steps),
            leaps: Box::new(leaps),
            captures: Box::new(captures),
            specials,
        }
    }
}

impl Iterator for Actions<'_> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Actions::None => None,
            Actions::Some {
                stage,
                steps,
                leaps,
                captures,
                specials,
            } => match stage {
                0 => match captures.next() {
                    None => {
                        *stage += 1;
                        self.next()
                    }
                    Some(action) => Some(action),
                },
                1 => match specials.next() {
                    None => {
                        *stage += 1;
                        self.next()
                    }
                    Some(action) => Some(action),
                },
                2 => match leaps.next() {
                    None => {
                        *stage += 1;
                        self.next()
                    }
                    Some(action) => Some(action),
                },
                _ => match steps.next() {
                    None => None,
                    Some(action) => Some(action),
                },
            },
        }
    }
}

pub enum SpecialActions {
    None,
    Nurse(iter::Empty<Action>),
    Explorer(iter::Empty<Action>),
    Builder(iter::Empty<Action>),
    Queen(iter::Empty<Action>),
}

impl Iterator for SpecialActions {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SpecialActions::None => None,
            SpecialActions::Nurse(iter) => iter.next(),
            SpecialActions::Explorer(iter) => iter.next(),
            SpecialActions::Builder(iter) => iter.next(),
            SpecialActions::Queen(iter) => iter.next(),
        }
    }
}

// Boilerplate to allow non-C-style enum `Action`
#[cfg(feature = "wasm-bindgen")]
mod wasm {
    use wasm_bindgen::prelude::*;

    use crate::{Piece, Pos};

    impl wasm_bindgen::convert::IntoWasmAbi for super::Action {
        type Abi = <Action as wasm_bindgen::convert::IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            match Action::try_from(self) {
                Ok(value) => value.into_abi(),
                Err(_) => wasm_bindgen::throw_str("failed to serialize enum"),
            }
        }
    }

    impl wasm_bindgen::convert::FromWasmAbi for super::Action {
        type Abi = <Action as wasm_bindgen::convert::FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            match Action::from_abi(js).try_into() {
                Ok(value) => value,
                Err(_) => wasm_bindgen::throw_str("failed to deserialize enum"),
            }
        }
    }

    impl wasm_bindgen::convert::OptionFromWasmAbi for super::Action {
        #[inline]
        fn is_none(val: &Self::Abi) -> bool {
            Action::is_none(val)
        }
    }

    impl wasm_bindgen::convert::OptionIntoWasmAbi for super::Action {
        #[inline]
        fn none() -> Self::Abi {
            Action::none()
        }
    }

    impl wasm_bindgen::describe::WasmDescribe for super::Action {
        fn describe() {
            Action::describe()
        }
    }

    #[wasm_bindgen]
    pub struct Action {
        tag: ActionTag,
        content: ActionContent,
    }

    #[wasm_bindgen]
    #[derive(Copy, Clone)]
    pub enum ActionTag {
        Move,
        Spawn,
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "[Pos, Pos] | [Pos, Piece]")]
        type ActionContent;
    }

    impl TryFrom<Action> for super::Action {
        type Error = serde_wasm_bindgen::Error;

        fn try_from(value: Action) -> Result<Self, Self::Error> {
            match value.tag {
                ActionTag::Move => {
                    let (param0, param1) =
                        serde_wasm_bindgen::from_value::<(Pos, Pos)>(value.content.obj)?;
                    Ok(super::Action::Move(param0, param1))
                }
                ActionTag::Spawn => {
                    let (param0, param1) =
                        serde_wasm_bindgen::from_value::<(Pos, Piece)>(value.content.obj)?;
                    Ok(super::Action::Spawn(param0, param1))
                }
            }
        }
    }

    impl TryFrom<super::Action> for Action {
        type Error = serde_wasm_bindgen::Error;

        fn try_from(value: super::Action) -> Result<Self, Self::Error> {
            let tag = match value {
                super::Action::Move(_, _) => ActionTag::Move,
                super::Action::Spawn(_, _) => ActionTag::Spawn,
            };

            let content = match value {
                super::Action::Move(param0, param1) => ActionContent {
                    obj: serde_wasm_bindgen::to_value(&(param0, param1))?,
                },
                super::Action::Spawn(param0, param1) => ActionContent {
                    obj: serde_wasm_bindgen::to_value(&(param0, param1))?,
                },
            };

            Ok(Action { tag, content })
        }
    }
}
