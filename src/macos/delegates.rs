use crate::window::WindowState;
use cocoa::base::{id, BOOL, YES};
use cocoa::foundation::NSUInteger;
use objc::runtime::{Object, Sel};
use objc::{msg_send, sel, sel_impl};

pub fn setup_window_delegate(ns_window: id, current_delegate: id, app_state: WindowState) {
    use cocoa::appkit::NSWindow;
    use cocoa::base::{id, BOOL};
    use cocoa::foundation::NSUInteger;
    use objc::runtime::{Object, Sel};
    use std::ffi::c_void;

    let app_box = Box::into_raw(Box::new(app_state)) as *mut c_void;

    let delegate_name = generate_random_delegate_name();

    unsafe {
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
                }));
    }
}

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

extern "C" fn on_window_did_change_backing_properties(this: &Object, _cmd: Sel, notification: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, windowDidChangeBackingProperties: notification];
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

extern "C" fn on_dragging_entered(this: &Object, _cmd: Sel, notification: id) -> BOOL {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        msg_send![super_del, draggingEntered: notification]
    }
}

extern "C" fn on_prepare_for_drag_operation(this: &Object, _cmd: Sel, notification: id) -> BOOL {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        msg_send![super_del, prepareForDragOperation: notification]
    }
}

extern "C" fn on_perform_drag_operation(this: &Object, _cmd: Sel, sender: id) -> BOOL {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        msg_send![super_del, performDragOperation: sender]
    }
}

extern "C" fn on_conclude_drag_operation(this: &Object, _cmd: Sel, notification: id) {
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

extern "C" fn on_window_did_enter_full_screen(this: &Object, _cmd: Sel, notification: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, windowDidEnterFullScreen: notification];
    }
}

extern "C" fn on_window_will_enter_full_screen(this: &Object, _cmd: Sel, notification: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, windowWillEnterFullScreen: notification];
    }
}

extern "C" fn on_window_did_exit_full_screen(this: &Object, _cmd: Sel, notification: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, windowDidExitFullScreen: notification];
    }
}

extern "C" fn on_window_will_exit_full_screen(this: &Object, _cmd: Sel, notification: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, windowWillExitFullScreen: notification];
    }
}

extern "C" fn on_window_did_fail_to_enter_full_screen(this: &Object, _cmd: Sel, window: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, windowDidFailToEnterFullScreen: window];
    }
}

extern "C" fn on_effective_appearance_did_change(this: &Object, _cmd: Sel, notification: id) {
    unsafe {
        let super_del: id = *this.get_ivar("super_delegate");
        let _: () = msg_send![super_del, effectiveAppearanceDidChange: notification];
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

fn generate_random_delegate_name() -> String {
    use rand::{distributions::Alphanumeric, Rng};
    format!(
        "windowDelegate_{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>()
    )
}
