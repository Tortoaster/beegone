use std::collections::BTreeMap;
use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Action, Bee, Board, Color, Piece, Pos, State};
use crate::domain::action::{MoveAction, SpawnAction};

#[wasm_bindgen(js_name = "Piece")]
pub struct WasmPiece {
    /// If `None`, the piece is a wall.
    bee: Option<Bee>,
}

impl From<Piece> for WasmPiece {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Wall => WasmPiece { bee: None },
            Piece::Bee(bee) => WasmPiece {
                bee: Some(bee),
            },
        }
    }
}

impl From<WasmPiece> for Piece {
    fn from(value: WasmPiece) -> Self {
        match value.bee {
            None => Piece::Wall,
            Some(bee) => Piece::Bee(bee),
        }
    }
}

#[wasm_bindgen(js_name = "Board")]
pub struct WasmBoard {
    pieces: BTreeMap<Pos, WasmPiece>,
}

impl From<Board> for WasmBoard {
    fn from(value: Board) -> Self {
        Self {
            pieces: value
                .pieces()
                .iter()
                .map(|(&k, &v)| (k.into(), v.into()))
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
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

#[wasm_bindgen(js_name = "State")]
pub struct WasmState {
    board: WasmBoard,
    turn: Color,
}

impl From<State> for WasmState {
    fn from(value: State) -> Self {
        Self {
            board: value.board().clone().into(),
            turn: value.turn().into(),
        }
    }
}

impl From<WasmState> for State {
    fn from(value: WasmState) -> Self {
        State::new(value.board.into(), value.turn.into())
    }
}

#[wasm_bindgen(js_name = "Action")]
pub struct WasmAction {
    move_action: Option<MoveAction>,
    spawn_action: Option<SpawnAction>,
}

impl From<Action> for WasmAction {
    fn from(value: Action) -> Self {
        match value {
            Action::Move(move_action) => Self {
                move_action: Some(move_action),
                spawn_action: None,
            },
            Action::Spawn(spawn_action) => Self {
                move_action: None,
                spawn_action: Some(spawn_action),
            },
        }
    }
}

impl TryFrom<WasmAction> for Action {
    type Error = ActionError;

    fn try_from(value: WasmAction) -> Result<Self, Self::Error> {
        match (value.move_action, value.spawn_action) {
            (Some(move_action), None) => Ok(Action::Move(move_action)),
            (None, Some(spawn_action)) => Ok(Action::Spawn(spawn_action)),
            _ => Err(ActionError),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Error)]
#[error("Invalid action: must be either a move or a spawn action, not both.")]
pub struct ActionError;
