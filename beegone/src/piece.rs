use std::ops::Not;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::id::Id;

#[typeshare]
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    #[default]
    Light,
    Dark,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Dark => Color::Light,
            Color::Light => Color::Dark,
        }
    }
}

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Species {
    Drone,
    Worker,
    Nurse,
    Explorer,
    Builder,
    Guard,
    Queen,
}

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Bee {
    color: Color,
    species: Species,
}

impl Bee {
    pub const fn new(color: Color, species: Species) -> Self {
        Bee { color, species }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn species(&self) -> Species {
        self.species
    }
}

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Piece {
    id: Id,
    kind: PieceKind,
}

impl Piece {
    pub const DARK_DRONE: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Drone));
    pub const DARK_WORKER: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Worker));
    pub const DARK_NURSE: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Nurse));
    pub const DARK_EXPLORER: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Explorer));
    pub const DARK_BUILDER: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Builder));
    pub const DARK_GUARD: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Guard));
    pub const DARK_QUEEN: PieceKind = PieceKind::Bee(Bee::new(Color::Dark, Species::Queen));

    pub const LIGHT_DRONE: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Drone));
    pub const LIGHT_WORKER: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Worker));
    pub const LIGHT_NURSE: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Nurse));
    pub const LIGHT_EXPLORER: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Explorer));
    pub const LIGHT_BUILDER: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Builder));
    pub const LIGHT_GUARD: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Guard));
    pub const LIGHT_QUEEN: PieceKind = PieceKind::Bee(Bee::new(Color::Light, Species::Queen));

    pub fn new(kind: PieceKind) -> Self {
        Piece {
            id: Id::new(),
            kind,
        }
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
    }
}

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum PieceKind {
    Bee(Bee),
    Wall,
}

impl PieceKind {
    pub fn can_capture(&self, other: &PieceKind) -> bool {
        match self {
            PieceKind::Bee(bee) if bee.species != Species::Queen => match other {
                PieceKind::Bee(other) => bee.color != other.color && bee.species >= other.species,
                PieceKind::Wall => bee.species == Species::Builder,
            },
            _ => false,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            PieceKind::Bee(bee) => Some(bee.color),
            PieceKind::Wall => None,
        }
    }
}
