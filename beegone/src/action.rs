use std::iter;

use crate::{piece::Piece, pos::Pos};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "tag", content = "content"))]
pub enum Action {
    Move(Pos, Pos),
    Spawn(Pos, Piece),
}

pub enum Actions<'a> {
    None,
    Some {
        stage: usize,
        steps: Box<dyn Iterator<Item = Action> + 'a>,
        leaps: Box<dyn Iterator<Item = Action> + 'a>,
        captures: Box<dyn Iterator<Item = Action> + 'a>,
        specials: SpecialActions,
    },
}

impl<'a> Actions<'a> {
    pub fn new(
        steps: impl Iterator<Item = Action> + 'a,
        leaps: impl Iterator<Item = Action> + 'a,
        captures: impl Iterator<Item = Action> + 'a,
        specials: SpecialActions,
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
                _ => match steps.next() {
                    None => None,
                    Some(action) => Some(action),
                },
            },
        }
    }
}

pub enum SpecialActions {
    None,
    Nurse(iter::Empty<Action>),
    Explorer(iter::Empty<Action>),
    Builder(iter::Empty<Action>),
    Queen(iter::Empty<Action>),
}

impl Iterator for SpecialActions {
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
