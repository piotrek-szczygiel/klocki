use ggez::graphics::Image;
use ggez::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const VANISH: usize = 20;

type Grid = [[u8; WIDTH]; HEIGHT + VANISH];

pub struct Matrix {
    grid: Grid,
    outline_image: Image,
    block_images: [Image; 7],
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Matrix {
            grid: [[0u8; WIDTH]; HEIGHT + VANISH],
            outline_image: Image::new(ctx, "/outline.png")?,
            block_images: [
                Image::new(ctx, "/block_I.png")?,
                Image::new(ctx, "/block_J.png")?,
                Image::new(ctx, "/block_L.png")?,
                Image::new(ctx, "/block_O.png")?,
                Image::new(ctx, "/block_S.png")?,
                Image::new(ctx, "/block_T.png")?,
                Image::new(ctx, "/block_Z.png")?,
            ],
        })
    }
}
