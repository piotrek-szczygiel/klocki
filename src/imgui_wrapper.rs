use std::{
    io::Read,
    time::{Duration, Instant},
};

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez::{event, filesystem, graphics, timer, Context};
use imgui::{self, im_str, FontId, FontSource, ImString, StyleColor, Window};
use imgui_gfx_renderer::{Renderer, Shaders};

use crate::{global::Global, utils};

#[derive(Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

#[derive(Default)]
pub struct ImGuiState {
    pub paused: bool,
    pub restart: bool,
    pub game_over: bool,
    pub debug_t_spin_tower: bool,
    pub update_last: Duration,
    pub draw_last: Duration,
    pub update: Vec<Duration>,
    pub draw: Vec<Duration>,
    pub game_over_window: bool,
    pub save_replay: bool,
    pub replay_score: i32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    regular_font: FontId,
    bold_font: FontId,
    last_frame: Instant,
    mouse_state: MouseState,
    show_debug_window: bool,
}

impl ImGuiWrapper {
    fn load_font(imgui: &mut imgui::Context, ctx: &mut Context) -> (FontId, FontId) {
        let regular = filesystem::open(ctx, utils::path(ctx, "fonts/regular.ttf"));
        let bold = filesystem::open(ctx, utils::path(ctx, "fonts/bold.ttf"));

        let mut regular_font = None;
        let mut bold_font = None;

        if let (Ok(mut regular), Ok(mut bold)) = (regular, bold) {
            let mut regular_bytes = vec![];
            if regular.read_to_end(&mut regular_bytes).is_ok() {
                regular_font = Some(imgui.fonts().add_font(&[FontSource::TtfData {
                    data: &regular_bytes,
                    size_pixels: 16.0,
                    config: None,
                }]));
            }

            let mut bold_bytes = vec![];
            if bold.read_to_end(&mut bold_bytes).is_ok() {
                bold_font = Some(imgui.fonts().add_font(&[FontSource::TtfData {
                    data: &bold_bytes,
                    size_pixels: 16.0,
                    config: None,
                }]));
            }
        }

        if regular_font.is_none() || bold_font.is_none() {
            log::error!("Unable to load fonts");
        }

        (regular_font.unwrap(), bold_font.unwrap())
    }

    pub fn new(ctx: &mut Context) -> ImGuiWrapper {
        let mut imgui = imgui::Context::create();

        let (regular_font, bold_font) = ImGuiWrapper::load_font(&mut imgui, ctx);
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

        let mut renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();
        renderer
            .reload_font_texture(&mut imgui, &mut *factory)
            .expect("Failed to reload fonts");

        let style = imgui.style_mut();
        style.window_padding = [15.0, 15.0];
        style.window_rounding = 5.0;
        style.frame_padding = [5.0, 5.0];
        style.frame_rounding = 4.0;
        style.item_spacing = [12.0, 8.0];
        style.item_inner_spacing = [8.0, 6.0];
        style.indent_spacing = 25.0;
        style.scrollbar_size = 15.0;
        style.scrollbar_rounding = 9.0;
        style.grab_min_size = 5.0;
        style.grab_rounding = 3.0;

        use imgui::StyleColor::*;
        style[Text] = [0.80, 0.80, 0.83, 1.00];
        style[TextDisabled] = [0.24, 0.23, 0.29, 1.00];
        style[WindowBg] = [0.06, 0.05, 0.07, 1.00];
        style[ChildBg] = [0.07, 0.07, 0.09, 1.00];
        style[PopupBg] = [0.07, 0.07, 0.09, 1.00];
        style[Border] = [0.80, 0.80, 0.83, 0.88];
        style[BorderShadow] = [0.92, 0.91, 0.88, 0.00];
        style[FrameBg] = [0.10, 0.09, 0.12, 1.00];
        style[FrameBgHovered] = [0.24, 0.23, 0.29, 1.00];
        style[FrameBgActive] = [0.56, 0.56, 0.58, 1.00];
        style[TitleBg] = [0.10, 0.09, 0.12, 1.00];
        style[TitleBgCollapsed] = [1.00, 0.98, 0.95, 0.75];
        style[TitleBgActive] = [0.07, 0.07, 0.09, 1.00];
        style[MenuBarBg] = [0.10, 0.09, 0.12, 1.00];
        style[ScrollbarBg] = [0.10, 0.09, 0.12, 1.00];
        style[ScrollbarGrab] = [0.80, 0.80, 0.83, 0.31];
        style[ScrollbarGrabHovered] = [0.56, 0.56, 0.58, 1.00];
        style[ScrollbarGrabActive] = [0.06, 0.05, 0.07, 1.00];
        style[CheckMark] = [0.80, 0.80, 0.83, 0.31];
        style[SliderGrab] = [0.80, 0.80, 0.83, 0.31];
        style[SliderGrabActive] = [0.06, 0.05, 0.07, 1.00];
        style[Button] = [0.10, 0.09, 0.12, 1.00];
        style[ButtonHovered] = [0.24, 0.23, 0.29, 1.00];
        style[ButtonActive] = [0.56, 0.56, 0.58, 1.00];
        style[Header] = [0.10, 0.09, 0.12, 1.00];
        style[HeaderHovered] = [0.56, 0.56, 0.58, 1.00];
        style[HeaderActive] = [0.06, 0.05, 0.07, 1.00];
        style[ResizeGrip] = [0.00, 0.00, 0.00, 0.00];
        style[ResizeGripHovered] = [0.56, 0.56, 0.58, 1.00];
        style[ResizeGripActive] = [0.06, 0.05, 0.07, 1.00];
        style[PlotLines] = [0.40, 0.39, 0.38, 0.63];
        style[PlotLinesHovered] = [0.25, 1.00, 0.00, 1.00];
        style[PlotHistogram] = [0.40, 0.39, 0.38, 0.63];
        style[PlotHistogramHovered] = [0.25, 1.00, 0.00, 1.00];
        style[TextSelectedBg] = [0.25, 1.00, 0.00, 0.43];
        style[ModalWindowDimBg] = [1.00, 0.98, 0.95, 0.73];

        ImGuiWrapper {
            imgui,
            renderer,
            regular_font,
            bold_font,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            show_debug_window: false,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, g: &mut Global) {
        self.update_mouse();

        const AVG_FRAMES: usize = 30;

        if g.imgui_state.update.len() == AVG_FRAMES {
            g.imgui_state.update_last = g.imgui_state.update.iter().sum();
            g.imgui_state.update.clear();
        }

        if g.imgui_state.draw.len() == AVG_FRAMES {
            g.imgui_state.draw_last = g.imgui_state.draw.iter().sum();
            g.imgui_state.draw.clear();
        }

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
            let font_id = ui.push_font(self.regular_font);
            if self.show_debug_window {
                Window::new(im_str!("Debug"))
                    // .size([300.0, 400.0], Condition::Appearing)
                    // .position([50.0, 50.0], Condition::Appearing)
                    .build(&ui, || {
                        ui.text(im_str!("Debugging window"));
                        ui.separator();

                        ui.checkbox(im_str!("Paused"), &mut g.imgui_state.paused);

                        g.imgui_state.restart = ui.button(im_str!("Restart"), [0.0, 0.0]);

                        g.imgui_state.game_over = ui.button(im_str!("Game over"), [0.0, 0.0]);

                        g.imgui_state.debug_t_spin_tower =
                            ui.button(im_str!("T-Spin tower"), [0.0, 0.0]);

                        ui.separator();
                        ui.text(im_str!("Window size: {}x{}", w, h));

                        ui.separator();
                        ui.text(im_str!("Delta:  {:.1?}", timer::average_delta(ctx)));
                        ui.text(im_str!(
                            "Update: {:.1?}",
                            g.imgui_state.update_last / AVG_FRAMES as u32
                        ));
                        ui.text(im_str!(
                            "Draw:   {:.1?}",
                            g.imgui_state.draw_last / AVG_FRAMES as u32
                        ));
                    });
            }

            if g.imgui_state.game_over_window {
                let mut opened = true;
                Window::new(im_str!("Game over"))
                    .opened(&mut opened)
                    .resizable(false)
                    .collapsible(false)
                    .build(&ui, || {
                        ui.text(im_str!("Score: {}", g.imgui_state.replay_score));
                        ui.separator();

                        g.imgui_state.save_replay = ui.button(im_str!("Save replay"), [0.0, 0.0]);

                        ui.separator();
                        if g.imgui_state.save_replay || ui.button(im_str!("Close"), [0.0, 0.0]) {
                            g.imgui_state.game_over_window = false;
                        }
                    });

                if !opened {
                    g.imgui_state.game_over_window = false;
                }
            }

            if !g.settings.graphics.hide_menu {
                if let Some(menu_bar) = ui.begin_main_menu_bar() {
                    if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                        if imgui::MenuItem::new(im_str!("Quit")).build(&ui) {
                            event::quit(ctx);
                        }

                        menu.end(&ui);
                    }

                    g.settings.draw(&mut g.settings_state, &ui, self.bold_font);

                    ui.separator();
                    ui.text(im_str!("FPS:"));

                    let fps = timer::fps(ctx) as i32;
                    let color = if fps > 58 {
                        [0.0, 1.0, 0.0, 1.0]
                    } else {
                        [1.0, 0.0, 0.0, 1.0]
                    };

                    let token = ui.push_style_color(StyleColor::Text, color);
                    ui.text(ImString::from(fps.to_string()));
                    token.pop(&ui);

                    menu_bar.end(&ui);
                }
            }

            font_id.pop(&ui);
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
