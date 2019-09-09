use crate::piece::Piece;

use ggez::{Context, graphics, GameResult};

pub struct Holder {
    piece: Optional<Piece>
}

impl Holder {
    pub fn new() -> Holder {
        Holder {
            piece: None
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {

    }
}
