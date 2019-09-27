use std::time::Duration;

use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect},
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};
use rand_distr::{Distribution, Normal, Uniform};

use crate::{blocks::Blocks, global::Global, piece::Piece, utils};

struct Clearing {
    rows: Vec<i32>,
    cleared: bool,
    current_duration: Duration,
    max_duration: Duration,
}

pub type Grid = Vec<Vec<usize>>;

pub struct Stack {
    pub width: i32,
    pub height: i32,
    pub vanish: i32,

    clearing: Option<Clearing>,
    destroyed_blocks: Vec<DestroyedBlock>,
    randomizer: Randomizer,
    game_over: bool,

    grid: Grid,
    grid_mesh: Option<(Mesh, i32)>,
    block_size: i32,
    update_grid: bool,
}

struct DestroyedBlock {
    block_id: usize,
    position: Vector2<f32>,
    speed: Vector2<f32>,
    rotation: f32,
    rotation_speed: f32,
    visible: Duration,
    lifetime: Duration,
    alpha: f32,
}

struct Randomizer {
    pub uniform_vx: Uniform<f32>,
    pub normal_vy: Normal<f32>,
    pub uniform_vr: Uniform<f32>,
    pub uniform_lifetime: Uniform<u64>,
}

impl Randomizer {
    fn new() -> Randomizer {
        Randomizer {
            uniform_vx: Uniform::new(-7.5, 7.5),
            normal_vy: Normal::new(-10.0, 5.0).unwrap(),
            uniform_vr: Uniform::new(-8.0 * std::f32::consts::PI, 8.0 * std::f32::consts::PI),
            uniform_lifetime: Uniform::new(500, 1000),
        }
    }
}

pub enum Locked {
    Collision,
    Success(i32),
}

impl Stack {
    pub fn new(width: i32, height: i32, vanish: i32) -> Stack {
        Stack {
            width,
            height,
            vanish,
            clearing: None,
            destroyed_blocks: vec![],
            randomizer: Randomizer::new(),
            game_over: false,
            grid: vec![vec![0; width as usize]; (height + vanish) as usize],
            grid_mesh: None,
            block_size: 0,
            update_grid: true,
        }
    }

    pub fn build_grid(&mut self, ctx: &mut Context, grid: bool, outline: bool) -> GameResult {
        let mut grid_mesh = MeshBuilder::new();

        const GRID_COLOR: Color = Color::new(0.1, 0.11, 0.12, 0.5);
        const OUTLINE_COLOR: Color = Color::new(0.8, 0.9, 1.0, 0.8);
        const BACKGROUND_COLOR: Color = Color::new(0.02, 0.03, 0.04, 0.95);

        const GRID_WIDTH: f32 = 1.0;
        const OUTLINE_WIDTH: f32 = 3.0;

        grid_mesh.rectangle(
            DrawMode::fill(),
            Rect::new(
                0.0,
                0.0,
                (self.width * self.block_size) as f32,
                (self.height * self.block_size) as f32,
            ),
            BACKGROUND_COLOR,
        );

        if grid {
            for y in self.vanish..self.vanish + self.height {
                for x in 0..self.width {
                    if self.grid[y as usize][x as usize] != 0 {
                        continue;
                    }

                    let y = y - self.vanish;

                    grid_mesh.rectangle(
                        DrawMode::stroke(GRID_WIDTH),
                        Rect::new(
                            (x * self.block_size) as f32,
                            (y * self.block_size) as f32,
                            self.block_size as f32,
                            self.block_size as f32,
                        ),
                        GRID_COLOR,
                    );
                }
            }
        }

        if outline {
            for y in self.vanish..self.vanish + self.height {
                for x in 0..self.width {
                    if self.grid[y as usize][x as usize] == 0 {
                        continue;
                    }

                    let mut up = false;
                    let mut down = false;
                    let mut left = false;
                    let mut right = false;

                    if y == self.vanish || self.grid[y as usize - 1][x as usize] == 0 {
                        up = true;
                    }

                    if y == self.vanish + self.height - 1
                        || self.grid[y as usize + 1][x as usize] == 0
                    {
                        down = true;
                    }

                    if x == 0 || self.grid[y as usize][x as usize - 1] == 0 {
                        left = true;
                    }

                    if x == self.width - 1 || self.grid[y as usize][x as usize + 1] == 0 {
                        right = true;
                    }

                    let y = y - self.vanish;

                    let corner = 1.0;

                    if up {
                        grid_mesh.line(
                            &[
                                Point2::new(
                                    (x * self.block_size) as f32 - corner,
                                    (y * self.block_size) as f32,
                                ),
                                Point2::new(
                                    ((x + 1) * self.block_size) as f32 + corner,
                                    (y * self.block_size) as f32,
                                ),
                            ],
                            OUTLINE_WIDTH,
                            OUTLINE_COLOR,
                        )?;
                    }

                    if down {
                        grid_mesh.line(
                            &[
                                Point2::new(
                                    (x * self.block_size) as f32 - corner,
                                    ((y + 1) * self.block_size) as f32,
                                ),
                                Point2::new(
                                    ((x + 1) * self.block_size) as f32 + corner,
                                    ((y + 1) * self.block_size) as f32,
                                ),
                            ],
                            OUTLINE_WIDTH,
                            OUTLINE_COLOR,
                        )?;
                    }

                    if left {
                        grid_mesh.line(
                            &[
                                Point2::new(
                                    (x * self.block_size) as f32,
                                    (y * self.block_size) as f32 - corner,
                                ),
                                Point2::new(
                                    (x * self.block_size) as f32,
                                    ((y + 1) * self.block_size) as f32 + corner,
                                ),
                            ],
                            OUTLINE_WIDTH,
                            OUTLINE_COLOR,
                        )?;
                    }

                    if right {
                        grid_mesh.line(
                            &[
                                Point2::new(
                                    ((x + 1) * self.block_size) as f32,
                                    (y * self.block_size) as f32 - corner,
                                ),
                                Point2::new(
                                    ((x + 1) * self.block_size) as f32,
                                    ((y + 1) * self.block_size) as f32 + corner,
                                ),
                            ],
                            OUTLINE_WIDTH,
                            OUTLINE_COLOR,
                        )?;
                    }
                }
            }
        }

        grid_mesh.rectangle(
            DrawMode::stroke(OUTLINE_WIDTH),
            Rect::new(
                0.0,
                0.0,
                (self.width * self.block_size) as f32,
                (self.height * self.block_size) as f32,
            ),
            OUTLINE_COLOR,
        );

        self.grid_mesh = Some((grid_mesh.build(ctx)?, self.block_size));

        Ok(())
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn clear(&mut self) {
        self.update_grid = true;
        self.grid = vec![vec![0; self.width as usize]; (self.height + self.vanish) as usize]
    }

    pub fn collision(&self, piece: &Piece) -> bool {
        let grid = piece.grid();
        let x = piece.x + grid.offset_x;
        let y = piece.y + grid.offset_y;

        if x < 0
            || x + grid.width > self.width
            || y < 0
            || y + grid.height > self.height + self.vanish
        {
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

    pub fn lock(&mut self, piece: &Piece, clear_delay: Duration) -> Locked {
        self.update_grid = true;
        let mut collision = self.collision(&piece);

        let grid = piece.grid();
        let x = piece.x + grid.offset_x;
        let y = piece.y + grid.offset_y;

        if y + grid.height <= self.vanish {
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
            Locked::Success(self.clear_full_rows(clear_delay))
        } else {
            Locked::Collision
        }
    }

    pub fn blocked(&self) -> bool {
        self.clearing.is_some()
    }

    pub fn update(&mut self, ctx: &mut Context, g: &mut Global, sfx: bool) -> GameResult {
        if let Some(clearing) = &mut self.clearing {
            clearing.current_duration += timer::delta(ctx);

            if !clearing.cleared {
                clearing.cleared = true;
                for &y in &clearing.rows {
                    for x in 0..self.width {
                        self.grid[y as usize][x as usize] = 0;
                    }
                }
            }

            if clearing.current_duration >= clearing.max_duration {
                for &y in &clearing.rows {
                    for y in (1..=y).rev() {
                        for x in 0..self.width {
                            self.grid[y as usize][x as usize] =
                                self.grid[y as usize - 1][x as usize];
                        }
                    }
                }

                if sfx {
                    g.sfx.play("linefall");
                }

                self.clearing = None;
                self.update_grid = true;
            }
        }

        let dt = utils::dt_f32(ctx);
        let g_force = Vector2::new(0.0, 75.0) * dt;

        for block in &mut self.destroyed_blocks {
            block.speed += g_force;
            block.position += block.speed * dt;
            block.rotation += block.rotation_speed * dt;
            block.visible += timer::delta(ctx);
            block.alpha = (1.0
                - timer::duration_to_f64(block.visible) / timer::duration_to_f64(block.lifetime))
                as f32;
        }

        self.destroyed_blocks
            .retain(|block| block.visible < block.lifetime);

        if self.update_grid {
            self.build_grid(
                ctx,
                g.settings.gameplay.stack_grid,
                g.settings.gameplay.stack_outline,
            )?;
            self.update_grid = false;
        }

        Ok(())
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        blocks: &mut Blocks,
        block_size: i32,
    ) -> GameResult {
        if self.block_size != block_size {
            self.block_size = block_size;
            self.update_grid = true;
        }

        blocks.clear();

        let alpha = 0.6;

        for y in 0..=self.height {
            for x in 0..self.width {
                let block = self.grid[(self.vanish + y - 1) as usize][x as usize];
                if block == 0 {
                    continue;
                }

                let destination = Point2::new(
                    position[0] + (x * block_size) as f32,
                    position[1] + ((y - 1) * block_size) as f32,
                );

                blocks.add(block, block_size, destination, alpha);
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
                    .color(Color::new(1.0, 1.0, 1.0, alpha * block.alpha)),
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
        lifetime: Duration,
    ) {
        let block_id = self.grid[y as usize][x as usize];
        if block_id != 0 {
            self.destroyed_blocks.push(DestroyedBlock {
                block_id,
                position: Vector2::new(x as f32, (y - self.vanish) as f32),
                speed,
                rotation: 0.0,
                rotation_speed,
                visible: Duration::new(0, 0),
                lifetime,
                alpha: 1.0,
            });
        }
    }

    fn clear_full_rows(&mut self, clear_delay: Duration) -> i32 {
        let rows = self.get_full_rows();
        let length = rows.len();

        if length > 0 {
            self.clear_rows(&rows, clear_delay);
        }

        length as i32
    }

    fn get_full_rows(&self) -> Vec<i32> {
        let mut rows = vec![];

        for y in 0..self.height + self.vanish {
            let mut full = true;

            for x in 0..self.width {
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

    fn clear_rows(&mut self, rows: &[i32], clear_delay: Duration) {
        self.update_grid = true;
        let mut rng = rand::thread_rng();

        for &row in rows {
            for x in 0..self.width {
                let vx = self.randomizer.uniform_vx.sample(&mut rng);
                let vy = self.randomizer.normal_vy.sample(&mut rng);
                let vr = self.randomizer.uniform_vr.sample(&mut rng);
                let lifetime =
                    Duration::from_millis(self.randomizer.uniform_lifetime.sample(&mut rng));
                self.generate_destroyed_block(x, row, Vector2::new(vx, vy), vr, lifetime);
            }
        }

        self.clearing = Some(Clearing {
            rows: Vec::from(rows),
            cleared: false,
            current_duration: Duration::new(0, 0),
            max_duration: clear_delay,
        });
    }

    pub fn game_over(&mut self) {
        let mut rows = vec![];
        for y in 0..self.height + self.vanish {
            for x in 0..self.width {
                if self.grid[y as usize][x as usize] != 0 {
                    rows.push(y);
                    break;
                }
            }
        }

        self.clear_rows(&rows, Duration::new(0, 0));
        self.game_over = true;
    }

    pub fn debug_tetris(&mut self) {
        let mut bricks: Vec<(usize, usize)> = vec![];
        for y in 24..40 {
            for x in 0..9 {
                bricks.push((y, x));
            }
        }

        self.clear();

        for (y, x) in bricks {
            self.grid[y][x] = 5;
        }
    }

    pub fn debug_t_spin(&mut self) {
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