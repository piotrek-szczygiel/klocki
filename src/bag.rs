use std::{
    collections::{vec_deque::Iter, VecDeque},
    iter::Take,
};

use ggez::{
    graphics::{self, Color, DrawParam, Font, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::{
    blocks::Blocks,
    shape::{self, Shape, ShapeType},
};
use ggez::graphics::Align;

pub struct Bag {
    bag: VecDeque<ShapeType>,
    rng: StdRng,
}

impl Bag {
    pub fn new(seed: &[u8; 32]) -> Bag {
        let rng: StdRng = SeedableRng::from_seed(*seed);

        let mut bag = Bag {
            bag: VecDeque::with_capacity(14),
            rng,
        };

        bag.fill();
        bag
    }

    pub fn pop(&mut self) -> ShapeType {
        let shape = self.bag.pop_front();
        self.fill();
        shape.unwrap()
    }

    pub fn peek(&self, n: usize) -> Take<Iter<ShapeType>> {
        self.bag.iter().take(n)
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
            text: "Next".to_string(),
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

        for (i, &shape) in self.peek(6).enumerate() {
            let shape = Shape::new(shape);
            let position = position
                + Vector2::new(
                    block_size as f32 * 3.0 - shape.grids[0].width as f32 * block_size as f32 / 2.0,
                    (i as i32 * block_size * 3) as f32,
                );
            shape.draw(ctx, 0, position, blocks, block_size, 0.9)?;
        }

        Ok(())
    }

    fn fill(&mut self) {
        match self.bag.len() {
            0 => {
                self.fill_7();
                self.fill_7();
            }
            7 => self.fill_7(),
            _ => (),
        }
    }

    fn fill_7(&mut self) {
        let mut shapes = shape::all_shape_types();
        shapes.shuffle(&mut self.rng);
        self.bag.extend(shapes);
    }
}

#[test]
fn bag_test() {
    let seed = [0; 32];
    let mut bag = Bag::new(&seed);
    assert_eq!(14, bag.peek(14).len());

    for _ in 0..7 {
        bag.pop();
    }

    let mut types = Vec::<ShapeType>::with_capacity(7);

    for _ in 0..7 {
        let shape = bag.pop();
        types.push(shape);
    }

    let shapes = shape::all_shape_types();

    for shape in shapes {
        assert!(types.contains(&shape));
    }
}
