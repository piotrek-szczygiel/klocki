use std::time::Duration;

use ggez::{
    conf::NumSamples,
    graphics::{self, Align, Canvas, Color, DrawParam, Font, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};

use crate::utils;

#[derive(Default)]
pub struct Popup {
    canvas: Option<Canvas>,
    fragments: Vec<PopupFragment>,
    visible: Duration,
    lifetime: Duration,
    height: u32,
    alpha: f32,
    hide: f32,
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

    pub fn finish(
        &mut self,
        ctx: &mut Context,
        font: Font,
        width: f32,
        height: f32,
        scale: f32,
    ) -> GameResult {
        let mut text = Text::default();
        text.set_font(font, Scale::uniform(scale))
            .set_bounds(Point2::new(width, height), Align::Center);

        let mut shadow = Text::default();
        shadow
            .set_font(font, Scale::uniform(scale))
            .set_bounds(Point2::new(width, height), Align::Center);

        for f in self.fragments.drain(..) {
            text.add(
                TextFragment::from(f.text.clone())
                    .color(f.color)
                    .scale(Scale::uniform(scale * f.scale)),
            );

            shadow.add(
                TextFragment::from(f.text)
                    .color(graphics::BLACK)
                    .scale(Scale::uniform(scale * f.scale)),
            );
        }

        self.height = text.height(ctx);

        let screen = graphics::screen_coordinates(ctx);

        let shadow_offset = scale / 8.0;

        let canvas = Some(Canvas::new(
            ctx,
            (screen.w + shadow_offset) as u16,
            (screen.h + shadow_offset) as u16,
            NumSamples::One,
        )?);

        graphics::set_canvas(ctx, canvas.as_ref());
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 0.0));
        graphics::draw(
            ctx,
            &shadow,
            DrawParam::default().dest(Point2::new(shadow_offset, shadow_offset)),
        )?;
        graphics::draw(ctx, &text, DrawParam::default())?;
        graphics::set_canvas(ctx, None);

        self.canvas = canvas;

        Ok(())
    }

    pub fn update(&mut self, dt: Duration) -> bool {
        self.visible += dt;

        let ratio =
            (timer::duration_to_f64(self.visible) / timer::duration_to_f64(self.lifetime)) as f32;

        self.alpha = 1.0;
        self.hide = 0.0;
        if ratio >= 0.5 {
            self.alpha = 1.0 - (ratio - 0.5) * 6.0;
            if self.alpha < 0.0 {
                self.alpha = 0.0;
            }

            self.hide = (ratio - 0.5) * 2.0;

            true
        } else {
            false
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, position: Point2<f32>, height: f32) -> GameResult {
        if let Some(canvas) = &self.canvas {
            graphics::draw(
                ctx,
                canvas,
                DrawParam::new()
                    .dest(
                        position
                            + Vector2::new(
                                0.0,
                                (height - self.height as f32) / 2.0 - self.hide * height * 2.0,
                            ),
                    )
                    .color(Color::new(1.0, 1.0, 1.0, self.alpha)),
            )?;
        }

        Ok(())
    }
}

pub struct Popups {
    active_popup: Option<Popup>,
    fading_popups: Vec<Popup>,
    font: Font,
    just_created: bool,
}

impl Popups {
    pub fn new(ctx: &mut Context) -> GameResult<Popups> {
        Ok(Popups {
            active_popup: None,
            fading_popups: vec![],
            font: Font::new(ctx, utils::path(ctx, "fonts/bold.ttf"))?,
            just_created: true,
        })
    }

    pub fn add(&mut self, popup: Popup) {
        let mut popup = Some(popup);
        std::mem::swap(&mut popup, &mut self.active_popup);

        if let Some(mut popup) = popup {
            if popup.hide < 0.5 {
                popup.visible = popup.lifetime / 2;
            }
            self.fading_popups.push(popup);
        }
    }

    pub fn lock(&mut self, rows: i32, t_spin: bool, btb: bool, combo: Option<i32>, delay: u64) {
        let mut lifetime = delay;
        if lifetime < 500 {
            lifetime = 500;
        }

        let mut popup = Popup::new(Duration::from_millis(lifetime * 2));

        if t_spin {
            popup.add("T-Spin\n", Color::new(1.0, 0.5, 0.9, 1.0), 4.0);

            match rows {
                1 => popup.add("Single\n", Color::new(0.8, 0.9, 1.0, 1.0), 2.0),
                2 => popup.add("Double\n", Color::new(0.5, 0.9, 0.7, 1.0), 2.0),
                3 => popup.add("Triple\n", Color::new(0.2, 0.9, 0.3, 1.0), 2.0),
                _ => (),
            }
        }

        if rows == 4 {
            popup.add("Tetris\n", Color::new(0.5, 0.8, 1.0, 1.0), 4.0);
        }

        if btb {
            popup.add("Back-to-Back\n", Color::new(0.8, 0.9, 1.0, 1.0), 1.5);
        }

        if let Some(combo) = combo {
            if combo > 0 {
                let mut rank = combo as f32 / 10.0;
                if rank > 1.0 {
                    rank = 1.0;
                }

                popup.add(
                    &format!("{} combo\n", combo),
                    Color::new(1.0, 1.0, 1.0 - rank, 1.0),
                    rank + 1.0,
                );
            }
        }

        self.add(popup);
    }

    pub fn update(&mut self, ctx: &mut Context, width: f32, height: f32, scale: f32) -> GameResult {
        let dt = timer::delta(ctx);

        if self.just_created {
            self.just_created = false;
            return Ok(());
        }

        let mut fading = false;
        if let Some(p) = self.active_popup.as_mut() {
            if p.canvas.is_none() {
                p.finish(ctx, self.font, width, height, scale)?;
            }

            fading = p.update(dt);

            if p.visible >= p.lifetime {
                self.active_popup = None;
            }
        }

        if fading {
            let mut p = None;
            std::mem::swap(&mut p, &mut self.active_popup);
            self.fading_popups.push(p.unwrap());
        }

        for p in self.fading_popups.iter_mut() {
            p.update(dt);
        }

        self.fading_popups.retain(|p| p.visible < p.lifetime);

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, position: Point2<f32>, height: f32) -> GameResult {
        if let Some(p) = self.active_popup.as_mut() {
            p.draw(ctx, position, height)?;
        }

        if let Some(p) = self.active_popup.as_mut() {
            p.draw(ctx, position, height)?;
        }

        for p in self.fading_popups.iter_mut() {
            p.draw(ctx, position, height)?;
        }

        Ok(())
    }
}
