pub type Kick = [(i64, i64); 4];
pub type Kicks = [(Kick, Kick); 4];

const KICKS_JLSTZ: Kicks = [
    (
        [(-1, 0), (-1, -1), (0, 2), (-1, 2)],
        [(1, 0), (1, -1), (0, 2), (1, 2)],
    ),
    (
        [(1, 0), (1, 1), (0, -2), (1, -2)],
        [(1, 0), (1, 1), (0, -2), (1, -2)],
    ),
    (
        [(1, 0), (1, -1), (0, 2), (1, 2)],
        [(-1, 0), (-1, -1), (0, 2), (-1, 2)],
    ),
    (
        [(-1, 0), (-1, 1), (0, -2), (-1, -2)],
        [(-1, 0), (-1, 1), (0, -2), (-1, -2)],
    ),
];

const KICKS_I: Kicks = [
    (
        [(-2, 0), (1, 0), (-2, 1), (1, -2)],
        [(-1, 0), (2, 0), (-1, -2), (2, 1)],
    ),
    (
        [(-1, 0), (2, 0), (-1, -2), (2, 1)],
        [(2, 0), (-1, 0), (2, -1), (-1, 2)],
    ),
    (
        [(2, 0), (-1, 0), (2, -1), (-1, 2)],
        [(1, 0), (-2, 0), (1, 2), (-2, -1)],
    ),
    (
        [(1, 0), (-2, 0), (1, 2), (-2, -1)],
        [(-2, 0), (1, 0), (-2, 1), (1, -2)],
    ),
];

pub struct ShapeGrid {
    pub offset_x: usize,
    pub offset_y: usize,
    pub width: usize,
    pub height: usize,
    pub grid: [[usize; 4]; 4],
}

impl ShapeGrid {
    fn new(
        offset_x: usize,
        offset_y: usize,
        width: usize,
        height: usize,
        grid: [[usize; 4]; 4],
    ) -> ShapeGrid {
        ShapeGrid {
            offset_x,
            offset_y,
            width,
            height,
            grid,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeType {
    I = 1,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Shape {
    pub shape_type: ShapeType,
    pub grids: [ShapeGrid; 4],
    pub kicks: Kicks,
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Shape {
        match shape_type {
            ShapeType::I => Shape {
                shape_type: ShapeType::I,
                grids: [
                    ShapeGrid::new(
                        0,
                        1,
                        4,
                        1,
                        [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        2,
                        0,
                        1,
                        4,
                        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        2,
                        4,
                        1,
                        [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        1,
                        4,
                        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
                    ),
                ],
                kicks: KICKS_I,
            },
            ShapeType::J => Shape {
                shape_type: ShapeType::J,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[2, 0, 0, 0], [2, 2, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 2, 2, 0], [0, 2, 0, 0], [0, 2, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [2, 2, 2, 0], [0, 0, 2, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[0, 2, 0, 0], [0, 2, 0, 0], [2, 2, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::L => Shape {
                shape_type: ShapeType::L,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[0, 0, 3, 0], [3, 3, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 3, 0, 0], [0, 3, 0, 0], [0, 3, 3, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [3, 3, 3, 0], [3, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[3, 3, 0, 0], [0, 3, 0, 0], [0, 3, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::O => Shape {
                shape_type: ShapeType::O,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::S => Shape {
                shape_type: ShapeType::S,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 5, 0, 0], [0, 5, 5, 0], [0, 0, 5, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[5, 0, 0, 0], [5, 5, 0, 0], [0, 5, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::T => Shape {
                shape_type: ShapeType::T,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[0, 6, 0, 0], [6, 6, 6, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 6, 0, 0], [0, 6, 6, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [6, 6, 6, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[0, 6, 0, 0], [6, 6, 0, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::Z => Shape {
                shape_type: ShapeType::Z,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[7, 7, 0, 0], [0, 7, 7, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 0, 7, 0], [0, 7, 7, 0], [0, 7, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [7, 7, 0, 0], [0, 7, 7, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[0, 7, 0, 0], [7, 7, 0, 0], [7, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
        }
    }

    pub fn get_all_shapes() -> Vec<Shape> {
        use ShapeType::*;
        vec![
            Shape::new(Z),
            Shape::new(L),
            Shape::new(O),
            Shape::new(S),
            Shape::new(I),
            Shape::new(J),
            Shape::new(T),
        ]
    }
}
