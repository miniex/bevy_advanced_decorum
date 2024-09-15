use crate::settings::DecorumSettings;
use bevy::prelude::*;
use bevy::window::RawHandleWrapper;
use objc::{msg_send, sel, sel_impl};
use rand::{distributions::Alphanumeric, Rng};

const WINDOW_CONTROL_PAD_X: f64 = 8.0;
const WINDOW_CONTROL_PAD_Y: f64 = 12.0;

pub struct UnsafeWindowHandle(pub *mut std::ffi::c_void);
unsafe impl Send for UnsafeWindowHandle {}
unsafe impl Sync for UnsafeWindowHandle {}

#[cfg(target_os = "macos")]
pub fn position_traffic_light(ns_window_handle: UnsafeWindowHandle, x: f64, y: f64) {
    use cocoa::appkit::{NSView, NSWindow, NSWindowButton};
    use cocoa::base::id;
    use cocoa::foundation::NSRect;

    let ns_window = ns_window_handle.0 as id;

    unsafe {
        let close = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let miniaturize =
            ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let zoom = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

        let title_bar_container_view = close.superview().superview();

        let close_rect: NSRect = msg_send![close, frame];
        let button_height = close_rect.size.height;

        let title_bar_frame_height = button_height + y;
        let mut title_bar_rect = NSView::frame(title_bar_container_view);
        title_bar_rect.size.height = title_bar_frame_height;
        title_bar_rect.origin.y = NSView::frame(ns_window).size.height - title_bar_frame_height;
        let _: () = msg_send![title_bar_container_view, setFrame: title_bar_rect];

        let window_buttons = vec![close, miniaturize, zoom];
        let space_between = NSView::frame(miniaturize).origin.x - NSView::frame(close).origin.x;

        for (i, button) in window_buttons.into_iter().enumerate() {
            let mut rect: NSRect = NSView::frame(button);
            rect.origin.x = x + (i as f64 * space_between);
            button.setFrameOrigin(rect.origin);
        }
    }
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
#[derive(Debug)]
struct WindowState {
    window: Window,
}

#[cfg(target_os = "macos")]
pub fn setup_traffic_light_positioner(
    raw_handle_wrapper_query: Query<(Entity, &Window, &RawHandleWrapper)>,
    settings: Res<DecorumSettings>,
) {
    use cocoa::appkit::NSWindow;
    use cocoa::base::{id, BOOL};
    use cocoa::foundation::NSUInteger;
    use objc::runtime::{Object, Sel};
    use raw_window_handle::RawWindowHandle;
    use std::ffi::c_void;

    for (_entity, window, raw_handle_wrapper) in raw_handle_wrapper_query.iter() {
        if let RawWindowHandle::AppKit(appkit_handle) = raw_handle_wrapper.window_handle {
            let ns_view_ptr: *mut c_void = appkit_handle.ns_view.as_ptr();

            unsafe {
                let ns_view: id = ns_view_ptr as id;
                let ns_window_ptr = msg_send![ns_view, window];
                let ns_window = ns_window_ptr as id;

                ns_window.setTitlebarAppearsTransparent_(settings.transparent_titlebar);
                ns_window.setStyleMask_(
                    ns_window.styleMask()
                        | cocoa::appkit::NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                );

                position_traffic_light(
                    UnsafeWindowHandle(ns_window_ptr),
                    WINDOW_CONTROL_PAD_X,
                    WINDOW_CONTROL_PAD_Y,
                );

                let current_delegate: id = ns_window.delegate();

                extern "C" fn on_window_should_close(this: &Object, _cmd: Sel, sender: id) -> BOOL {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        msg_send![super_del, windowShouldClose: sender]
                    }
                }
                extern "C" fn on_window_will_close(this: &Object, _cmd: Sel, notification: id) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowWillClose: notification];
                    }
                }
                extern "C" fn on_window_did_resize(this: &Object, _cmd: Sel, notification: id) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidResize: notification];
                    }
                }
                extern "C" fn on_window_did_move(this: &Object, _cmd: Sel, notification: id) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidMove: notification];
                    }
                }
                extern "C" fn on_window_did_change_backing_properties(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () =
                            msg_send![super_del, windowDidChangeBackingProperties: notification];
                    }
                }
                extern "C" fn on_window_did_become_key(this: &Object, _cmd: Sel, notification: id) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidBecomeKey: notification];
                    }
                }
                extern "C" fn on_window_did_resign_key(this: &Object, _cmd: Sel, notification: id) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidResignKey: notification];
                    }
                }
                extern "C" fn on_dragging_entered(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) -> BOOL {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        msg_send![super_del, draggingEntered: notification]
                    }
                }
                extern "C" fn on_prepare_for_drag_operation(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) -> BOOL {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        msg_send![super_del, prepareForDragOperation: notification]
                    }
                }
                extern "C" fn on_perform_drag_operation(
                    this: &Object,
                    _cmd: Sel,
                    sender: id,
                ) -> BOOL {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        msg_send![super_del, performDragOperation: sender]
                    }
                }
                extern "C" fn on_conclude_drag_operation(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, concludeDragOperation: notification];
                    }
                }
                extern "C" fn on_dragging_exited(this: &Object, _cmd: Sel, notification: id) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, draggingExited: notification];
                    }
                }
                extern "C" fn on_window_will_use_full_screen_presentation_options(
                    this: &Object,
                    _cmd: Sel,
                    window: id,
                    proposed_options: NSUInteger,
                ) -> NSUInteger {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        msg_send![super_del, window: window willUseFullScreenPresentationOptions: proposed_options]
                    }
                }
                extern "C" fn on_window_did_enter_full_screen(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidEnterFullScreen: notification];
                    }
                }
                extern "C" fn on_window_will_enter_full_screen(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowWillEnterFullScreen: notification];
                    }
                }
                extern "C" fn on_window_did_exit_full_screen(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidExitFullScreen: notification];
                    }
                }
                extern "C" fn on_window_will_exit_full_screen(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowWillExitFullScreen: notification];
                    }
                }
                extern "C" fn on_window_did_fail_to_enter_full_screen(
                    this: &Object,
                    _cmd: Sel,
                    window: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![super_del, windowDidFailToEnterFullScreen: window];
                    }
                }
                extern "C" fn on_effective_appearance_did_change(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () =
                            msg_send![super_del, effectiveAppearanceDidChange: notification];
                    }
                }
                extern "C" fn on_effective_appearance_did_changed_on_main_thread(
                    this: &Object,
                    _cmd: Sel,
                    notification: id,
                ) {
                    unsafe {
                        let super_del: id = *this.get_ivar("super_delegate");
                        let _: () = msg_send![
                            super_del,
                            effectiveAppearanceDidChangedOnMainThread: notification
                        ];
                    }
                }

                let app_state = WindowState {
                    window: window.clone(),
                };
                let app_box = Box::into_raw(Box::new(app_state)) as *mut c_void;

                let random_str: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(20)
                    .map(char::from)
                    .collect();

                let delegate_name = format!("windowDelegate_{}", random_str);

                ns_window.setDelegate_(cocoa::delegate!(&delegate_name, {
                    window: id = ns_window,
                    app_box: *mut c_void = app_box,
                    toolbar: id = cocoa::base::nil,
                    super_delegate: id = current_delegate,
                    (windowShouldClose:) => on_window_should_close as extern fn(&Object, Sel, id) -> BOOL,
                    (windowWillClose:) => on_window_will_close as extern fn(&Object, Sel, id),
                    (windowDidResize:) => on_window_did_resize as extern fn(&Object, Sel, id),
                    (windowDidMove:) => on_window_did_move as extern fn(&Object, Sel, id),
                    (windowDidChangeBackingProperties:) => on_window_did_change_backing_properties as extern fn(&Object, Sel, id),
                    (windowDidBecomeKey:) => on_window_did_become_key as extern fn(&Object, Sel, id),
                    (windowDidResignKey:) => on_window_did_resign_key as extern fn(&Object, Sel, id),
                    (draggingEntered:) => on_dragging_entered as extern fn(&Object, Sel, id) -> BOOL,
                    (prepareForDragOperation:) => on_prepare_for_drag_operation as extern fn(&Object, Sel, id) -> BOOL,
                    (performDragOperation:) => on_perform_drag_operation as extern fn(&Object, Sel, id) -> BOOL,
                    (concludeDragOperation:) => on_conclude_drag_operation as extern fn(&Object, Sel, id),
                    (draggingExited:) => on_dragging_exited as extern fn(&Object, Sel, id),
                    (window:willUseFullScreenPresentationOptions:) => on_window_will_use_full_screen_presentation_options as extern fn(&Object, Sel, id, NSUInteger) -> NSUInteger,
                    (windowDidEnterFullScreen:) => on_window_did_enter_full_screen as extern fn(&Object, Sel, id),
                    (windowWillEnterFullScreen:) => on_window_will_enter_full_screen as extern fn(&Object, Sel, id),
                    (windowDidExitFullScreen:) => on_window_did_exit_full_screen as extern fn(&Object, Sel, id),
                    (windowWillExitFullScreen:) => on_window_will_exit_full_screen as extern fn(&Object, Sel, id),
                    (windowDidFailToEnterFullScreen:) => on_window_did_fail_to_enter_full_screen as extern fn(&Object, Sel, id),
                    (effectiveAppearanceDidChange:) => on_effective_appearance_did_change as extern fn(&Object, Sel, id),
                    (effectiveAppearanceDidChangedOnMainThread:) => on_effective_appearance_did_changed_on_main_thread as extern fn(&Object, Sel, id)
                }))
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn update_traffic_light_positioner(
    raw_handle_wrapper_query: Query<(Entity, &Window, &RawHandleWrapper)>,
) {
    use cocoa::base::id;
    use raw_window_handle::RawWindowHandle;
    use std::ffi::c_void;

    for (_entity, _window, raw_handle_wrapper) in raw_handle_wrapper_query.iter() {
        if let RawWindowHandle::AppKit(appkit_handle) = raw_handle_wrapper.window_handle {
            let ns_view_ptr: *mut c_void = appkit_handle.ns_view.as_ptr();

            unsafe {
                let ns_view: id = ns_view_ptr as id;
                let ns_window_ptr = msg_send![ns_view, window];

                position_traffic_light(
                    UnsafeWindowHandle(ns_window_ptr),
                    WINDOW_CONTROL_PAD_X,
                    WINDOW_CONTROL_PAD_Y,
                );
            }
        }
    }
}
