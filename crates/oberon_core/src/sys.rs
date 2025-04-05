use std::io::{Error, Result as IoResult};

use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

use crate::linalg::vec2::Vec2;

// NOTE: This will only work on unix systems.
pub fn current_window_size() -> IoResult<Vec2>
{
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    match unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) }
    {
        0 => Ok(Vec2::new(size.ws_col as usize, size.ws_row as usize)),
        _ => Err(Error::last_os_error()),
    }
}
