use wasm_bindgen::prelude::wasm_bindgen;
use crate::domain::{piece::Piece, pos::Pos};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Action {
    Move(MoveAction),
    Spawn(SpawnAction),
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MoveAction {
    from: Pos,
    to: Pos,
}

impl MoveAction {
    pub fn new(from: Pos, to: Pos) -> Self {
        MoveAction { from, to }
    }

    pub fn from(&self) -> Pos {
        self.from
    }

    pub fn to(&self) -> Pos {
        self.to
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpawnAction {
    on: Pos,
    spawn: Piece,
}

impl SpawnAction {
    pub fn new(on: Pos, spawn: Piece) -> Self {
        SpawnAction { on, spawn }
    }

    pub fn on(&self) -> Pos {
        self.on
    }

    pub fn spawn(self) -> Piece {
        self.spawn
    }
}

#[derive(Default)]
pub enum Actions<'a> {
    #[default]
    None,
    Some {
        stage: usize,
        steps: Box<dyn Iterator<Item = Action> + 'a>,
        leaps: Box<dyn Iterator<Item = Action> + 'a>,
        captures: Box<dyn Iterator<Item = Action> + 'a>,
        specials: SpecialActions<'a>,
    },
}

impl<'a> Actions<'a> {
    pub fn new(
        steps: impl Iterator<Item = Action> + 'a,
        leaps: impl Iterator<Item = Action> + 'a,
        captures: impl Iterator<Item = Action> + 'a,
        specials: SpecialActions<'a>,
    ) -> Self {
        Actions::Some {
            stage: 0,
            steps: Box::new(steps),
            leaps: Box::new(leaps),
            captures: Box::new(captures),
            specials,
        }
    }
}

impl Iterator for Actions<'_> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Actions::None => None,
            Actions::Some {
                stage,
                steps,
                leaps,
                captures,
                specials,
            } => match stage {
                0 => match captures.next() {
                    None => {
                        *stage += 1;
                        self.next()
                    }
                    Some(action) => Some(action),
                },
                1 => match specials.next() {
                    None => {
                        *stage += 1;
                        self.next()
                    }
                    Some(action) => Some(action),
                },
                2 => match leaps.next() {
                    None => {
                        *stage += 1;
                        self.next()
                    }
                    Some(action) => Some(action),
                },
                _ => steps.next(),
            },
        }
    }
}

#[derive(Default)]
pub enum SpecialActions<'a> {
    #[default]
    None,
    Nurse(Box<dyn Iterator<Item = Action> + 'a>),
    Explorer(Box<dyn Iterator<Item = Action> + 'a>),
    Builder(Box<dyn Iterator<Item = Action> + 'a>),
    Queen(Box<dyn Iterator<Item = Action> + 'a>),
}

impl Iterator for SpecialActions<'_> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SpecialActions::None => None,
            SpecialActions::Nurse(iter) => iter.next(),
            SpecialActions::Explorer(iter) => iter.next(),
            SpecialActions::Builder(iter) => iter.next(),
            SpecialActions::Queen(iter) => iter.next(),
        }
    }
}
