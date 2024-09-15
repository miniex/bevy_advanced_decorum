use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct DecorumSettings {
    pub visible_title: bool,
    pub visible_buttons: bool,

    /// Only **`macos`** is supported.
    pub visible_border_radius: bool,
    /// Only **`macos`** is supported.
    pub transparent_titlebar: bool,

    pub control_pad_x: f64,
    pub control_pad_y: f64,
}

impl Default for DecorumSettings {
    fn default() -> Self {
        DecorumSettings {
            visible_title: true,
            visible_buttons: true,
            visible_border_radius: true,
            transparent_titlebar: false,
            control_pad_x: 8.0,
            control_pad_y: 12.0,
        }
    }
}
