mod types;

use std::sync::Mutex;

use rival::Rival;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{player, player::Player, Board, Pos, State};
use crate::inbound::wasm::types::{ActionError, WasmAction, WasmBoard, WasmPiece, WasmState};

#[wasm_bindgen(js_name = "stateNew")]
pub fn state_new(players: u8) -> WasmState {
    let player = match players {
        1 => Player::Computer(Mutex::new(Rival::new())),
        2 => Player::Local,
        _ => panic!("unsupported number of players"),
    };

    player::initialize(player);

    State::default().into()
}

#[wasm_bindgen(js_name = "stateActionsFrom")]
pub fn state_actions_from(state: WasmState, pos: Pos) -> Vec<WasmAction> {
    State::from(state).actions_from(pos).map(Into::into).collect()
}

#[wasm_bindgen(js_name = "submitAction")]
pub async fn submit_action(action: WasmAction) -> Result<(), ActionError> {
    Ok(player::submit_action(action.try_into()?).await)
}

#[wasm_bindgen(js_name = "stateProgress")]
pub async fn state_progress(state: WasmState) -> Result<WasmState, JsValue> {
    let mut state: State = state.into();
    player::progress(&mut state).await?;
    Ok(state.into())
}

#[wasm_bindgen(js_name = "boardGet")]
pub fn board_get(board: WasmBoard, pos: Pos) -> Option<WasmPiece> {
    let board: Board = board.into();
    board.get(&pos).map(Into::into)
}

#[wasm_bindgen(js_name = "boardPositions")]
pub fn board_positions() -> Vec<Pos> {
    Board::positions().collect()
}
