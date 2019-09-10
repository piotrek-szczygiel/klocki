use std::time::Instant;

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez::{event, graphics, timer, Context};

use imgui::{self, im_str, ImString, StyleColor};
use imgui_gfx_renderer::{Renderer, Shaders};

use crate::settings::{self, Settings};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

#[derive(Copy, Clone)]
pub struct State {
    pub restart: bool,
    pub paused: bool,
    pub debug_t_spin_tower: bool,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    pub state: State,
    last_frame: Instant,
    mouse_state: MouseState,
    show_debug_window: bool,
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> ImGuiWrapper {
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

        ImGuiWrapper {
            imgui,
            renderer,
            state: State {
                restart: false,
                paused: false,
                debug_t_spin_tower: false,
            },
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            show_debug_window: false,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        settings: &mut Settings,
        settings_state: &mut settings::State,
    ) {
        self.update_mouse();

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let (w, h) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [w, h];
        self.imgui.io_mut().display_framebuffer_scale = [dpi_factor, dpi_factor];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();
        {
            let mut state = State {
                restart: false,
                debug_t_spin_tower: false,
                paused: self.state.paused,
            };

            if self.show_debug_window {
                imgui::Window::new(im_str!("Debug"))
                    .size([300.0, 600.0], imgui::Condition::FirstUseEver)
                    .position([100.0, 100.0], imgui::Condition::FirstUseEver)
                    .build(&ui, || {
                        ui.text(im_str!("Debugging"));
                        ui.separator();

                        if ui.small_button(im_str!("Restart")) {
                            state.restart = true;
                        }

                        ui.checkbox(im_str!("Paused"), &mut state.paused);

                        if ui.small_button(im_str!("T-Spin tower")) {
                            state.debug_t_spin_tower = true;
                        }
                    });
            }

            if let Some(menu_bar) = ui.begin_main_menu_bar() {
                if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                    if imgui::MenuItem::new(im_str!("Quit")).build(&ui) {
                        event::quit(ctx);
                    }

                    menu.end(&ui);
                }

                settings.draw(settings_state, &ui);

                ui.separator();
                ui.text(im_str!("FPS:"));

                let fps = timer::fps(ctx) as i32;
                let color = if fps > 55 {
                    [0.0, 1.0, 0.0, 1.0]
                } else {
                    [1.0, 0.0, 0.0, 1.0]
                };

                let token = ui.push_style_color(StyleColor::Text, color);
                ui.text(ImString::from(fps.to_string()));
                token.pop(&ui);

                menu_bar.end(&ui);
            }

            self.state = state;
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
            .unwrap_or_else(|e| log::error!("Error while rendering ImGui: {:?}", e));
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

    pub fn update_mouse_scroll(&mut self, lines: f32) {
        self.mouse_state.wheel = lines;
    }

    pub fn toggle_window(&mut self) {
        self.show_debug_window = !self.show_debug_window;
    }
}
