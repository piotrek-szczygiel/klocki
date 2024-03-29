use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image, Rect},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

pub const BLOCKS_NUM: usize = 10;

pub struct Blocks {
    batch: SpriteBatch,
    rects: Vec<Rect>,
    tileset_size: i32,
}

impl Blocks {
    pub fn new(tileset: Image) -> Blocks {
        let tileset_size = (tileset.width() as usize / BLOCKS_NUM) as i32;

        if tileset.height() != tileset_size as u16 {
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

        Blocks {
            batch,
            rects,
            tileset_size,
        }
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }

    pub fn add(&mut self, block_id: usize, size: i32, dest: Point2<f32>, alpha: f32) {
        let scale = size as f32 / self.tileset_size as f32;
        let scale = Vector2::new(scale, scale);

        let color = Color::new(1.0, 1.0, 1.0, alpha);

        self.batch.add(
            DrawParam::new()
                .src(self.rects[block_id])
                .dest(dest)
                .scale(scale)
                .color(color),
        );
    }

    pub fn add_destroyed(&mut self, block_id: usize, size: i32, params: DrawParam) {
        let scale = size as f32 / self.tileset_size as f32;
        let scale = Vector2::new(scale, scale);

        match block_id {
            1..=BLOCKS_NUM => {
                self.batch
                    .add(params.src(self.rects[block_id - 1]).scale(scale));
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
