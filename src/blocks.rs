use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image, Rect},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

pub const BLOCK_SIZE: i32 = 40;
pub const BLOCKS_NUM: usize = 8;

pub struct Blocks {
    batch: SpriteBatch,
    rects: Vec<Rect>,
    scale: Vector2<f32>,
}

impl Blocks {
    pub fn new(tileset: Image) -> Blocks {
        let size = tileset.width() as usize / BLOCKS_NUM;

        if tileset.height() as usize != size {
            log::error!(
                "Invalid tileset size {}:{}",
                tileset.width(),
                tileset.height()
            );
            std::process::exit(1);
        }

        let batch = SpriteBatch::new(tileset);

        let mut rects: Vec<Rect> = Vec::with_capacity(BLOCKS_NUM);

        for i in 0..BLOCKS_NUM {
            rects.push(Rect::new(
                i as f32 / BLOCKS_NUM as f32,
                0.0,
                1.0 / BLOCKS_NUM as f32,
                1.0,
            ));
        }

        let scale = BLOCK_SIZE as f32 / size as f32;
        let scale = Vector2::new(scale, scale);

        Blocks {
            batch,
            rects,
            scale,
        }
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }

    pub fn add(&mut self, block_id: usize, dest: Point2<f32>, alpha: f32) {
        match block_id {
            1..=BLOCKS_NUM => {
                self.batch.add(
                    DrawParam::new()
                        .src(self.rects[block_id - 1])
                        .dest(dest)
                        .scale(self.scale)
                        .color(Color::new(1.0, 1.0, 1.0, alpha)),
                );
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
