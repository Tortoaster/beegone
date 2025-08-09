use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    inbound::wasm::{
        action::WasmAction,
        board::WasmBoard,
        color::WasmColor,
        error::{InvalidBee, PerformError},
    },
    Action, Pos, State,
};

#[wasm_bindgen(js_name = "State")]
#[derive(Clone)]
pub struct WasmState {
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub board: WasmBoard,
    #[wasm_bindgen(readonly)]
    pub turn: WasmColor,
}

#[wasm_bindgen(js_class = "State")]
impl WasmState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        State::default().into()
    }

    #[wasm_bindgen(js_name = "actionsFrom")]
    pub fn actions_from(&self, pos: &Pos) -> Result<Vec<WasmAction>, InvalidBee> {
        Ok(State::try_from(self.clone())?
            .actions_from(*pos)
            .map(Into::into)
            .collect())
    }

    #[wasm_bindgen]
    pub fn perform(&mut self, action: &WasmAction) -> Result<WasmState, PerformError> {
        let mut state: State = self.clone().try_into()?;
        let action: Action = (*action).try_into()?;
        state
            .perform(action)
            .map_err(|_| PerformError::InvalidAction)?;
        Ok(state.into())
    }
}

impl From<State> for WasmState {
    fn from(value: State) -> Self {
        Self {
            board: value.board().clone().into(),
            turn: value.turn().into(),
        }
    }
}

impl TryFrom<WasmState> for State {
    type Error = InvalidBee;

    fn try_from(value: WasmState) -> Result<Self, InvalidBee> {
        Ok(State::new(value.board.try_into()?, value.turn.try_into()?))
    }
}
