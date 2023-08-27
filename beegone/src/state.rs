use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    action::{Action, Actions, MoveAction, SpawnAction, SpecialActions},
    board::Board,
    id::IdExt,
    iter::IteratorExt,
    piece::{Color, Piece},
    pos::{Pos, Shift},
    Bee, Species,
};

#[typeshare]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct State {
    board: Board,
    turn: Color,
}

impl State {
    pub fn new() -> Self {
        State {
            board: Board::default(),
            turn: Color::default(),
        }
    }

    pub fn actions<'a>(&'a self) -> impl Iterator<Item = Action> + 'a {
        Board::positions().flat_map(|pos| self.actions_from(pos))
    }

    pub fn actions_from(&self, from: Pos) -> Actions {
        let piece = match self.board.get(&from) {
            None => return Actions::None,
            Some(piece) => piece,
        };

        let bee = match *piece {
            Piece::Bee(bee) => bee,
            Piece::Wall => return Actions::None,
        };

        if bee.color() != self.turn {
            return Actions::None;
        }

        // All bees can move to empty adjacent tiles
        let steps = from
            .adjacent()
            .within_bounds()
            .filter(move |pos| self.board.get(pos).is_none())
            .map(move |adj| Action::Move(MoveAction::new(from, adj)));

        // All bees can jump over adjacent bees of the same color, as long as the tile
        // they land on is empty
        let leaps = Shift::directions()
            .filter(move |shift| {
                self.board
                    .get(&(from + *shift))
                    .and_then(|piece| piece.color())
                    .map(|color| color == bee.color())
                    .unwrap_or_default()
            })
            .map(move |shift| from + shift * 2)
            .within_bounds()
            .filter(move |pos| self.board.get(pos).is_none())
            .map(move |pos| Action::Move(MoveAction::new(from, pos)));

        // Most bees can capture weaker adjacent pieces of the opposite color, and
        // builders can capture walls
        let captures = from
            .adjacent()
            .filter(move |adj| {
                self.board
                    .get(adj)
                    .as_ref()
                    .map(|other| piece.can_capture(&other))
                    .unwrap_or_default()
            })
            .map(move |adj| Action::Move(MoveAction::new(from, adj)));

        let specials = match bee.species() {
            // Drones, workers and guards have no special power
            Species::Drone | Species::Worker | Species::Guard => SpecialActions::None,
            // Nurses can promote adjacent workers to explorers, builders, or guards
            Species::Nurse => SpecialActions::Nurse(Box::new(
                from.adjacent()
                    .filter(move |adj| {
                        self.board
                            .get(adj)
                            .map(|piece| match *piece {
                                Piece::Bee(adj) => {
                                    adj.color() == bee.color() && adj.species() == Species::Worker
                                }
                                Piece::Wall => false,
                            })
                            .unwrap_or_default()
                    })
                    .flat_map(move |adj| {
                        vec![Species::Explorer, Species::Builder, Species::Guard]
                            .into_iter()
                            .map(move |species| {
                                Action::Spawn(SpawnAction::new(
                                    adj,
                                    Piece::Bee(Bee::new(bee.color(), species)).with_id(),
                                ))
                            })
                    }),
            )),
            // Explorers can move any number of tiles in a straight line, as long as
            // they are all empty, and optionally capture on the last tile TODO
            Species::Explorer => {
                SpecialActions::Explorer(Box::new(Shift::directions().flat_map(move |dir| {
                    (1..)
                        .map(move |distance| from + dir * distance)
                        .scan(false, move |encountered_enemy, pos| {
                            match (*encountered_enemy, self.board.get(&pos)) {
                                (true, _) => None,
                                (false, None) => Some(pos),
                                (false, Some(other)) => {
                                    if piece.can_capture(&other) {
                                        *encountered_enemy = true;
                                        Some(pos)
                                    } else {
                                        None
                                    }
                                }
                            }
                        })
                        .skip(1)
                        .take_while(|pos| Board::within_bounds(*pos))
                        .map(move |to| Action::Move(MoveAction::new(from, to)))
                })))
            }
            // Builders can spawn walls on empty adjacent tiles
            Species::Builder => SpecialActions::Builder(Box::new(
                from.adjacent()
                    .within_bounds()
                    .filter(move |adj| self.board.get(adj).is_none())
                    .map(|adj| Action::Spawn(SpawnAction::new(adj, Piece::Wall.with_id()))),
            )),
            // Queens can spawn drones on empty adjacent tiles, and if a drone is
            // already adjacent to the queen, she can spawn workers as well
            Species::Queen => {
                let drones = from
                    .adjacent()
                    .within_bounds()
                    .filter(move |adj| self.board.get(adj).is_none())
                    .map(move |adj| {
                        Action::Spawn(SpawnAction::new(
                            adj,
                            Piece::Bee(Bee::new(bee.color(), Species::Drone)).with_id(),
                        ))
                    });

                let fertilized = from.adjacent().any(|adj| {
                    self.board
                        .get(&adj)
                        .map(|piece| {
                            piece.bee().copied() == Some(Bee::new(bee.color(), Species::Drone))
                        })
                        .unwrap_or_default()
                });

                let workers = if fertilized {
                    Box::new(
                        from.adjacent()
                            .within_bounds()
                            .filter(move |adj| self.board.get(adj).is_none())
                            .map(move |adj| {
                                Action::Spawn(SpawnAction::new(
                                    adj,
                                    Piece::Bee(Bee::new(bee.color(), Species::Worker)).with_id(),
                                ))
                            }),
                    )
                } else {
                    Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Action>>
                };

                SpecialActions::Queen(Box::new(drones.chain(workers)))
            }
        };

        Actions::new(steps, leaps, captures, specials)
    }

    pub fn perform(&mut self, action: Action) -> Result<(), &'static str> {
        match action {
            Action::Move(move_action) => {
                self.actions_from(move_action.from())
                    .find(|a| *a == action)
                    .ok_or("illegal move")?;

                let piece = self.board.get(&move_action.from()).unwrap();
                self.board.set(move_action.from(), None);
                self.board.set(move_action.to(), Some(piece));
            }
            Action::Spawn(spawn_action) => {
                // TODO: Legal?

                self.board
                    .set(spawn_action.on(), Some(spawn_action.spawn()));
            }
        }

        self.turn = !self.turn;

        Ok(())
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
