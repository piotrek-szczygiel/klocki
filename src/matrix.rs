use crate::{
    blocks::{Blocks, BLOCK_SIZE},
    piece::Piece,
};

use ggez::{
    graphics::{self, Color, DrawParam, Mesh, MeshBuilder},
    nalgebra::Point2,
    Context, GameResult,
};
use rand_distr::{Distribution, Uniform};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const VANISH: usize = 20;

type Grid = [[usize; WIDTH]; HEIGHT + VANISH];

pub struct Matrix {
    grid: Grid,
    grid_mesh: Mesh,
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> GameResult<Matrix> {
        let grid_mesh = &mut MeshBuilder::new();
        let grid_color = Color::new(0.2, 0.2, 0.2, 1.0);

        for x in 0..=WIDTH {
            let x = (x * BLOCK_SIZE) as f32;

            grid_mesh.line(
                &[
                    Point2::new(x, 0.0),
                    Point2::new(x, (BLOCK_SIZE * HEIGHT) as f32),
                ],
                1.0,
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
                1.0,
                grid_color,
            )?;
        }

        let grid_mesh = grid_mesh.build(ctx)?;

        Ok(Matrix {
            grid: [[0; WIDTH]; HEIGHT + VANISH],
            grid_mesh,
        })
    }

    pub fn clear(&mut self) {
        self.grid = [[0; WIDTH]; HEIGHT + VANISH];
    }

    pub fn collision(&self, piece: &Piece) -> bool {
        let grid = piece.get_grid();
        let x = piece.position[0] + grid.offset_x;
        let y = piece.position[1] + grid.offset_y;

        if x + grid.width > WIDTH {
            return true;
        } else if y + grid.height > HEIGHT + VANISH {
            return true;
        }

        for my in 0..grid.height {
            for mx in 0..grid.width {
                let c = grid.grid[my + grid.offset_y][mx + grid.offset_x];
                if c != 0 && self.grid[y + my][x + mx] != 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        blocks: &mut Blocks,
    ) -> GameResult {
        graphics::draw(ctx, &self.grid_mesh, DrawParam::new().dest(position))?;

        blocks.clear();

        for y in 0..HEIGHT + 1 {
            for x in 0..WIDTH {
                let block = self.grid[VANISH + y - 1][x];
                if block == 0 {
                    continue;
                }

                let dest = Point2::new(
                    position[0] + (x * BLOCK_SIZE) as f32,
                    position[1] + ((y - 1) * BLOCK_SIZE) as f32,
                );

                blocks.add(block, dest);
            }
        }

        blocks.draw(ctx)?;

        Ok(())
    }

    pub fn debug_tower(&mut self) {
        let mut bricks: Vec<(usize, usize)> = vec![
            (39, 0),
            (39, 1),
            (38, 0),
            (37, 0),
            (37, 1),
            (36, 0),
            (36, 1),
            (35, 0),
            (34, 0),
            (34, 1),
            (33, 0),
            (33, 1),
            (32, 0),
            (31, 0),
            (31, 1),
            (30, 0),
            (30, 1),
            (29, 0),
            (28, 0),
            (28, 1),
            (26, 2),
            (25, 2),
        ];

        for y in 0..14 {
            bricks.push((39 - y, 3));
        }

        for y in 0..12 {
            for x in 4..10 {
                bricks.push((39 - y, x));
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
