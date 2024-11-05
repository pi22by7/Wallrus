use crate::errors::{Result, WallrusError};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use winapi::um::winuser::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

pub fn set_wallpaper(image_path: &Path) -> Result<()> {
    if !image_path.exists() {
        return Err(WallrusError::Config(format!(
            "Wallpaper file does not exist: {:?}",
            image_path
        )));
    }

    let wide_path: Vec<u16> = OsStr::new(image_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide_path.as_ptr() as *const _ as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result == 0 {
            return Err(WallrusError::Io(std::io::Error::last_os_error()));
        }
    }

    Ok(())
}
