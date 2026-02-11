#![allow(unused_imports)]

use std::io::{self, Read, Write, stdin, stdout};
use crate::control_sequences::*;

/// Flush stdout, ignoring errors
pub fn flush() {
    let _ = stdout().flush();
}

/** Read some amount of bytes from stdin
  * ```
  * let bytes = &*read_bytes::<8>()?;
  *
  * if bytes == ESCAPE {
  *     break Ok(())
  * }
  *
  * let string = str::from_utf8(&bytes)?;
  * ```
  */
pub fn read_bytes<const A: usize>() -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
    let mut bytes = [0; A];
    stdin().read(&mut bytes)?;
    Ok(strip_bytes(&bytes))
}

/// Block until Ctrl-C is pressed
pub fn block_until_interrupt() {
    loop {
        if let Ok(bytes) = read_bytes::<1>() {
            if &*bytes == crate::input_sequences::CTRL_C {
                break
            }
        } else {
            break
        }
    }
}

/// Strip trailing zeros (null bytes)
pub fn strip_bytes(bytes: &[u8]) -> Box<[u8]> {
    bytes.iter().map(|byte| *byte).take_while(|byte| *byte != 0).collect::<Vec<_>>().into()
}

#[cfg(feature = "crossterm")]
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

/// Enable raw mode and clear screen, without flushing stdout
#[cfg(feature = "crossterm")]
pub fn init_term() -> Result<(), io::Error> {
    let res = enable_raw_mode();
    print!("{ERASE_SCREEN}{CUR_HOME}{RESET}");
    flush();
    res
}

/// Disable raw mode, clear screen, show cursor, and flush stdout
#[cfg(feature = "crossterm")]
pub fn restore_term() -> Result<(), io::Error> {
    let res = disable_raw_mode();
    print!("{RESET}{ERASE_SCREEN}{CUR_HOME}{CUR_SHOW}");
    flush();
    res
}
