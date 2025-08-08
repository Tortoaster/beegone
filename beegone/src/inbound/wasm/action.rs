use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    domain::action::{MoveAction, SpawnAction},
    inbound::wasm::piece::WasmPiece,
    Action, Pos,
};
use crate::inbound::wasm::error::InvalidBee;

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
    pub fn new_move(from: &Pos, to: &Pos) -> Self {
        Self {
            move_action: Some(WasmMoveAction {
                from: *from,
                to: *to,
            }),
            spawn_action: None,
        }
    }

    #[wasm_bindgen(js_name = "spawn")]
    pub fn new_spawn(on: &Pos, spawn: &WasmPiece) -> Self {
        Self {
            move_action: None,
            spawn_action: Some(WasmSpawnAction {
                on: *on,
                spawn: *spawn,
            }),
        }
    }
}

impl From<Action> for WasmAction {
    fn from(value: Action) -> Self {
        match value {
            Action::Move(move_action) => Self {
                move_action: Some(move_action.into()),
                spawn_action: None,
            },
            Action::Spawn(spawn_action) => Self {
                move_action: None,
                spawn_action: Some(spawn_action.into()),
            },
        }
    }
}

impl TryFrom<WasmAction> for Action {
    type Error = InvalidBee;

    fn try_from(value: WasmAction) -> Result<Self, Self::Error> {
        match (value.move_action, value.spawn_action) {
            (Some(move_action), None) => Ok(Action::Move(move_action.into())),
            (None, Some(spawn_action)) => Ok(Action::Spawn(spawn_action.try_into()?)),
            _ => unreachable!("invariant violated: an action must be either a move or a spawn"),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmMoveAction {
    #[wasm_bindgen(readonly)]
    pub from: Pos,
    #[wasm_bindgen(readonly)]
    pub to: Pos,
}

impl From<MoveAction> for WasmMoveAction {
    fn from(value: MoveAction) -> Self {
        Self {
            from: value.from(),
            to: value.to(),
        }
    }
}

impl From<WasmMoveAction> for MoveAction {
    fn from(value: WasmMoveAction) -> Self {
        MoveAction::new(value.from, value.to)
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmSpawnAction {
    #[wasm_bindgen(readonly)]
    pub on: Pos,
    #[wasm_bindgen(readonly)]
    pub spawn: WasmPiece,
}

impl From<SpawnAction> for WasmSpawnAction {
    fn from(value: SpawnAction) -> Self {
        Self {
            on: value.on(),
            spawn: value.spawn().into(),
        }
    }
}

impl TryFrom<WasmSpawnAction> for SpawnAction {
    type Error = InvalidBee;

    fn try_from(value: WasmSpawnAction) -> Result<Self, InvalidBee> {
        Ok(SpawnAction::new(value.on, value.spawn.try_into()?))
    }
}
