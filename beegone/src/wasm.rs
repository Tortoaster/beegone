use std::sync::Mutex;

use js_sys::Array;
use rival::Rival;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{player, player::Player, Action, Board, Pos, State};

#[wasm_bindgen(typescript_custom_section)]
const IMPORTS: &str = "import { Action, Board, Piece, Pos, State, WithId } from './types';";

#[wasm_bindgen(typescript_custom_section)]
const EXPORTS: &str = "export { Action, Board, Piece, Pos, State, WithId };";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Action")]
    pub type JsAction;

    #[wasm_bindgen(typescript_type = "Action[]")]
    pub type JsActionArray;

    #[wasm_bindgen(typescript_type = "Board")]
    pub type JsBoard;

    #[wasm_bindgen(typescript_type = "WithId<Piece>")]
    pub type JsPiece;

    #[wasm_bindgen(typescript_type = "Pos")]
    pub type JsPos;

    #[wasm_bindgen(typescript_type = "Pos[]")]
    pub type JsPosArray;

    #[wasm_bindgen(typescript_type = "State")]
    pub type JsState;
}

#[wasm_bindgen(js_name = "stateNew")]
pub fn state_new(players: u8) -> Result<JsState, JsValue> {
    let player = match players {
        1 => Player::Computer(Mutex::new(Rival::new())),
        2 => Player::Local,
        _ => panic!("unsupported number of players"),
    };

    player::initialize(player);

    let state = State::new();

    let js_state = JsState {
        obj: to_value(&state)?,
    };

    Ok(js_state)
}

#[wasm_bindgen(js_name = "stateActionsFrom")]
pub fn state_actions_from(state: JsState, pos: JsPos) -> Result<JsActionArray, JsValue> {
    let state: State = from_value(state.obj)?;
    let pos: Pos = from_value(pos.obj)?;

    let actions = state.actions_from(pos);

    let array = actions
        .map(|action| to_value(&action))
        .collect::<Result<Array, _>>()?
        .unchecked_into();

    Ok(array)
}

#[wasm_bindgen(js_name = "submitAction")]
pub async fn submit_action(action: JsAction) -> Result<(), JsValue> {
    let action: Action = from_value(action.obj)?;

    player::submit_action(action).await;

    Ok(())
}

#[wasm_bindgen(js_name = "stateProgress")]
pub async fn state_progress(state: JsState) -> Result<JsState, JsValue> {
    let mut state: State = from_value(state.obj)?;

    player::progress(&mut state).await?;

    let js_state = JsState {
        obj: to_value(&state)?,
    };

    Ok(js_state)
}

#[wasm_bindgen(js_name = "boardGet")]
pub fn board_get(board: JsBoard, pos: JsPos) -> Result<Option<JsPiece>, JsValue> {
    let board: Board = from_value(board.obj)?;
    let pos: Pos = from_value(pos.obj)?;

    let piece = board.get(&pos);

    let js_piece = match piece {
        None => None,
        Some(piece) => Some(JsPiece {
            obj: to_value(&piece)?,
        }),
    };

    Ok(js_piece)
}

#[wasm_bindgen(js_name = "boardPositions")]
pub fn board_positions() -> Result<JsPosArray, JsValue> {
    let positions = Board::positions();

    let array = positions
        .map(|pos| to_value(&pos))
        .collect::<Result<Array, _>>()?
        .unchecked_into();

    Ok(array)
}
