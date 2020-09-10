#![cfg(target_os = "macos")]

//! Help is wanted for this part. I don't have a Mac machine so I can't test
//! the code. The current implementation is a bit hacky and I would like to get rid
//! of it.  
//! An idea would be to port [this
//! code](https://github.com/sindresorhus/macos-wallpaper/blob/master/Sources/wallpaper/Wallpaper.swift).

mod macos;

pub use macos::{get, set};
