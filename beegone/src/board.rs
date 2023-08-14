use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    piece::{Piece, PieceKind},
    pos::Pos,
};

#[typeshare]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Board {
    #[typeshare(skip)]
    pieces: BTreeMap<Pos, Piece>,
}

impl Board {
    const DIAMETER: usize = 7;
    pub const RADIUS: usize = Self::DIAMETER / 2;
    const CENTER: Pos = Board::D4;

    const A4: Pos = Pos { q: -3, r: 0 };
    const A5: Pos = Pos { q: -3, r: 1 };
    const A6: Pos = Pos { q: -3, r: 2 };
    const A7: Pos = Pos { q: -3, r: 3 };

    const B3: Pos = Pos { q: -2, r: -1 };
    const B4: Pos = Pos { q: -2, r: 0 };
    const B5: Pos = Pos { q: -2, r: 1 };
    const B6: Pos = Pos { q: -2, r: 2 };
    const B7: Pos = Pos { q: -2, r: 3 };

    const C2: Pos = Pos { q: -1, r: -2 };
    const C3: Pos = Pos { q: -1, r: -1 };
    const C4: Pos = Pos { q: -1, r: 0 };
    const C5: Pos = Pos { q: -1, r: 1 };
    const C6: Pos = Pos { q: -1, r: 2 };
    const C7: Pos = Pos { q: -1, r: 3 };

    const D1: Pos = Pos { q: 0, r: -3 };
    const D2: Pos = Pos { q: 0, r: -2 };
    const D3: Pos = Pos { q: 0, r: -1 };
    const D4: Pos = Pos { q: 0, r: 0 };
    const D5: Pos = Pos { q: 0, r: 1 };
    const D6: Pos = Pos { q: 0, r: 2 };
    const D7: Pos = Pos { q: 0, r: 3 };

    const E1: Pos = Pos { q: 1, r: -3 };
    const E2: Pos = Pos { q: 1, r: -2 };
    const E3: Pos = Pos { q: 1, r: -1 };
    const E4: Pos = Pos { q: 1, r: 0 };
    const E5: Pos = Pos { q: 1, r: 1 };
    const E6: Pos = Pos { q: 1, r: 2 };

    const F1: Pos = Pos { q: 2, r: -3 };
    const F2: Pos = Pos { q: 2, r: -2 };
    const F3: Pos = Pos { q: 2, r: -1 };
    const F4: Pos = Pos { q: 2, r: 0 };
    const F5: Pos = Pos { q: 2, r: 1 };

    const G1: Pos = Pos { q: 3, r: -3 };
    const G2: Pos = Pos { q: 3, r: -2 };
    const G3: Pos = Pos { q: 3, r: -1 };
    const G4: Pos = Pos { q: 3, r: 0 };

    #[rustfmt::skip]
    const POSITIONS: [Pos; 37] = [
                    Self::D1,
                Self::C2, Self::E1,
            Self::B3, Self::D2, Self::F1,
        Self::A4, Self::C3, Self::E2, Self::G1,
            Self::B4, Self::D3, Self::F2,
        Self::A5, Self::C4, Self::E3, Self::G2,
            Self::B5, Self::D4, Self::F3,
        Self::A6, Self::C5, Self::E4, Self::G3,
            Self::B6, Self::D5, Self::F4,
        Self::A7, Self::C6, Self::E5, Self::G4,
            Self::B7, Self::D6, Self::F5,
                Self::C7, Self::E6,
                    Self::D7,
    ];

    pub fn new() -> Self {
        let mut pieces = BTreeMap::new();

        pieces.insert(Self::D2, Piece::new(Piece::DARK_QUEEN));
        pieces.insert(Self::D1, Piece::new(Piece::DARK_DRONE));
        pieces.insert(Self::B3, Piece::new(Piece::DARK_NURSE));
        pieces.insert(Self::F1, Piece::new(Piece::DARK_NURSE));

        pieces.insert(Self::D6, Piece::new(Piece::LIGHT_QUEEN));
        pieces.insert(Self::D7, Piece::new(Piece::LIGHT_DRONE));
        pieces.insert(Self::B7, Piece::new(Piece::LIGHT_NURSE));
        pieces.insert(Self::F5, Piece::new(Piece::LIGHT_NURSE));

        pieces.insert(Self::A5, Piece::new(PieceKind::Wall));
        pieces.insert(Self::A6, Piece::new(PieceKind::Wall));
        pieces.insert(Self::B5, Piece::new(PieceKind::Wall));
        pieces.insert(Self::C4, Piece::new(PieceKind::Wall));
        pieces.insert(Self::C5, Piece::new(PieceKind::Wall));
        pieces.insert(Self::D4, Piece::new(PieceKind::Wall));
        pieces.insert(Self::E3, Piece::new(PieceKind::Wall));
        pieces.insert(Self::E4, Piece::new(PieceKind::Wall));
        pieces.insert(Self::F3, Piece::new(PieceKind::Wall));
        pieces.insert(Self::G2, Piece::new(PieceKind::Wall));
        pieces.insert(Self::G3, Piece::new(PieceKind::Wall));

        Board { pieces }
    }

    pub fn within_bounds(pos: Pos) -> bool {
        (pos - Self::CENTER).distance() <= Self::RADIUS as i32
    }

    pub fn get(&self, pos: &Pos) -> Option<Piece> {
        self.pieces.get(pos).copied()
    }

    pub fn set(&mut self, pos: Pos, piece: Option<Piece>) {
        match piece {
            None => {
                self.pieces.remove(&pos);
            }
            Some(piece) => {
                self.pieces
                    .entry(pos)
                    .and_modify(|p| *p = piece)
                    .or_insert(piece);
            }
        }
    }

    pub fn positions() -> impl Iterator<Item = Pos> {
        Self::POSITIONS.into_iter()
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
