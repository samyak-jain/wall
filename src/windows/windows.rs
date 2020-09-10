#![cfg(windows)]

use {
    anyhow::Context,
    std::{
        ffi::OsStr,
        io, iter, mem,
        os::{raw::c_void, windows::ffi::OsStrExt},
        path::PathBuf,
    },
    winapi::um::winuser::{
        SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETDESKWALLPAPER,
        SPI_SETDESKWALLPAPER,
    },
};

/// Returns the full path to the current wallpaper.
pub fn get() -> anyhow::Result<PathBuf> {
    let buffer: [u16; 260] = unsafe { mem::zeroed() };
    let success = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            buffer.as_ptr() as *mut c_void,
            0,
        ) == 1
    };

    if success {
        // Removes trailing zeroes from buffer
        let mut buffer = &buffer[..];
        while let Some(0) = buffer.last() {
            buffer = buffer.split_last().unwrap().1;
        }
        Ok(String::from_utf16(buffer)?.into())
    } else {
        Err(io::Error::last_os_error().into())
    }
}

/// Sets the wallpaper given the full path of an image.
pub fn set<P>(full_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let path = OsStr::new(full_path)
        .encode_wide()
        // Append null byte
        .chain(iter::once(0))
        .collect::<Vec<u16>>();

    let success = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1
    };

    if success {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}
