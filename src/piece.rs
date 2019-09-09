use crate::{
    blocks::{Blocks, BLOCK_SIZE},
    matrix::{self, Matrix},
    shape::{Shape, ShapeGrid, ShapeType},
};
use ggez::{self, nalgebra::Point2, Context, GameResult};

pub struct Piece {
    shape: Shape,
    pub x: i32,
    pub y: i32,
    rotation: usize,
    last_movement: Movement,
}

pub enum Movement {
    None,
    Shift,
    Rotate,
}

impl Piece {
    pub fn new(shape_type: ShapeType) -> Piece {
        let mut piece = Piece {
            shape: Shape::new(shape_type),
            x: 0,
            y: 0,
            rotation: 0,
            last_movement: Movement::None,
        };

        piece.reset();
        piece
    }

    pub fn reset(&mut self) {
        self.x = 5 - (self.shape.grids[0].width + 1) / 2;
        self.y = matrix::HEIGHT - self.shape.grids[0].height - self.shape.grids[0].offset_y;
        self.rotation = 0;
        self.last_movement = Movement::None;
    }

    pub fn shift(&mut self, x: i32, y: i32, matrix: &Matrix) -> bool {
        if self.collision(x, y, matrix) {
            return false;
        }

        self.x += x;
        self.y += y;
        self.last_movement = Movement::Shift;
        true
    }

    pub fn rotate(&mut self, clockwise: bool, matrix: &Matrix) -> bool {
        let kicks = self.shape.kicks[self.rotation];
        let last_rotation = self.rotation;
        let mut rotated = false;

        let kicks = if clockwise { kicks.0 } else { kicks.1 };

        if clockwise && self.rotation == 3 {
            self.rotation = 0;
        } else if clockwise {
            self.rotation += 1;
        } else if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }

        if !matrix.collision(&self) {
            rotated = true;
        } else {
            for kick in &kicks {
                if self.shift(kick.0, kick.1, matrix) {
                    rotated = true;
                    break;
                }
            }
        }

        if rotated {
            self.last_movement = Movement::Rotate;
        } else {
            self.rotation = last_rotation;
        }

        rotated
    }

    pub fn fall(&mut self, matrix: &Matrix) -> i32 {
        let mut rows = 0;
        while self.shift(0, 1, &matrix) {
            rows += 1;
        }

        if rows > 0 {
            self.last_movement = Movement::None;
        }

        rows
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

                let x = self.x + x as i32;
                let y = self.y + y as i32 - matrix::VANISH;

                let dest = Point2::new(
                    position[0] + (x * BLOCK_SIZE) as f32,
                    position[1] + (y * BLOCK_SIZE) as f32,
                );

                blocks.add(block, dest);
            }
        }

        blocks.draw(ctx)?;

        Ok(())
    }

    fn collision(&mut self, x: i32, y: i32, matrix: &Matrix) -> bool {
        self.x += x;
        self.y += y;

        let result = matrix.collision(&self);

        self.x -= x;
        self.y -= y;
        result
    }
}
