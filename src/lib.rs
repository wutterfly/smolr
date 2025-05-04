#![no_std]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod rand;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "linux")]
mod linux;
