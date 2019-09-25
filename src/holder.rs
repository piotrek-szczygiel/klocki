use ggez::{
    graphics::Align,
    graphics::{self, Color, DrawParam, Font, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

use crate::{
    bag::Bag,
    blocks::Blocks,
    shape::{Shape, ShapeType},
};

#[derive(Default)]
pub struct Holder {
    shape: Option<Shape>,
    locked: bool,
}

impl Holder {
    pub fn hold(&mut self, shape_type: ShapeType, bag: &mut Bag) -> Option<ShapeType> {
        if self.locked {
            return None;
        }

        self.locked = true;

        let mut swap = Some(Shape::new(shape_type));
        std::mem::swap(&mut self.shape, &mut swap);

        match swap {
            None => Some(bag.pop()),
            Some(s) => Some(s.shape_type),
        }
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        position: Point2<f32>,
        blocks: &mut Blocks,
        block_size: i32,
        text_color: Color,
        font: Font,
    ) -> GameResult {
        let mut text = Text::new(TextFragment {
            text: "Hold".to_string(),
            color: Some(text_color),
            font: Some(font),
            scale: Some(Scale::uniform(block_size as f32 * 2.0)),
        });

        text.set_bounds(
            Point2::new(block_size as f32 * 6.0, block_size as f32),
            Align::Center,
        );

        graphics::draw(ctx, &text, DrawParam::new().dest(position))?;

        let position = position + Vector2::new(0.0, block_size as f32 * 2.5);

        if let Some(shape) = &self.shape {
            let position = position
                + Vector2::new(
                    block_size as f32 * 3.0 - shape.grids[0].width as f32 * block_size as f32 / 2.0,
                    0.0,
                );
            shape.draw(ctx, 0, position, blocks, block_size, 1.0)?;
        }

        Ok(())
    }
}
