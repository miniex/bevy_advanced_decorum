//! # Bevy Advanced Decorum
//!
//! A highly customizable window decoration plugin for the Bevy engine,inspired by tauri-plugin-decorum

#![warn(missing_docs, clippy::doc_markdown, clippy::unwrap_or_default)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::type_complexity)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod plugin;
pub mod settings;
#[cfg(target_os = "macos")]
mod traffic;

pub mod prelude {
    pub use crate::plugin::DecorumPlugin;
    pub use crate::settings::DecorumSettings;
}
