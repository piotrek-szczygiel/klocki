use ggez::{self, nalgebra::Point2, Context, GameResult};

use crate::{
    blocks::Blocks,
    shape::{Shape, ShapeGrid, ShapeType},
    stack::Stack,
};

#[derive(Clone, PartialEq)]
pub enum Movement {
    None,
    Shift,
    Rotate,
}

#[derive(Clone)]
pub struct Piece {
    shape: Shape,
    pub x: i32,
    pub y: i32,
    rotation: usize,
    last_movement: Movement,
}

impl Piece {
    pub fn new(shape_type: ShapeType, stack: &Stack) -> Piece {
        let mut piece = Piece {
            shape: Shape::new(shape_type),
            x: 0,
            y: 0,
            rotation: 0,
            last_movement: Movement::None,
        };

        piece.reset(&stack);
        piece
    }

    pub fn t_spin(&self, stack: &Stack) -> bool {
        if self.shape.shape_type != ShapeType::T || self.last_movement != Movement::Rotate {
            return false;
        }

        // Position of the center tile
        let x = self.x as usize + 1;
        let y = self.y as usize + 1;

        let mut occupied = 0;

        let last_horizontal = stack.width as usize - 1;
        let last_vertical = (stack.height + stack.vanish) as usize - 1;

        let stack = stack.grid();

        if x == 0 || stack[y - 1][x - 1] != 0 {
            occupied += 1;
        }

        if x == last_horizontal || stack[y - 1][x + 1] != 0 {
            occupied += 1;
        }

        if x == 0 || y == last_vertical || stack[y + 1][x - 1] != 0 {
            occupied += 1;
        }

        if x == last_horizontal || y == last_vertical || stack[y + 1][x + 1] != 0 {
            occupied += 1;
        }

        occupied >= 3
    }

    pub fn reset(&mut self, stack: &Stack) {
        self.x = (stack.width as f32 / 2.0 - self.shape.grids[0].width as f32 / 2.0) as i32;
        self.y = stack.vanish - self.shape.grids[0].height - self.shape.grids[0].offset_y;
        self.rotation = 0;
        self.last_movement = Movement::None;
    }

    pub fn shift(&mut self, x: i32, y: i32, stack: &Stack) -> bool {
        if self.collision(x, y, stack) {
            return false;
        }

        self.x += x;
        self.y += y;
        self.last_movement = Movement::Shift;
        true
    }

    pub fn rotate(&mut self, clockwise: bool, stack: &Stack) -> bool {
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

        if !stack.collision(&self) {
            rotated = true;
        } else {
            for kick in &kicks {
                if self.shift(kick.0, kick.1, stack) {
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

    pub fn fall(&mut self, stack: &Stack) -> i32 {
        let mut rows = 0;
        while self.shift(0, 1, &stack) {
            rows += 1;
        }

        if rows > 0 {
            self.last_movement = Movement::None;
        }

        rows
    }

    pub fn touching_floor(&mut self, stack: &Stack) -> bool {
        self.collision(0, 1, stack)
    }

    pub fn grid(&self) -> &ShapeGrid {
        &self.shape.grids[self.rotation]
    }

    pub fn shape(&self) -> ShapeType {
        self.shape.shape_type
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        position: Point2<f32>,
        vanish: i32,
        blocks: &mut Blocks,
        block_size: i32,
        alpha: f32,
    ) -> GameResult {
        blocks.clear();

        let position = Point2::new(
            position[0] + (self.x * block_size) as f32,
            position[1] + ((self.y - vanish) * block_size) as f32,
        );

        self.shape
            .draw(ctx, self.rotation, position, blocks, block_size, alpha)?;

        Ok(())
    }

    fn collision(&mut self, x: i32, y: i32, stack: &Stack) -> bool {
        self.x += x;
        self.y += y;

        let result = stack.collision(&self);

        self.x -= x;
        self.y -= y;
        result
    }
}
