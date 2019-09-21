use crate::{
    imgui_wrapper::ImGuiState,
    settings::{Settings, SettingsState},
    sfx::Sfx,
};

pub struct Global {
    pub settings: Settings,
    pub settings_state: SettingsState,
    pub sfx: Sfx,
    pub imgui_state: ImGuiState,
}

impl Global {
    pub fn new() -> Global {
        Global {
            settings: Settings::new(),
            settings_state: SettingsState::default(),
            sfx: Sfx::default(),
            imgui_state: ImGuiState::default(),
        }
    }
}
