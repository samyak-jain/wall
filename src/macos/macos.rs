#![cfg(macos)]

use std::path::{Path, PathBuf};
use std::process::Command;

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

pub fn get() -> anyhow::Result<PathBuf> {
    // Generate the Applescript string
    let cmd = r#"tell app "finder" to get posix path of (get desktop picture as alias)"#;
    // Run it using osascript
    let output = Command::new("osascript").args(&["-e", cmd]).output()?;
    Ok(String::from_utf8(output.stdout)?.trim().into())
}
