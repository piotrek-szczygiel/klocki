use rand::{seq::SliceRandom, thread_rng};

use crate::shape::{self, ShapeType};

pub struct Bag {
    bag: Vec<ShapeType>,
}

impl Bag {
    pub fn new() -> Bag {
        let mut bag = Bag {
            bag: Vec::with_capacity(14),
        };

        bag.fill();
        bag
    }

    pub fn pop(&mut self) -> ShapeType {
        let shape = self.bag.pop();
        self.fill();
        shape.unwrap()
    }

    pub fn peek(&self, mut n: usize) -> &[ShapeType] {
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
        let mut shapes = shape::all_shape_types();
        shapes.shuffle(&mut thread_rng());
        self.bag.extend(shapes);
    }
}

#[test]
fn bag_test() {
    let mut bag = Bag::new();
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
