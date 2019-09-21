use std::{collections::VecDeque, time::Duration};

use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect},
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};
use rand_distr::{Distribution, Normal, Uniform};

use crate::{blocks::Blocks, global::Global, piece::Piece, utils};

pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 20;
pub const VANISH: i32 = 20;

type Grid = [[usize; WIDTH as usize]; (HEIGHT + VANISH) as usize];

pub struct Matrix {
    grid: Grid,
    grid_mesh: Option<(Mesh, i32)>,

    clearing: Option<(VecDeque<i32>, Duration)>,
    destroyed_blocks: Vec<DestroyedBlock>,
    game_over: bool,
}

struct DestroyedBlock {
    block_id: usize,
    position: Vector2<f32>,
    speed: Vector2<f32>,
    rotation: f32,
    rotation_speed: f32,
}

pub enum Locked {
    Collision,
    Success(i32),
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            grid: [[0; WIDTH as usize]; (HEIGHT + VANISH) as usize],
            grid_mesh: None,
            clearing: None,
            destroyed_blocks: vec![],
            game_over: false,
        }
    }

    pub fn build_grid(&mut self, ctx: &mut Context, block_size: i32) -> GameResult {
        let grid_mesh = &mut MeshBuilder::new();
        let grid_color = Color::new(0.5, 0.5, 0.5, 1.0);
        let outline_color = Color::new(0.2, 1.0, 0.8, 0.8);
        let background_color = Color::new(0.03, 0.04, 0.05, 0.95);

        grid_mesh.rectangle(
            DrawMode::fill(),
            Rect::new(
                0.0,
                0.0,
                (WIDTH * block_size) as f32,
                (HEIGHT * block_size) as f32,
            ),
            background_color,
        );

        for y in VANISH..VANISH + HEIGHT {
            for x in 0..WIDTH {
                if self.grid[y as usize][x as usize] != 0 {
                    continue;
                }

                let y = y - VANISH;

                grid_mesh.rectangle(
                    DrawMode::stroke(1.0),
                    Rect::new(
                        (x * block_size) as f32,
                        (y * block_size) as f32,
                        block_size as f32,
                        block_size as f32,
                    ),
                    grid_color,
                );
            }
        }

        grid_mesh.rectangle(
            DrawMode::stroke(2.0),
            Rect::new(
                0.0,
                0.0,
                (WIDTH * block_size) as f32,
                (HEIGHT * block_size) as f32,
            ),
            outline_color,
        );

        self.grid_mesh = Some((grid_mesh.build(ctx)?, block_size));

        Ok(())
    }

    pub fn clear(&mut self) {
        self.grid = [[0; WIDTH as usize]; (HEIGHT + VANISH) as usize];
    }

    pub fn collision(&self, piece: &Piece) -> bool {
        let grid = piece.grid();
        let x = piece.x + grid.offset_x;
        let y = piece.y + grid.offset_y;

        if x < 0 || x + grid.width > WIDTH || y < 0 || y + grid.height > HEIGHT + VANISH {
            return true;
        }

        for my in 0..grid.height {
            for mx in 0..grid.width {
                let c = grid.grid[(my + grid.offset_y) as usize][(mx + grid.offset_x) as usize];
                if c != 0 && self.grid[(y + my) as usize][(x + mx) as usize] != 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn lock(&mut self, piece: &Piece) -> Locked {
        let mut collision = self.collision(&piece);

        let grid = piece.grid();
        let x = piece.x + grid.offset_x;
        let y = piece.y + grid.offset_y;

        if y + grid.height <= VANISH {
            collision = true;
        }

        for my in 0..grid.height {
            for mx in 0..grid.width {
                let c = grid.grid[(my + grid.offset_y) as usize][(mx + grid.offset_x) as usize];
                if c != 0 {
                    self.grid[(y + my) as usize][(x + mx) as usize] = c;
                }
            }
        }

        if !collision {
            Locked::Success(self.clear_full_rows())
        } else {
            Locked::Collision
        }
    }

    pub fn blocked(&self) -> bool {
        self.clearing.is_some()
    }

    pub fn update(&mut self, ctx: &mut Context, g: &mut Global, sfx: bool) {
        let mut clear = None;
        if let Some((rows, duration)) = &mut self.clearing {
            *duration += timer::delta(ctx);

            if rows.is_empty() {
                self.clearing = None;
            } else if *duration > Duration::from_millis(75) {
                *duration = Duration::new(0, 0);
                clear = rows.pop_front();

                if sfx && !self.game_over {
                    g.sfx.play("linefall");
                }

                *rows = rows.iter().map(|row| row + 1).collect();
            }
        }

        if let Some(row) = clear {
            self.collapse_row(row);
        }

        let dt = utils::dt_f32(ctx);
        let g = Vector2::new(0.0, 75.0) * dt;

        for block in &mut self.destroyed_blocks {
            block.speed += g;
            block.position += block.speed * dt;
            block.rotation += block.rotation_speed * dt;
        }

        self.destroyed_blocks
            .retain(|block| block.position[1] < 60.0);
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        blocks: &mut Blocks,
        block_size: i32,
    ) -> GameResult {
        self.build_grid(ctx, block_size)?;

        blocks.clear();

        let alpha = 0.6;

        for y in 0..=HEIGHT {
            for x in 0..WIDTH {
                let block = self.grid[(VANISH + y - 1) as usize][x as usize];
                if block == 0 {
                    continue;
                }

                let dest = Point2::new(
                    position[0] + (x * block_size) as f32,
                    position[1] + ((y - 1) * block_size) as f32,
                );

                blocks.add(block, block_size, dest, alpha);
            }
        }

        graphics::draw(
            ctx,
            &self.grid_mesh.as_ref().unwrap().0,
            DrawParam::new().dest(position),
        )?;

        for block in &self.destroyed_blocks {
            blocks.add_destroyed(
                block.block_id,
                block_size,
                DrawParam::new()
                    .dest(position + block.position * block_size as f32)
                    .rotation(block.rotation)
                    .offset(Point2::new(0.5, 0.5))
                    .color(Color::new(1.0, 1.0, 1.0, alpha)),
            );
        }

        blocks.draw(ctx)?;

        Ok(())
    }

    fn generate_destroyed_block(
        &mut self,
        x: i32,
        y: i32,
        speed: Vector2<f32>,
        rotation_speed: f32,
    ) {
        let block_id = self.grid[y as usize][x as usize];
        if block_id != 0 {
            self.destroyed_blocks.push(DestroyedBlock {
                block_id,
                position: Vector2::new(x as f32, (y - VANISH) as f32),
                speed,
                rotation: 0.0,
                rotation_speed,
            });
        }
    }

    fn clear_full_rows(&mut self) -> i32 {
        let rows = self.get_full_rows();
        let result = rows.len();

        if !rows.is_empty() {
            self.clearing = Some((VecDeque::from(rows), Duration::new(0, 0)));
        }

        result as i32
    }

    fn get_full_rows(&self) -> Vec<i32> {
        let mut rows = vec![];

        for y in (0..HEIGHT + VANISH).rev() {
            let mut full = true;

            for x in 0..WIDTH {
                if self.grid[y as usize][x as usize] == 0 {
                    full = false;
                    break;
                }
            }

            if full {
                rows.push(y);
            }
        }

        rows
    }

    fn collapse_row(&mut self, row: i32) {
        let mut rng = rand::thread_rng();

        let uniform_vx = Uniform::new(-5.0, 5.0);
        let normal_vy = Normal::new(-10.0, 5.0).unwrap();
        let uniform_vr = Uniform::new(-4.0 * std::f32::consts::PI, 4.0 * std::f32::consts::PI);

        for x in 0..WIDTH {
            let vx = uniform_vx.sample(&mut rng);
            let vy = normal_vy.sample(&mut rng);
            let vr = uniform_vr.sample(&mut rng);
            self.generate_destroyed_block(x, row, Vector2::new(vx, vy), vr);
        }

        for y in (1..=row).rev() {
            for x in 0..WIDTH {
                self.grid[y as usize][x as usize] = self.grid[y as usize - 1][x as usize];
            }
        }
    }

    pub fn game_over(&mut self) {
        let mut rows = vec![];
        for y in (0..HEIGHT + VANISH).rev() {
            for x in 0..WIDTH {
                if self.grid[y as usize][x as usize] != 0 {
                    rows.push(y);
                    break;
                }
            }
        }

        self.clearing = Some((VecDeque::from(rows), Duration::new(0, 0)));
        self.game_over = true;
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
