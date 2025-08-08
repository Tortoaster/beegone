pub use domain::{
    action::Action,
    board::Board,
    piece::{Bee, Color, Piece, Species},
    pos::Pos,
    state::State,
};

mod domain;
mod inbound;
mod outbound;
mod player;
