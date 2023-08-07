#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

use crate::{
    action::{Action, Actions, SpecialActions},
    board::Board,
    error::Result,
    iter::IteratorExt,
    piece::{Color, Piece},
    pos::{Pos, Shift},
};

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<Action>")]
    pub type ActionArray;
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct State {
    // TODO: cfg
    #[wasm_bindgen(getter_with_clone)]
    pub board: Board,
    pub turn: Color,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl State {
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        State::default()
    }

    #[cfg(feature = "wasm-bindgen")]
    pub fn all_actions_from(&self, from: Pos) -> ActionArray {
        let array: js_sys::Array = self
            .actions_from(from)
            .map(|action| serde_wasm_bindgen::to_value(&action).unwrap())
            .collect();
        array.unchecked_into::<ActionArray>()
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn perform(&mut self, _action: Action) -> Result<()> {
        todo!()
    }
}

impl State {
    pub fn actions<'a>(&'a self) -> impl Iterator<Item = Action> + 'a {
        Board::positions().flat_map(|pos| self.actions_from(pos))
    }

    pub fn actions_from(&self, from: Pos) -> Actions {
        let piece = match self.board.get(&from) {
            None => return Actions::None,
            Some(piece) => piece,
        };

        let bee = match piece {
            Piece::Bee(bee) => bee,
            Piece::Wall => return Actions::None,
        };

        if bee.color != self.turn {
            return Actions::None;
        }

        // All bees can move to empty adjacent tiles
        let steps = from
            .adjacent()
            .on_board()
            .filter(move |pos| self.board.get(pos).is_none())
            .map(move |adj| Action::Move(from, adj));

        // All bees can jump over adjacent bees of the same color, as long as the tile
        // they land on is empty
        let leaps = Shift::directions()
            .filter(move |shift| {
                self.board
                    .get(&(from + *shift))
                    .and_then(Piece::color)
                    .map(|color| color == bee.color)
                    .unwrap_or_default()
            })
            .map(move |shift| from + shift * 2)
            .on_board()
            .filter(move |pos| self.board.get(pos).is_none())
            .map(move |pos| Action::Move(from, pos));

        // Most bees can capture weaker adjacent pieces of the opposite color, and
        // builders can capture walls
        let captures = from
            .adjacent()
            .filter(move |adj| {
                self.board
                    .get(adj)
                    .map(|p| piece.can_capture(p))
                    .unwrap_or_default()
            })
            .map(move |adj| Action::Move(from, adj));

        let specials = SpecialActions::None;

        // let specials = match bee.species {
        //     // Drones, workers and guards have no special power
        //     Species::Drone | Species::Worker | Species::Guard =>
        // SpecialActions::None,     // Nurses can promote adjacent workers to
        // explorers, builders, or guards     Species::Nurse =>
        // SpecialActions::Nurse(         from.adjacent()
        //             .filter(move |adj| {
        //                 self.pieces
        //                     .get(adj)
        //                     .map(|piece| match piece {
        //                         Piece::Bee(bee) => color == bee.color && *s ==
        // Species::Worker,                         Piece::Wall => false,
        //                     })
        //                     .unwrap_or_default()
        //             })
        //             .flat_map(move |adj| {
        //                 vec![Species::Explorer, Species::Builder, Species::Guard]
        //                     .into_iter()
        //                     .map(move |species| Action::Spawn(adj, Piece::Bee(*color,
        // species)))             }),
        //     ),
        //     // Explorers can move any number of tiles in a straight line, as long as
        //     // they are all empty, and optionally capture on the last tile TODO
        //     Species::Explorer => SpecialActions::Explorer(iter::empty()),
        //     // Builders can spawn walls on empty adjacent tiles
        //     Species::Builder => Box::new(
        //         from.adjacent()
        //             .filter(move |adj| self.in_bounds(*adj))
        //             .filter(move |adj| self.pieces.get(adj).is_none())
        //             .map(|adj| Action::Spawn(adj, Piece::Wall)),
        //     ),
        //     // Queens can spawn drones on empty adjacent tiles, and if a drone is
        //     // already adjacent to the queen, she can spawn workers as well
        //     Species::Queen => {
        //         let drones = from
        //             .adjacent()
        //             .filter(move |adj2| self.in_bounds(*adj2))
        //             .filter(move |adj2| self.pieces.get(adj2).is_none())
        //             .map(move |adj| {
        //                 Action::Spawn(adj, Piece::Bee(Bee::new(*color,
        // Species::Drone)))             });
        //
        //         let fertilized = from.adjacent().any(|adj| {
        //             self.pieces.get(&adj) == Some(&Piece::Bee(Bee::new(color,
        // Species::Drone)))         });
        //
        //         let workers = if fertilized {
        //             Box::new(
        //                 from.adjacent()
        //                     .filter(move |adj2| self.in_bounds(*adj2))
        //                     .filter(move |adj2| self.pieces.get(adj2).is_none())
        //                     .map(move |adj| {
        //                         Action::Spawn(adj, Piece::Bee(Bee::new(*color,
        // Species::Worker)))                     }),
        //             )
        //         } else {
        //             Box::new(iter::empty()) as Box<dyn Iterator<Item = Action>>
        //         };
        //
        //         Box::new(drones.chain(workers))
        //     }
        // };

        Actions::new(steps, leaps, captures, specials)
    }
}
