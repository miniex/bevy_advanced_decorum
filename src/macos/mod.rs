mod delegates;
mod position;

use crate::settings::DecorumSettings;
use crate::window::WindowState;
use bevy::prelude::*;
use bevy::window::RawHandleWrapper;
use objc::{msg_send, sel, sel_impl};

pub fn setup_traffic_light(
    _entity: Entity,
    window: &Window,
    raw_handle_wrapper: &RawHandleWrapper,
    settings: &DecorumSettings,
) {
    use cocoa::appkit::NSWindow;

    if let raw_window_handle::RawWindowHandle::AppKit(appkit_handle) =
        raw_handle_wrapper.window_handle
    {
        unsafe {
            let ns_view = appkit_handle.ns_view.as_ptr() as cocoa::base::id;
            let ns_window: cocoa::base::id = msg_send![ns_view, window];

            position::configure_window(ns_window, settings);

            let current_delegate: cocoa::base::id = ns_window.delegate();
            let app_state = WindowState::new(window.clone());
            delegates::setup_window_delegate(ns_window, current_delegate, app_state);
        }
    }
}

pub fn update_traffic_light(
    _entity: Entity,
    _window: &Window,
    raw_handle_wrapper: &RawHandleWrapper,
    settings: &DecorumSettings,
) {
    if let raw_window_handle::RawWindowHandle::AppKit(appkit_handle) =
        raw_handle_wrapper.window_handle
    {
        unsafe {
            let ns_view = appkit_handle.ns_view.as_ptr() as cocoa::base::id;
            let ns_window: cocoa::base::id = msg_send![ns_view, window];
            position::position_traffic_light(
                ns_window,
                settings.control_pad_x,
                settings.control_pad_y,
            );
        }
    }
}
