use ggez::{
    graphics::{self, Rect},
    input,
    nalgebra::Point2,
    timer, Context,
};

pub fn mouse_position_coords(ctx: &mut Context) -> Point2<f32> {
    let (w, h) = graphics::size(ctx);
    let Rect { w: cw, h: ch, .. } = graphics::screen_coordinates(ctx);
    let pos = input::mouse::position(ctx);

    Point2::new(pos.x * cw / w, pos.y * ch / h)
}

pub fn dt(ctx: &mut Context) -> f32 {
    timer::duration_to_f64(timer::delta(ctx)) as f32
}
