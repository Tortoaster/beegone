use std::{collections::BTreeMap, sync::OnceLock};

use rival::{MaxN, Rival};

use crate::{Action, Color, State};

mod computer;
mod local;

pub use local::submit_action;

static PLAYERS: OnceLock<BTreeMap<Color, Player>> = OnceLock::new();

pub fn initialize(opponent: Player) {
    let local_color = Color::Light;

    let mut players = BTreeMap::new();

    players.insert(local_color, Player::Local);
    players.insert(!local_color, opponent);

    PLAYERS.set(players).unwrap()
}

pub fn players() -> &'static BTreeMap<Color, Player> {
    PLAYERS.get().unwrap()
}

pub async fn progress(state: &mut State) -> Result<(), &'static str> {
    let action = players()[&state.turn()].get_action(state).await;
    state.perform(action)
}

#[derive(Clone, Debug, Default)]
pub enum Player {
    #[default]
    Local,
    Computer(Rival<State, MaxN, 2>),
    Remote,
}

impl Player {
    /// Called as soon as this player's turn starts
    pub async fn get_action(&self, state: &mut State) -> Action {
        match self {
            Player::Local => local::retrieve_action().await,
            Player::Computer(rival) => rival.get_best(state, 3).unwrap(),
            Player::Remote => todo!(),
        }
    }
}
