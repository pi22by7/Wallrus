// src/wallpaper/mod.rs
mod common;
mod generators;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub use common::*;
#[cfg(target_os = "linux")]
pub use linux::set_wallpaper;
#[cfg(target_os = "macos")]
pub use macos::set_wallpaper;
#[cfg(target_os = "windows")]
pub use windows::set_wallpaper;
