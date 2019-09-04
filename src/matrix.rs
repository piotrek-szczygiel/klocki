use crate::blocks::{Blocks, BLOCK_SIZE};

use ggez::{
    graphics::{self, Mesh},
    nalgebra::Point2,
    Context, GameResult,
};
use rand_distr::{Distribution, Uniform};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const VANISH: usize = 20;

type Grid = [[usize; WIDTH]; HEIGHT + VANISH];

pub struct Matrix {
    grid: Grid,
    grid_mesh: Mesh,
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let grid_mesh = &mut graphics::MeshBuilder::new();
        let grid_color = graphics::Color::new(0.3, 0.3, 0.3, 0.5);

        for x in 0..=WIDTH {
            let x = (x * BLOCK_SIZE) as f32;

            grid_mesh.line(
                &[
                    Point2::new(x, 0.0),
                    Point2::new(x, (BLOCK_SIZE * HEIGHT) as f32),
                ],
                2.0,
                grid_color,
            )?;
        }

        for y in 0..=HEIGHT {
            let y = (y * BLOCK_SIZE) as f32;

            grid_mesh.line(
                &[
                    Point2::new(0.0, y),
                    Point2::new((BLOCK_SIZE * WIDTH) as f32, y),
                ],
                2.0,
                grid_color,
            )?;
        }

        let grid_mesh = grid_mesh.build(ctx)?;

        Ok(Matrix {
            grid: [[0; WIDTH]; HEIGHT + VANISH],
            grid_mesh,
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
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        blocks: &mut Blocks,
    ) -> GameResult {
        graphics::draw(
            ctx,
            &self.grid_mesh,
            graphics::DrawParam::new().dest(position),
        )?;

        blocks.clear();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let block = self.grid[y][x];
                if block == 0 {
                    continue;
                }

                let dest = Point2::new(
                    position[0] + (x * BLOCK_SIZE) as f32,
                    position[1] + ((HEIGHT - y - 1) * BLOCK_SIZE) as f32,
                );

                blocks.add(block, dest);
            }
        }

        blocks.draw(ctx)?;

        Ok(())
    }
}
