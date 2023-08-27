use rival::{EvaluateTwoPlayers, Moves, PlayClone, Value};

use crate::{Action, Bee, Color, Species, State};

impl Moves for State {
    type Move = Action;
    type Iter<'a> = Box<dyn Iterator<Item = Action> + 'a>;

    fn moves(&self) -> Self::Iter<'_> {
        Box::new(self.actions())
    }
}

impl PlayClone for State {
    fn play(&mut self, m: &Self::Move) {
        self.perform(*m).unwrap()
    }
}

impl EvaluateTwoPlayers for State {
    fn second_players_turn(&self) -> bool {
        self.turn() == Color::Dark
    }

    fn evaluate(&self) -> Value {
        self.board()
            .pieces()
            .values()
            .filter_map(|piece| piece.bee())
            .map(Bee::value)
            .sum()
    }
}

impl Species {
    fn value(&self) -> Value {
        match self {
            Species::Drone => 1,
            Species::Worker => 3,
            Species::Nurse => 20,
            Species::Explorer => 7,
            Species::Builder => 7,
            Species::Guard => 7,
            Species::Queen => 0,
        }
    }
}

impl Bee {
    pub fn value(&self) -> Value {
        match self.color() {
            Color::Light => self.species().value(),
            Color::Dark => -self.species().value(),
        }
    }
}
