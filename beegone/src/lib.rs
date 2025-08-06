pub use domain::action::Action;
pub use domain::board::Board;
pub use domain::piece::{Bee, Color, Piece, Species};
pub use domain::pos::Pos;
pub use domain::state::State;

mod player;
mod domain;
mod outbound;
mod inbound;
