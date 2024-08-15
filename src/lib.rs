//! # Bevy Advanced Decorum
//!
//! A highly customizable window decoration plugin for the Bevy engine,inspired by tauri-plugin-decorum
//!
//!

mod decorum;
pub mod plugin;
#[cfg(target_os = "macos")]
mod traffic;

pub use decorum::*;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod prelude {
    pub use crate::plugin::DecorumPlugin;
}
