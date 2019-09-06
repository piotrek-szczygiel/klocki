use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Image, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

pub const BLOCK_SIZE: usize = 32;
pub const BLOCKS_NUM: usize = 8;

pub struct Blocks {
    batch: SpriteBatch,
    rects: Vec<Rect>,
}

impl Blocks {
    pub fn new(tileset: Image) -> Self {
        if tileset.width() as usize != BLOCK_SIZE * BLOCKS_NUM
            || tileset.height() as usize != BLOCK_SIZE
        {
            log::error!(
                "Invalid tileset size {}:{}",
                tileset.width(),
                tileset.height()
            );
            std::process::exit(1);
        }

        let mut rects: Vec<Rect> = Vec::with_capacity(BLOCKS_NUM);

        for i in 0..BLOCKS_NUM {
            rects.push(Rect::new(
                i as f32 / BLOCKS_NUM as f32,
                0.0,
                1.0 / BLOCKS_NUM as f32,
                1.0,
            ));
        }

        Blocks {
            batch: SpriteBatch::new(tileset),
            rects,
        }
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }

    pub fn add(&mut self, block_id: usize, dest: Point2<f32>) {
        match block_id {
            1..=BLOCKS_NUM => {
                self.batch
                    .add(DrawParam::new().src(self.rects[block_id - 1]).dest(dest));
            }
            0 => (),
            _ => log::error!("Attempt to draw a non-existing block: {}", block_id),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.batch, DrawParam::new())?;

        Ok(())
    }
}
