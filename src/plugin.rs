use crate::settings::DecorumSettings;
use bevy::{prelude::*, window::RawHandleWrapper};

pub struct DecorumPlugin {
    settings: DecorumSettings,
}

impl Default for DecorumPlugin {
    fn default() -> Self {
        DecorumPlugin {
            settings: DecorumSettings::default(),
        }
    }
}

impl DecorumPlugin {
    pub fn new(settings: DecorumSettings) -> Self {
        DecorumPlugin { settings }
    }
}

impl Plugin for DecorumPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone());

        app.add_systems(PreStartup, setup_positioner)
            .add_systems(PreUpdate, update_positioner);
    }
}

fn setup_positioner(
    raw_handle_wrapper_query: Query<(Entity, &Window, &RawHandleWrapper)>,
    settings: Res<DecorumSettings>,
) {
    for (entity, window, raw_handle_wrapper) in raw_handle_wrapper_query.iter() {
        #[cfg(target_os = "macos")]
        crate::macos::setup_traffic_light(entity, window, raw_handle_wrapper, &settings);

        #[cfg(target_os = "windows")]
        crate::windows::setup_window_decorations(entity, window, raw_handle_wrapper, &settings);

        #[cfg(target_os = "linux")]
        crate::linux::setup_window_decorations(entity, window, raw_handle_wrapper, &settings);
    }
}

fn update_positioner(
    raw_handle_wrapper_query: Query<(Entity, &Window, &RawHandleWrapper)>,
    settings: Res<DecorumSettings>,
) {
    for (entity, window, raw_handle_wrapper) in raw_handle_wrapper_query.iter() {
        #[cfg(target_os = "macos")]
        crate::macos::update_traffic_light(entity, window, raw_handle_wrapper, &settings);

        #[cfg(target_os = "windows")]
        crate::windows::update_window_decorations(entity, window, raw_handle_wrapper);

        #[cfg(target_os = "linux")]
        crate::linux::update_window_decorations(entity, window, raw_handle_wrapper);
    }
}
