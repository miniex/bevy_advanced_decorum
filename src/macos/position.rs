use crate::settings::DecorumSettings;
use cocoa::appkit::{
    CGFloat, NSWindow, NSWindowButton, NSWindowStyleMask, NSWindowTitleVisibility,
};
use cocoa::base::{id, nil, BOOL, NO, YES};
use cocoa::foundation::{NSPoint, NSRect};
use objc::{msg_send, sel, sel_impl};

pub fn configure_window(ns_window: id, settings: &DecorumSettings) {
    unsafe {
        ns_window.setTitlebarAppearsTransparent_(settings.transparent_titlebar);

        let mut style_mask = ns_window.styleMask();
        style_mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;

        if !settings.visible_title {
            style_mask |= NSWindowStyleMask::NSTitledWindowMask;
            ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
        } else {
            ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleVisible);
        }

        ns_window.setStyleMask_(style_mask);

        let buttons = [
            NSWindowButton::NSWindowCloseButton,
            NSWindowButton::NSWindowMiniaturizeButton,
            NSWindowButton::NSWindowZoomButton,
        ];

        for &button in &buttons {
            let btn: id = ns_window.standardWindowButton_(button);
            if btn != nil {
                let _: () = msg_send![btn, setHidden: !settings.visible_buttons];
            }
        }

        if settings.visible_border_radius {
            let corner_radius: CGFloat = 10.0;
            let responds_to_corner_radius: BOOL =
                msg_send![ns_window, respondsToSelector:sel!(setCornerRadius:)];
            if responds_to_corner_radius == YES {
                let _: () = msg_send![ns_window, setCornerRadius:corner_radius];
            } else {
                let content_view: id = msg_send![ns_window, contentView];
                let _: () = msg_send![content_view, setWantsLayer: YES];
                let layer: id = msg_send![content_view, layer];
                let _: () = msg_send![layer, setCornerRadius:corner_radius];
                let _: () = msg_send![layer, setMasksToBounds:YES];
            }
        } else {
            let responds_to_corner_radius: BOOL =
                msg_send![ns_window, respondsToSelector:sel!(setCornerRadius:)];
            if responds_to_corner_radius == YES {
                let _: () = msg_send![ns_window, setCornerRadius:0.0];
            } else {
                let content_view: id = msg_send![ns_window, contentView];
                let _: () = msg_send![content_view, setWantsLayer: YES];
                let layer: id = msg_send![content_view, layer];
                let _: () = msg_send![layer, setCornerRadius:0.0];
                let _: () = msg_send![layer, setMasksToBounds:NO];
            }
        }
    }
}

pub fn position_traffic_light(ns_window: id, control_pad_x: f64, control_pad_y: f64) {
    unsafe {
        let close: id = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let miniaturize: id =
            ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let zoom: id = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

        if close != nil && miniaturize != nil && zoom != nil {
            let title_bar_container_view: id = msg_send![close, superview];
            let title_bar_container_view: id = msg_send![title_bar_container_view, superview];

            let close_rect: NSRect = msg_send![close, frame];
            let button_height = close_rect.size.height;

            let title_bar_frame_height = button_height + control_pad_y as CGFloat;
            let mut title_bar_rect: NSRect = msg_send![title_bar_container_view, frame];
            title_bar_rect.size.height = title_bar_frame_height;
            let window_frame: NSRect = msg_send![ns_window, frame];
            title_bar_rect.origin.y = window_frame.size.height - title_bar_frame_height;
            let _: () = msg_send![title_bar_container_view, setFrame:title_bar_rect];

            let window_buttons = [close, miniaturize, zoom];
            let miniaturize_frame: NSRect = msg_send![miniaturize, frame];
            let close_frame: NSRect = msg_send![close, frame];
            let space_between = miniaturize_frame.origin.x - close_frame.origin.x;

            for (i, &button) in window_buttons.iter().enumerate() {
                let mut rect: NSRect = msg_send![button, frame];
                rect.origin.x = control_pad_x as CGFloat + (i as CGFloat * space_between);
                let origin = NSPoint {
                    x: rect.origin.x,
                    y: rect.origin.y,
                };
                let _: () = msg_send![button, setFrameOrigin:origin];
            }
        }
    }
}
