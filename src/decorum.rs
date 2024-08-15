use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct WindowDecorum {
    /// bevy window
    pub window: Option<Window>,
    pub visible_title: bool,
    pub visible_buttons: bool,
    /// Only **`macos`** is supported.
    pub visible_border_radius: bool,
}

impl Default for WindowDecorum {
    fn default() -> Self {
        WindowDecorum {
            window: None,
            visible_title: true,
            visible_buttons: true,
            visible_border_radius: true,
        }
    }
}
