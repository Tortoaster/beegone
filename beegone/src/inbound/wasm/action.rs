use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    domain::action::{MoveAction, SpawnAction},
    inbound::wasm::{error::BeegoneError, piece::WasmPiece, pos::WasmPos},
    Action,
};

#[wasm_bindgen(js_name = "Action")]
#[derive(Copy, Clone)]
pub struct WasmAction {
    #[wasm_bindgen(js_name = "move", readonly)]
    pub move_action: Option<WasmMoveAction>,
    #[wasm_bindgen(js_name = "spawn", readonly)]
    pub spawn_action: Option<WasmSpawnAction>,
}

#[wasm_bindgen(js_class = "Action")]
impl WasmAction {
    #[wasm_bindgen(js_name = "move")]
    pub fn new_move(from: WasmPos, to: WasmPos) -> Result<Self, BeegoneError> {
        Ok(Self {
            move_action: Some(WasmMoveAction { from, to }),
            spawn_action: None,
        })
    }

    #[wasm_bindgen(js_name = "spawn")]
    pub fn new_spawn(on: WasmPos, spawn: &WasmPiece) -> Self {
        Self {
            move_action: None,
            spawn_action: Some(WasmSpawnAction { on, spawn: *spawn }),
        }
    }
}

impl TryFrom<Action> for WasmAction {
    type Error = BeegoneError;

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        match value {
            Action::Move(move_action) => Ok(Self {
                move_action: Some(move_action.try_into()?),
                spawn_action: None,
            }),
            Action::Spawn(spawn_action) => Ok(Self {
                move_action: None,
                spawn_action: Some(spawn_action.try_into()?),
            }),
        }
    }
}

impl TryFrom<WasmAction> for Action {
    type Error = BeegoneError;

    fn try_from(value: WasmAction) -> Result<Self, Self::Error> {
        match (value.move_action, value.spawn_action) {
            (Some(move_action), None) => Ok(Action::Move(move_action.try_into()?)),
            (None, Some(spawn_action)) => Ok(Action::Spawn(spawn_action.try_into()?)),
            _ => unreachable!("invariant violated: an action must be either a move or a spawn"),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmMoveAction {
    #[wasm_bindgen(readonly)]
    pub from: WasmPos,
    #[wasm_bindgen(readonly)]
    pub to: WasmPos,
}

impl TryFrom<MoveAction> for WasmMoveAction {
    type Error = BeegoneError;

    fn try_from(value: MoveAction) -> Result<Self, Self::Error> {
        Ok(Self {
            from: value.from().try_into()?,
            to: value.to().try_into()?,
        })
    }
}

impl TryFrom<WasmMoveAction> for MoveAction {
    type Error = BeegoneError;

    fn try_from(value: WasmMoveAction) -> Result<Self, Self::Error> {
        Ok(MoveAction::new(
            value.from.try_into()?,
            value.to.try_into()?,
        ))
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmSpawnAction {
    #[wasm_bindgen(readonly)]
    pub on: WasmPos,
    #[wasm_bindgen(readonly)]
    pub spawn: WasmPiece,
}

impl TryFrom<SpawnAction> for WasmSpawnAction {
    type Error = BeegoneError;

    fn try_from(value: SpawnAction) -> Result<Self, Self::Error> {
        Ok(Self {
            on: value.on().try_into()?,
            spawn: value.spawn().into(),
        })
    }
}

impl TryFrom<WasmSpawnAction> for SpawnAction {
    type Error = BeegoneError;

    fn try_from(value: WasmSpawnAction) -> Result<Self, BeegoneError> {
        Ok(SpawnAction::new(
            value.on.try_into()?,
            value.spawn.try_into()?,
        ))
    }
}
