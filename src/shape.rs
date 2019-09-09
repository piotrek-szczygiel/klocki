pub type Kick = [(i32, i32); 4];
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
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
    pub grid: [[usize; 4]; 4],
}

impl ShapeGrid {
    fn new(
        offset_x: i32,
        offset_y: i32,
        width: i32,
        height: i32,
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
    Z = 1,
    L,
    O,
    S,
    I,
    J,
    T,
}

pub fn all_shape_types() -> Vec<ShapeType> {
    use ShapeType::*;
    vec![Z, L, O, S, I, J, T]
}

pub struct Shape {
    pub shape_type: ShapeType,
    pub grids: [ShapeGrid; 4],
    pub kicks: Kicks,
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Shape {
        match shape_type {
            ShapeType::Z => {
                let x = ShapeType::Z as usize;
                Shape {
                    shape_type: ShapeType::Z,
                    grids: [
                        ShapeGrid::new(
                            0,
                            0,
                            3,
                            2,
                            [[x, x, 0, 0], [0, x, x, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            1,
                            0,
                            2,
                            3,
                            [[0, 0, x, 0], [0, x, x, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            1,
                            3,
                            2,
                            [[0, 0, 0, 0], [x, x, 0, 0], [0, x, x, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            3,
                            [[0, x, 0, 0], [x, x, 0, 0], [x, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_JLSTZ,
                }
            }
            ShapeType::L => {
                let x = ShapeType::L as usize;
                Shape {
                    shape_type: ShapeType::L,
                    grids: [
                        ShapeGrid::new(
                            0,
                            0,
                            3,
                            2,
                            [[0, 0, x, 0], [x, x, x, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            1,
                            0,
                            2,
                            3,
                            [[0, x, 0, 0], [0, x, 0, 0], [0, x, x, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            1,
                            3,
                            2,
                            [[0, 0, 0, 0], [x, x, x, 0], [x, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            3,
                            [[x, x, 0, 0], [0, x, 0, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_JLSTZ,
                }
            }
            ShapeType::O => {
                let x = ShapeType::O as usize;
                Shape {
                    shape_type: ShapeType::O,
                    grids: [
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            2,
                            [[x, x, 0, 0], [x, x, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            2,
                            [[x, x, 0, 0], [x, x, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            2,
                            [[x, x, 0, 0], [x, x, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            2,
                            [[x, x, 0, 0], [x, x, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_JLSTZ,
                }
            }
            ShapeType::S => {
                let x = ShapeType::S as usize;
                Shape {
                    shape_type: ShapeType::S,
                    grids: [
                        ShapeGrid::new(
                            0,
                            0,
                            3,
                            2,
                            [[0, x, x, 0], [x, x, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            1,
                            0,
                            2,
                            3,
                            [[0, x, 0, 0], [0, x, x, 0], [0, 0, x, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            1,
                            3,
                            2,
                            [[0, 0, 0, 0], [0, x, x, 0], [x, x, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            3,
                            [[x, 0, 0, 0], [x, x, 0, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_JLSTZ,
                }
            }
            ShapeType::I => {
                let x = ShapeType::I as usize;
                Shape {
                    shape_type: ShapeType::I,
                    grids: [
                        ShapeGrid::new(
                            0,
                            1,
                            4,
                            1,
                            [[0, 0, 0, 0], [x, x, x, x], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            2,
                            0,
                            1,
                            4,
                            [[0, 0, x, 0], [0, 0, x, 0], [0, 0, x, 0], [0, 0, x, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            2,
                            4,
                            1,
                            [[0, 0, 0, 0], [0, 0, 0, 0], [x, x, x, x], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            1,
                            0,
                            1,
                            4,
                            [[0, x, 0, 0], [0, x, 0, 0], [0, x, 0, 0], [0, x, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_I,
                }
            }
            ShapeType::J => {
                let x = ShapeType::J as usize;
                Shape {
                    shape_type: ShapeType::J,
                    grids: [
                        ShapeGrid::new(
                            0,
                            0,
                            3,
                            2,
                            [[x, 0, 0, 0], [x, x, x, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            1,
                            0,
                            2,
                            3,
                            [[0, x, x, 0], [0, x, 0, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            1,
                            3,
                            2,
                            [[0, 0, 0, 0], [x, x, x, 0], [0, 0, x, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            3,
                            [[0, x, 0, 0], [0, x, 0, 0], [x, x, 0, 0], [0, 0, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_JLSTZ,
                }
            }
            ShapeType::T => {
                let x = ShapeType::T as usize;
                Shape {
                    shape_type: ShapeType::T,
                    grids: [
                        ShapeGrid::new(
                            0,
                            0,
                            3,
                            2,
                            [[0, x, 0, 0], [x, x, x, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            1,
                            0,
                            2,
                            3,
                            [[0, x, 0, 0], [0, x, x, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            1,
                            3,
                            2,
                            [[0, 0, 0, 0], [x, x, x, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                        ShapeGrid::new(
                            0,
                            0,
                            2,
                            3,
                            [[0, x, 0, 0], [x, x, 0, 0], [0, x, 0, 0], [0, 0, 0, 0]],
                        ),
                    ],
                    kicks: KICKS_JLSTZ,
                }
            }
        }
    }
}
