#![cfg(target_os = "macos")]

mod macos;

pub use macos::{get, set};
