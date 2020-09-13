#![cfg(target_os = "macos")]

//! Help is wanted for this part. I don't have a Mac machine so I can't test
//! the code. The current implementation is a bit hacky and I would like to get rid
//! of it.  
//! An idea would be to port [this
//! code](https://github.com/sindresorhus/macos-wallpaper/blob/master/Sources/wallpaper/Wallpaper.swift).

use std::path::{Path, PathBuf};
use std::process::Command;

/// Sets the wallpaper given the full path of an image.
pub fn set<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    // Generate the Applescript string
    let cmd = &format!(
        r#"tell app "finder" to set desktop picture to POSIX file {}"#,
        path.as_ref().display(),
    );
    // Run it using osascript
    Command::new("osascript").args(&["-e", cmd]).output()?;
    Ok(())
}

/// Gets the full path of the current wallpaper.
pub fn get() -> anyhow::Result<PathBuf> {
    // Generate the Applescript string
    let cmd = r#"tell app "finder" to get posix path of (get desktop picture as alias)"#;
    // Run it using osascript
    let output = Command::new("osascript").args(&["-e", cmd]).output()?;
    Ok(String::from_utf8(output.stdout)?.trim().into())
}
