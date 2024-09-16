use bevy::prelude::{Window, World};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct WindowState {
    window: Window,
    world: Option<World>,
    is_shutting_down: AtomicBool,
}

impl WindowState {
    pub fn new(window: Window) -> Self {
        WindowState {
            window,
            world: None,
            is_shutting_down: AtomicBool::new(false),
        }
    }

    pub fn set_world(&mut self, world: World) {
        self.world = Some(world);
    }

    pub fn start_shutdown(&self) {
        self.is_shutting_down.store(true, Ordering::Relaxed);
    }
}
