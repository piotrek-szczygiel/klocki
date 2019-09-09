use std::{path, time::Instant};

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez::{
    conf::WindowMode,
    event, filesystem,
    graphics::{self, Image},
    timer, Context, GameResult,
};

use imgui::{self, im_str, ImStr, ImString, StyleColor};
use imgui_gfx_renderer::{Renderer, Shaders};

use crate::utils;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

#[derive(Copy, Clone)]
pub struct State {
    pub restart: bool,
    pub debug_t_spin_tower: bool,
    pub current_skin_id: usize,
    pub skin_switched: bool,
    pub toggle_fullscreen: bool,
    pub current_scale: f32,
    pub ghost_piece: bool,
    pub block_size: i32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    pub state: State,
    last_frame: Instant,
    mouse_state: MouseState,
    show_debug_window: bool,
    skins: Vec<path::PathBuf>,
    skins_im: Vec<ImString>,
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

        let mut skins: Vec<path::PathBuf> = vec![];
        {
            let skins_dir: Vec<_> = filesystem::read_dir(ctx, utils::path(ctx, "blocks"))
                .unwrap()
                .collect();

            for skin in skins_dir {
                if skin.extension().is_none() {
                    continue;
                }

                if skin.extension().unwrap() != "png" {
                    continue;
                }

                skins.push(skin.clone());
            }
        }

        let skins_im = skins
            .iter()
            .map(|s| ImString::from(String::from(s.file_name().unwrap().to_str().unwrap())))
            .collect();

        ImGuiWrapper {
            imgui,
            renderer,
            state: State {
                restart: false,
                debug_t_spin_tower: false,
                skin_switched: false,
                current_skin_id: 0,
                toggle_fullscreen: false,
                current_scale: graphics::size(ctx).0 / 1920.0,
                ghost_piece: true,
                block_size: 32,
            },
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            show_debug_window: false,
            skins,
            skins_im,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
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
            let skins_im_len = self.skins_im.len() as i32;
            let skins_im: Vec<&ImStr> = self.skins_im.iter().map(|s| s.as_ref()).collect();

            let mut state = self.state;

            if self.show_debug_window {
                ui.window(im_str!("Debug"))
                    .size([300.0, 600.0], imgui::Condition::FirstUseEver)
                    .position([100.0, 100.0], imgui::Condition::FirstUseEver)
                    .build(|| {
                        ui.text(im_str!("Debugging"));
                        ui.separator();

                        if ui.small_button(im_str!("Restart")) {
                            state.restart = true;
                        }

                        if ui.small_button(im_str!("T-Spin tower")) {
                            state.debug_t_spin_tower = true;
                        }
                    });
            }

            ui.main_menu_bar(|| {
                ui.menu(im_str!("File")).build(|| {
                    if ui.menu_item(im_str!("Quit")).build() {
                        event::quit(ctx);
                    }
                });

                ui.menu(im_str!("Settings")).build(|| {
                    ui.text(im_str!("Fullscreen"));
                    if ui.button(im_str!("Toggle fullscreen"), [212.0, 20.0]) {
                        state.toggle_fullscreen = true;
                    }

                    ui.separator();
                    ui.text(im_str!("Window scale"));
                    ui.push_id(0);
                    if ui
                        .slider_float(im_str!(""), &mut state.current_scale, 0.25, 2.0)
                        .build()
                    {
                        graphics::set_mode(
                            ctx,
                            WindowMode::default().dimensions(
                                1920.0 * state.current_scale,
                                1080.0 * state.current_scale,
                            ),
                        )
                        .unwrap_or_else(|e| log::error!("Unable to change resolution: {:?}", e));
                    }
                    ui.pop_id();

                    ui.separator();
                    ui.text(im_str!("Blocks skin"));
                    let mut current_skin_id = state.current_skin_id as i32;
                    ui.push_id(1);
                    if ui.combo(im_str!(""), &mut current_skin_id, &skins_im, skins_im_len) {
                        state.current_skin_id = current_skin_id as usize;
                        state.skin_switched = true;
                    }
                    ui.pop_id();

                    ui.separator();
                    ui.text(im_str!("Block size"));
                    ui.push_id(2);
                    ui.slider_int(im_str!(""), &mut state.block_size, 16, 48)
                        .build();
                    ui.pop_id();

                    ui.separator();
                    ui.text(im_str!("Ghost piece"));
                    ui.checkbox(im_str!("Enabled"), &mut state.ghost_piece);
                });

                ui.separator();
                ui.text(im_str!("FPS:"));

                let fps = timer::fps(ctx) as i32;
                let color = if fps > 55 {
                    [0.0, 1.0, 0.0, 1.0]
                } else {
                    [1.0, 0.0, 0.0, 1.0]
                };

                let _token = ui.push_style_color(StyleColor::Text, color);
                ui.text(ImString::from(fps.to_string()));
            });

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

    pub fn toggle_window(&mut self) {
        self.show_debug_window = !self.show_debug_window;
    }

    pub fn tileset(&self, ctx: &mut Context) -> GameResult<Image> {
        Image::new(
            ctx,
            utils::path(
                ctx,
                self.skins[self.state.current_skin_id].to_str().unwrap(),
            ),
        )
    }
}
