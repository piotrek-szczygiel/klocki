use std::time::Duration;

use ggez::{
    graphics::{self, Align, Color, DrawParam, Font, Scale, Text, TextFragment},
    nalgebra::Point2,
    timer, Context, GameResult,
};

struct Popup {
    text: TextFragment,
    visible: Duration,
    lifetime: Duration,
    size: f32,
}

impl Popup {
    pub fn new(text: &str, color: Color, size: f32) -> Popup {
        Popup {
            text: TextFragment::from(text).color(color),
            visible: Duration::new(0, 0),
            lifetime: Duration::from_millis(1500),
            size,
        }
    }
}

#[derive(Default)]
pub struct Popups {
    popups: Vec<Popup>,
}

impl Popups {
    pub fn lock(&mut self, rows: i32, t_spin: bool, btb: bool, combo: Option<i32>) {
        if t_spin {
            self.popups
                .push(Popup::new("T-Spin\n", Color::new(1.0, 0.5, 0.8, 1.0), 2.0));

            match rows {
                1 => self
                    .popups
                    .push(Popup::new("Single\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0)),
                2 => self
                    .popups
                    .push(Popup::new("Double\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0)),
                3 => self
                    .popups
                    .push(Popup::new("Triple\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0)),
                _ => (),
            }
        }

        if rows == 4 {
            self.popups
                .push(Popup::new("Tetris\n", Color::new(0.5, 0.8, 1.0, 1.0), 2.0));
        }

        if btb {
            self.popups.push(Popup::new(
                "Back-to-Back\n",
                Color::new(0.8, 0.9, 1.0, 1.0),
                1.5,
            ));
        }

        if let Some(combo) = combo {
            if combo > 0 {
                let mut rank = combo as f32 / 20.0;
                if rank > 1.0 {
                    rank = 1.0;
                }

                self.popups.push(Popup::new(
                    &format!("{} combo\n", combo),
                    Color::new(1.0 - rank, 1.0, 1.0, 1.0),
                    rank + 1.0,
                ));
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let dt = timer::delta(ctx);

        self.popups.iter_mut().for_each(|p| {
            p.visible += dt;

            if let Some(color) = &mut p.text.color {
                color.a = (1.0
                    - timer::duration_to_f64(p.visible) / timer::duration_to_f64(p.lifetime))
                    as f32;
            }
        });

        self.popups.retain(|p| p.visible < p.lifetime);
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        font: Font,
        block_size: f32,
    ) -> GameResult {
        let mut popups = Text::default();
        popups
            .set_font(font, Scale::uniform(block_size))
            .set_bounds(
                Point2::new(block_size * 10.0, std::f32::INFINITY),
                Align::Center,
            );

        for p in &self.popups {
            let t = p.text.clone().scale(Scale::uniform(block_size * p.size));
            popups.add(t);
        }

        graphics::draw(ctx, &popups, DrawParam::new().dest(position))?;

        Ok(())
    }
}
