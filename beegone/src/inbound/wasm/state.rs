use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    inbound::wasm::{
        action::WasmAction, board::WasmBoard, color::WasmColor, error::BeegoneError, pos::WasmPos,
    },
    Action, State,
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
    pub fn new() -> Result<Self, BeegoneError> {
        State::default().try_into()
    }

    #[wasm_bindgen(js_name = "actionsFrom")]
    pub fn actions_from(&self, pos: WasmPos) -> Result<Vec<WasmAction>, BeegoneError> {
        Ok(State::try_from(self.clone())?
            .actions_from(pos.try_into()?)
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?)
    }

    #[wasm_bindgen]
    pub fn perform(&mut self, action: &WasmAction) -> Result<WasmState, BeegoneError> {
        let mut state: State = self.clone().try_into()?;
        let action: Action = (*action).try_into()?;
        state
            .perform(action)
            .map_err(|_| BeegoneError::InvalidAction)?;
        Ok(state.try_into()?)
    }
}

impl TryFrom<State> for WasmState {
    type Error = BeegoneError;

    fn try_from(value: State) -> Result<Self, BeegoneError> {
        let (board, turn) = value.split();
        Ok(Self {
            board: board.try_into()?,
            turn: turn.into(),
        })
    }
}

impl TryFrom<WasmState> for State {
    type Error = BeegoneError;

    fn try_from(value: WasmState) -> Result<Self, BeegoneError> {
        Ok(State::new(value.board.try_into()?, value.turn.try_into()?))
    }
}
