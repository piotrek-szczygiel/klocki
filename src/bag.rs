use rand::{seq::SliceRandom, thread_rng};

use crate::shape::Shape;

pub struct Bag {
    bag: Vec<Shape>,
}

impl Bag {
    pub fn new() -> Self {
        let mut bag = Bag {
            bag: Vec::with_capacity(14),
        };

        bag.fill();
        bag
    }

    pub fn pop(&mut self) -> Option<Shape> {
        let shape = self.bag.pop();
        self.fill();
        shape
    }

    pub fn peek(&self, mut n: usize) -> &[Shape] {
        if self.bag.len() < n {
            n = self.bag.len();
        }

        &self.bag[..n]
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
        let mut shapes = Shape::get_all_shapes();
        shapes.shuffle(&mut thread_rng());
        self.bag.extend(shapes);
    }
}

#[test]
fn bag_test() {
    use crate::shape::ShapeType;

    let mut bag = Bag::new();
    assert_eq!(14, bag.peek(14).len());

    for _ in 0..7 {
        bag.pop();
    }

    let mut types = Vec::<ShapeType>::with_capacity(7);

    for _ in 0..7 {
        let shape = bag.pop().unwrap();
        types.push(shape.shape_type);
    }

    let shapes = Shape::get_all_shapes();

    for shape in shapes {
        assert!(types.contains(&shape.shape_type));
    }
}
