use rand_distr::{Distribution, Normal, Uniform};

use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, MeshBuilder},
    nalgebra::{self, Point2, Vector2},
    Context, GameResult,
};

use crate::utils;

const MOUSE_THRESHOLD: f32 = 200.0;

struct Particle {
    position: Point2<f32>,
    speed: Vector2<f32>,
    starting_speed: Vector2<f32>,
    size: f32,
    color: Color,
}

impl Particle {
    fn new(n: usize, max_x: f32, max_y: f32) -> Vec<Self> {
        let mut particles: Vec<Self> = vec![];
        let mut rng = rand::thread_rng();

        let uniform_x = Uniform::new(0.01, 0.99);
        let uniform_y = Uniform::new(0.01, 0.99);

        let uniform_vx = Uniform::new(-1.0, 1.0);
        let uniform_vy = Uniform::new(-1.0, 1.0);

        let normal_size = Normal::new(2.0, 0.5).unwrap();
        let uniform_color = Normal::new(0.5, 0.2).unwrap();

        for _ in 0..n {
            let speed = Vector2::new(uniform_vx.sample(&mut rng), uniform_vy.sample(&mut rng));
            let starting_speed = speed.abs();

            let mut size = normal_size.sample(&mut rng);

            if size < 1.0 {
                size = 1.0;
            }

            let position = Point2::new(
                uniform_x.sample(&mut rng) * max_x,
                uniform_y.sample(&mut rng) * max_y,
            );

            let mut color = uniform_color.sample(&mut rng);

            if color < 0.1 {
                color = 0.1;
            } else if color > 1.0 {
                color = 1.0;
            }

            let color = Color::new(color, color, color, 1.0);

            particles.push(Particle {
                position,
                speed,
                starting_speed,
                size,
                color,
            })
        }

        particles
    }
}

pub struct ParticleAnimation {
    particles: Vec<Particle>,
    max_particles: usize,
    threshold: f32,
    max_speed: f32,
    width: f32,
    height: f32,
}

impl ParticleAnimation {
    pub fn new(
        max_particles: usize,
        threshold: f32,
        max_speed: f32,
        width: f32,
        height: f32,
    ) -> Self {
        ParticleAnimation {
            particles: Particle::new(max_particles, width, height),
            max_particles,
            threshold,
            max_speed,
            width,
            height,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = utils::dt(ctx);
        let pos = utils::mouse_position_coords(ctx);

        for particle in &mut self.particles {
            let speed = dt * self.max_speed;
            particle.position += particle.speed * speed;

            if particle.position[0] > self.width - particle.size
                || particle.position[0] < particle.size
            {
                particle.speed[0] = -particle.speed[0];
            }

            if particle.position[1] > self.height - particle.size
                || particle.position[1] < particle.size
            {
                particle.speed[1] = -particle.speed[1];
            }

            let distance = nalgebra::distance(&pos, &particle.position);

            if distance < MOUSE_THRESHOLD {
                let mut direction = Vector2::new(0.0, 0.0);

                if particle.position[0] < pos[0] {
                    direction[0] = -1.0;
                } else {
                    direction[0] = 1.0;
                }

                if particle.position[1] < pos[1] {
                    direction[1] = -1.0;
                } else {
                    direction[1] = 1.0;
                }

                particle.speed +=
                    direction * dt * (MOUSE_THRESHOLD - distance) / MOUSE_THRESHOLD * 50.0;
            } else {
                if particle.speed[0].abs() > particle.starting_speed[0] {
                    particle.speed[0] -=
                        particle.speed[0].signum() * dt * particle.starting_speed[0] * 10.0;
                }

                if particle.speed[1].abs() > particle.starting_speed[1] {
                    particle.speed[1] -=
                        particle.speed[1].signum() * dt * particle.starting_speed[1] * 10.0;
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut mesh_lines = MeshBuilder::new();
        let mut mesh_circles = MeshBuilder::new();

        for i in 0..self.particles.len() {
            let particle = &self.particles[i];

            for j in 0..i {
                let p1 = &self.particles[i];
                let p2 = &self.particles[j];

                let distance = nalgebra::distance(&p1.position, &p2.position);

                if distance < self.threshold {
                    let color = 0.3 - distance / self.threshold * 0.3;

                    mesh_lines.line(
                        &[p1.position, p2.position],
                        (p1.size + p2.size) / 4.0,
                        Color::new(color, color, color, 1.0),
                    )?;
                }
            }

            mesh_circles.circle(
                DrawMode::fill(),
                particle.position,
                particle.size,
                0.1,
                particle.color,
            );
        }

        let mesh_lines = mesh_lines.build(ctx)?;
        graphics::draw(ctx, &mesh_lines, DrawParam::new())?;

        let mesh_circles = mesh_circles.build(ctx)?;
        graphics::draw(ctx, &mesh_circles, DrawParam::new())?;

        Ok(())
    }
}
