use std::time::Duration;

use ggez::{
    graphics::{self, Align, Color, DrawParam, Font, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};

use crate::utils;

#[derive(Default)]
struct Popup {
    text: Text,
    fragments: Vec<PopupFragment>,
    visible: Duration,
    lifetime: Duration,
}

struct PopupFragment {
    text: String,
    color: Color,
    scale: f32,
}

impl Popup {
    pub fn new(lifetime: Duration) -> Popup {
        Popup {
            lifetime,
            ..Default::default()
        }
    }

    pub fn add(&mut self, text: &str, color: Color, scale: f32) {
        self.fragments.push(PopupFragment {
            text: String::from(text),
            color,
            scale,
        });
    }

    pub fn finish(&mut self, font: Font, width: f32, height: f32, scale: f32) {
        self.text
            .set_font(font, Scale::uniform(scale))
            .set_bounds(Point2::new(width, height), Align::Center);

        for f in self.fragments.drain(..) {
            self.text.add(
                TextFragment::from(f.text)
                    .color(f.color)
                    .scale(Scale::uniform(scale * f.scale)),
            );
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.visible += dt;

        let ratio =
            (timer::duration_to_f64(self.visible) / timer::duration_to_f64(self.lifetime)) as f32;

        let mut alpha = 1.0;
        if ratio >= 0.5 {
            alpha = 1.0 - (ratio - 0.5) * 2.0;
        }

        for f in self.text.fragments_mut().iter_mut() {
            let mut color = f.color.unwrap_or(graphics::WHITE);
            color.a = alpha;
            f.color = Some(color);
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        position: Point2<f32>,
        height: f32,
    ) -> GameResult<bool> {
        let mut fading = false;

        let mut hide = 0.0;
        let ratio =
            (timer::duration_to_f64(self.visible) / timer::duration_to_f64(self.lifetime)) as f32;

        if ratio >= 0.5 {
            hide = height * (ratio - 0.5) * 4.0;
            fading = true;
        }

        let position =
            position + Vector2::new(0.0, (height - self.text.height(ctx) as f32) / 2.0 - hide);
        graphics::draw(ctx, &self.text, DrawParam::new().dest(position))?;

        Ok(fading)
    }
}

#[derive(Default)]
pub struct Popups {
    active_popup: Option<Popup>,
    fading_popups: Vec<Popup>,
    font: Font,
}

impl Popups {
    pub fn new(ctx: &mut Context) -> GameResult<Popups> {
        Ok(Popups {
            font: Font::new(ctx, utils::path(ctx, "fonts/popup.ttf"))?,
            ..Default::default()
        })
    }

    pub fn lock(
        &mut self,
        rows: i32,
        t_spin: bool,
        btb: bool,
        combo: Option<i32>,
        width: f32,
        height: f32,
        scale: f32,
    ) {
        let mut popup = Popup::new(Duration::from_millis(1000));

        if t_spin {
            popup.add("T-Spin\n", Color::new(1.0, 0.5, 0.9, 1.0), 3.0);

            match rows {
                1 => popup.add("Single\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0),
                2 => popup.add("Double\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0),
                3 => popup.add("Triple\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0),
                _ => (),
            }
        }

        if rows == 4 {
            popup.add("Tetris\n", Color::new(0.5, 0.8, 1.0, 1.0), 3.0);
        }

        if btb {
            popup.add("Back-to-Back\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.0);
        }

        if let Some(combo) = combo {
            if combo > 0 {
                let mut rank = combo as f32 / 20.0;
                if rank > 1.0 {
                    rank = 1.0;
                }

                popup.add(
                    &format!("{} combo\n", combo),
                    Color::new(1.0 - rank, 1.0, 1.0, 1.0),
                    rank + 1.0,
                );
            }
        }

        popup.finish(self.font, width, height, scale);
        self.active_popup = Some(popup);
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let dt = timer::delta(ctx);

        if let Some(p) = self.active_popup.as_mut() {
            p.update(dt);
            if p.visible >= p.lifetime {
                self.active_popup = None;
            }
        }

        for p in self.fading_popups.iter_mut() {
            p.update(dt);
        }

        self.fading_popups.retain(|p| p.visible < p.lifetime);
    }

    pub fn draw(&mut self, ctx: &mut Context, position: Point2<f32>, height: f32) -> GameResult {
        let mut fading = false;
        if let Some(p) = self.active_popup.as_mut() {
            fading = p.draw(ctx, position, height)?;
        }

        if let Some(p) = self.active_popup.as_mut() {
            p.draw(ctx, position, height)?;
        }

        if fading {
            let mut p = None;
            std::mem::swap(&mut p, &mut self.active_popup);
            self.fading_popups.push(p.unwrap());
        }

        for p in self.fading_popups.iter_mut() {
            p.draw(ctx, position, height)?;
        }

        Ok(())
    }
}
