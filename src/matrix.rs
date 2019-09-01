use ggez::graphics::*;
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
            outline_image: Image::new(ctx, "outline.png")?,
            block_images: [
                Image::new(ctx, "block_I.png")?,
                Image::new(ctx, "block_J.png")?,
                Image::new(ctx, "block_L.png")?,
                Image::new(ctx, "block_O.png")?,
                Image::new(ctx, "block_S.png")?,
                Image::new(ctx, "block_T.png")?,
                Image::new(ctx, "block_Z.png")?,
            ],
        })
    }

    fn clear(&mut self) {
        self.grid = [[0u8; WIDTH]; HEIGHT + VANISH];
    }

    fn debug_tower(&mut self) {
        let mut bricks: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (1, 0),
            (2, 0),
            (2, 1),
            (3, 0),
            (3, 1),
            (4, 0),
            (5, 0),
            (5, 1),
            (6, 0),
            (6, 1),
            (7, 0),
            (8, 0),
            (8, 1),
            (9, 0),
            (9, 1),
            (10, 0),
            (11, 0),
            (11, 1),
            (13, 2),
            (14, 2),
        ];

        for y in 0..14 {
            bricks.push((y, 3));
        }

        for y in 0..12 {
            for x in 4..10 {
                bricks.push((y, x));
            }
        }

        self.clear();
        for (y, x) in bricks {
            self.grid[y][x] = 1;
        }
    }
}

impl Matrix {
    fn draw(&self, _ctx: &mut Context, _param: DrawParam) -> GameResult {
        Ok(())
    }

    fn dimensions(&self, _: &mut Context) -> Option<graphics::Rect> {
        Some(self.block_images[0].dimensions())
    }

    // fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
    //     self.blend_mode = mode;
    // }

    // fn blend_mode(&self) -> Option<BlendMode> {
    //     self.blend_mode
    // }
}
