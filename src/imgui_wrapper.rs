use std::{path, time::Instant};

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez::{
    filesystem,
    graphics::{self, Image},
    Context, GameResult,
};

use imgui::{self, im_str, ImStr, ImString};
use imgui_gfx_renderer::{Renderer, Shaders};

use crate::utils;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct State {
    pub debug_t_spin_tower: bool,
    pub skin_switched: bool,
    pub current_skin_id: usize,
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
    pub fn new(ctx: &mut ggez::Context) -> Self {
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
                println!("- {:?}", skin);

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

        Self {
            imgui,
            renderer,
            state: State {
                debug_t_spin_tower: false,
                skin_switched: false,
                current_skin_id: 0,
            },
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            show_debug_window: false,
            skins,
            skins_im,
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) {
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
            let skins_im_len = self.skins_im.len() as i32;
            let skins_im: Vec<&ImStr> = self.skins_im.iter().map(|s| s.as_ref()).collect();

            let mut state = State {
                debug_t_spin_tower: false,
                skin_switched: false,
                current_skin_id: self.state.current_skin_id,
            };

            if self.show_debug_window {
                ui.window(im_str!("Debug"))
                    .size([300.0, 600.0], imgui::Condition::FirstUseEver)
                    .position([100.0, 100.0], imgui::Condition::FirstUseEver)
                    .build(|| {
                        ui.text(im_str!("Debugging"));
                        ui.separator();

                        if ui.small_button(im_str!("T-Spin tower")) {
                            state.debug_t_spin_tower = true;
                        }
                    });
            }

            ui.main_menu_bar(|| {
                ui.menu(im_str!("File")).build(|| {
                    if ui.menu_item(im_str!("Quit")).build() {
                        ggez::event::quit(ctx);
                    }
                });

                ui.menu(im_str!("Skin")).build(|| {
                    let mut current_skin_id = state.current_skin_id as i32;
                    if ui.list_box(im_str!(""), &mut current_skin_id, &skins_im, skins_im_len) {
                        state.current_skin_id = current_skin_id as usize;
                        state.skin_switched = true;
                    }
                });
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
