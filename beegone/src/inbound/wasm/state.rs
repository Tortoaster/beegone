use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Color, Pos, State};
use crate::inbound::wasm::board::WasmBoard;
use crate::inbound::wasm::action::WasmAction;

#[wasm_bindgen(js_name = "State")]
#[derive(Clone)]
pub struct WasmState {
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub board: WasmBoard,
    #[wasm_bindgen(readonly)]
    pub turn: Color,
}

#[wasm_bindgen(js_class = "State")]
impl WasmState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        State::default().into()
    }

    #[wasm_bindgen(js_name = "actionsFrom")]
    pub fn actions_from(&self, pos: &Pos) -> Vec<WasmAction> {
        State::from(self.clone())
            .actions_from(*pos)
            .map(Into::into)
            .collect()
    }

    #[wasm_bindgen]
    pub fn perform(&mut self, action: &WasmAction) -> Result<WasmState, WasmInvalidAction> {
        let mut state = State::from(self.clone());
        state.perform((*action).into()).map_err(|_| WasmInvalidAction)?;
        Ok(state.into())
    }
}

impl From<State> for WasmState {
    fn from(value: State) -> Self {
        Self {
            board: value.board().clone().into(),
            turn: value.turn(),
        }
    }
}

impl From<WasmState> for State {
    fn from(value: WasmState) -> Self {
        State::new(value.board.into(), value.turn)
    }
}

#[wasm_bindgen(js_name = "InvalidAction")]
#[derive(Debug, Error)]
#[error("invalid action")]
pub struct WasmInvalidAction;
