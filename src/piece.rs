use crate::{
    blocks::{Blocks, BLOCK_SIZE},
    matrix::{self, Matrix},
    shape::{Shape, ShapeGrid, ShapeType},
};
use ggez::{self, nalgebra::Point2, Context, GameResult};

pub struct Piece {
    shape: Shape,
    pub position: Point2<usize>,
    rotation: usize,
}

impl Piece {
    pub fn new(shape_type: ShapeType, position: Point2<usize>, rotation: usize) -> Piece {
        Piece {
            shape: Shape::new(shape_type),
            position,
            rotation,
        }
    }

    pub fn move_piece(&mut self, x: i64, y: i64, matrix: &Matrix) -> bool {
        if matrix.collision(&self) {
            return false;
        }

        let new_position = Point2::new(self.position[0] as i64 + x, self.position[1] as i64 + y);

        if new_position[0] < 0 || new_position[1] < 0 {
            return false;
        }

        let old_position = self.position;
        self.position = Point2::new(new_position[0] as usize, new_position[1] as usize);

        if matrix.collision(&self) {
            self.position = old_position;
            false
        } else {
            true
        }
    }

    pub fn get_grid(&self) -> &ShapeGrid {
        &self.shape.grids[self.rotation]
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        blocks: &mut Blocks,
    ) -> GameResult {
        blocks.clear();

        let grid = self.get_grid();

        for y in 0..4 {
            for x in 0..4 {
                let block = grid.grid[y][x];
                if block == 0 {
                    continue;
                }

                let x = self.position[0] + x;
                let y = self.position[1] as i64 + y as i64 - matrix::VANISH as i64;

                let dest = Point2::new(
                    position[0] + (x * BLOCK_SIZE) as f32,
                    position[1] + (y * BLOCK_SIZE as i64) as f32,
                );

                blocks.add(block, dest);
            }
        }

        blocks.draw(ctx)?;

        Ok(())
    }
}
