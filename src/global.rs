use crate::{
    imgui_wrapper::ImGuiState,
    settings::{Settings, SettingsState},
};

pub struct Global {
    pub settings: Settings,
    pub settings_state: SettingsState,
    pub imgui_state: ImGuiState,
}

impl Global {
    pub fn new() -> Global {
        Global {
            settings: Settings::new(),
            settings_state: SettingsState::default(),
            imgui_state: ImGuiState::default(),
        }
    }
}
