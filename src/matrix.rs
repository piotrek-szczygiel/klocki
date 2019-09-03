use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Image, Mesh, Rect},
    nalgebra::Point2,
    Context, GameResult,
};
use rand_distr::{Distribution, Uniform};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const VANISH: usize = 20;

const BLOCK_SIZE: f32 = 35.0;

type Grid = [[usize; WIDTH]; HEIGHT + VANISH];

pub struct Matrix {
    grid: Grid,
    grid_mesh: Mesh,
    outline_image: Image,
    blocks: SpriteBatch,
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let grid_mesh = &mut graphics::MeshBuilder::new();
        let grid_color = graphics::Color::new(0.3, 0.3, 0.3, 0.5);

        for x in 0..=WIDTH {
            let x = x as f32 * BLOCK_SIZE;

            grid_mesh.line(
                &[
                    Point2::new(x, 0.0),
                    Point2::new(x, BLOCK_SIZE * HEIGHT as f32),
                ],
                2.0,
                grid_color,
            )?;
        }

        for y in 0..=HEIGHT {
            let y = y as f32 * BLOCK_SIZE;

            grid_mesh.line(
                &[
                    Point2::new(0.0, y),
                    Point2::new(BLOCK_SIZE * WIDTH as f32, y),
                ],
                2.0,
                grid_color,
            )?;
        }

        let grid_mesh = grid_mesh.build(ctx)?;

        Ok(Matrix {
            grid: [[0; WIDTH]; HEIGHT + VANISH],
            grid_mesh,
            outline_image: Image::new(ctx, "outline.png")?,
            blocks: SpriteBatch::new(Image::new(ctx, "blocks.png")?),
        })
    }

    fn clear(&mut self) {
        self.grid = [[0; WIDTH]; HEIGHT + VANISH];
    }

    pub fn debug_tower(&mut self) {
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
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(1, 8);

        for (y, x) in bricks {
            self.grid[y][x] = uniform.sample(&mut rng);
        }
    }
}

impl Matrix {
    pub fn draw(&mut self, ctx: &mut Context, position: Point2<f32>) -> GameResult {
        graphics::draw(
            ctx,
            &self.grid_mesh,
            graphics::DrawParam::new().dest(position),
        )?;

        self.blocks.clear();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let block = self.grid[y][x];
                if block == 0 {
                    continue;
                }

                let src = Rect::new((block - 1) as f32 / 7.0, 0.0, 1.0 / 7.0, 1.0);

                let dest = Point2::new(
                    position[0] + x as f32 * BLOCK_SIZE,
                    position[1] + HEIGHT as f32 * BLOCK_SIZE - y as f32 * BLOCK_SIZE,
                );

                self.blocks.add(DrawParam::new().src(src).dest(dest));
            }
        }

        graphics::draw(ctx, &self.blocks, DrawParam::new())?;
        Ok(())
    }
}
