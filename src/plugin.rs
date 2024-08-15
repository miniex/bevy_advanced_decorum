use bevy::prelude::*;

use crate::decorum::WindowDecorum;

pub struct DecorumPlugin {
    pub primary_window_decorum: Option<WindowDecorum>,
}

impl Default for DecorumPlugin {
    fn default() -> Self {
        DecorumPlugin {
            primary_window_decorum: Some(WindowDecorum::default()),
        }
    }
}

impl DecorumPlugin {
    pub fn new() -> Self {
        DecorumPlugin::default()
    }
}

impl Plugin for DecorumPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "macos")]
        {
            app.add_systems(PreStartup, crate::traffic::setup_traffic_light_positioner);
        }
    }
}
