use crate::settings::DecorumSettings;
use bevy::prelude::*;

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
    pub fn new() -> Self {
        DecorumPlugin::default()
    }

    pub fn with_settings(settings: DecorumSettings) -> Self {
        DecorumPlugin { settings }
    }
}

impl Plugin for DecorumPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "macos")]
        {
            app.insert_resource(self.settings.clone());
            app.add_systems(PreStartup, crate::traffic::setup_traffic_light_positioner);
            app.add_systems(PreUpdate, crate::traffic::update_traffic_light_positioner);
        }
    }
}
