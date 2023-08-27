use serde::{Deserialize, Serialize};

use crate::{Action, State};

mod local;

pub use local::submit_action;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Player {
    #[default]
    Local,
    Computer,
    Remote,
}

impl Player {
    /// Called as soon as this player's turn starts
    pub async fn get_action(&self, state: &State) -> Action {
        match self {
            Player::Local => local::retrieve_action().await,
            Player::Computer => state.actions().next().unwrap(),
            Player::Remote => todo!(),
        }
    }
}
