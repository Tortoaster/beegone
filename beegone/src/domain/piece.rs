use std::ops::Not;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    #[default]
    Light,
    Dark,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Light => Color::Dark,
            Color::Dark => Color::Light,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Species {
    Drone,
    Worker,
    Nurse,
    Builder,
    Explorer,
    Guard,
    Queen,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl Piece {
    pub const DARK_DRONE: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Drone));
    pub const DARK_WORKER: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Worker));
    pub const DARK_NURSE: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Nurse));
    pub const DARK_EXPLORER: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Explorer));
    pub const DARK_BUILDER: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Builder));
    pub const DARK_GUARD: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Guard));
    pub const DARK_QUEEN: Piece = Piece::Bee(Bee::new(Color::Dark, Species::Queen));

    pub const LIGHT_DRONE: Piece = Piece::Bee(Bee::new(Color::Light, Species::Drone));
    pub const LIGHT_WORKER: Piece = Piece::Bee(Bee::new(Color::Light, Species::Worker));
    pub const LIGHT_NURSE: Piece = Piece::Bee(Bee::new(Color::Light, Species::Nurse));
    pub const LIGHT_EXPLORER: Piece = Piece::Bee(Bee::new(Color::Light, Species::Explorer));
    pub const LIGHT_BUILDER: Piece = Piece::Bee(Bee::new(Color::Light, Species::Builder));
    pub const LIGHT_GUARD: Piece = Piece::Bee(Bee::new(Color::Light, Species::Guard));
    pub const LIGHT_QUEEN: Piece = Piece::Bee(Bee::new(Color::Light, Species::Queen));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Piece {
    Bee(Bee),
    Wall,
}

impl Piece {
    pub fn can_capture(&self, other: &Piece) -> bool {
        match self {
            Piece::Bee(bee) if bee.species != Species::Queen => match other {
                Piece::Bee(other) => bee.color != other.color && bee.species >= other.species,
                Piece::Wall => bee.species == Species::Builder,
            },
            _ => false,
        }
    }

    pub fn bee(&self) -> Option<&Bee> {
        match self {
            Piece::Bee(bee) => Some(&bee),
            Piece::Wall => None,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Piece::Bee(bee) => Some(bee.color),
            Piece::Wall => None,
        }
    }
}
