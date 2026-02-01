#![allow(unused_imports)]

use std::io::{
    self,
    Write,
    stdout,
};
use crate::control_sequences::*;

/// Flush stdout, ignoring errors
pub fn flush() {
    let _ = stdout().flush();
}

#[cfg(feature = "crossterm")]
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

/// Enable raw mode and clear screen, without flushing stdout
#[cfg(feature = "crossterm")]
pub fn init_term() -> Result<(), io::Error> {
    let res = enable_raw_mode();
    print!("{ERASE_SCREEN}{CUR_HOME}{RESET}");
    res
}

/// Disable raw mode and clear screen, and flush stdout
#[cfg(feature = "crossterm")]
pub fn restore_term() -> Result<(), io::Error> {
    let res = disable_raw_mode();
    print!("{RESET}{ERASE_SCREEN}{CUR_HOME}");
    flush();
    res
}
