use ggez::{
    filesystem,
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

pub fn dt_f32(ctx: &mut Context) -> f32 {
    timer::duration_to_f64(timer::delta(ctx)) as f32
}

pub fn path(ctx: &Context, path: &str) -> String {
    let slash_path = String::from("/") + path;

    if filesystem::is_file(ctx, &slash_path) || filesystem::is_dir(ctx, &slash_path) {
        slash_path
    } else {
        String::from(path)
    }
}
