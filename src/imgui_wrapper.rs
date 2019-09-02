use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};

use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
    show_window: bool,
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> Self {
        let mut imgui = imgui::Context::create();
        let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

        let shaders = {
            let version = gfx_device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                Shaders::GlSl130
            } else {
                Shaders::GlSl110
            }
        };

        let renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            show_window: false,
        }
    }

    pub fn render(&mut self, ctx: &mut Context) {
        self.update_mouse();

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let (w, h) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [w, h];
        self.imgui.io_mut().display_framebuffer_scale = [1.0, 1.0];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();

        {
            if self.show_window {
                ui.window(im_str!("Hello world"))
                    .size([300.0, 600.0], imgui::Condition::FirstUseEver)
                    .position([100.0, 100.0], imgui::Condition::FirstUseEver)
                    .build(|| {
                        ui.text(im_str!("Hello world!"));
                        ui.separator();
                        let mouse_pos = ui.io().mouse_pos;
                        ui.text(im_str!(
                            "Mouse Position: ({:.1},{:.1})",
                            mouse_pos[0],
                            mouse_pos[1]
                        ));

                        if ui.small_button(im_str!("small button")) {
                            println!("Small button clicked");
                        }
                    });
            }

            ui.main_menu_bar(|| {
                ui.menu(im_str!("Menu")).build(|| {
                    if ui.menu_item(im_str!("Quit")).build() {
                        ggez::event::quit(ctx);
                    }
                });
            });
        }

        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer
            .render(
                &mut *factory,
                encoder,
                &mut RenderTargetView::new(render_target.clone()),
                draw_data,
            )
            .unwrap();
    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos =
            [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
        self.mouse_state.pressed = pressed;
    }

    pub fn toggle_window(&mut self) {
        self.show_window = !self.show_window;
    }
}
