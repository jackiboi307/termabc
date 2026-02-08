// Keys

pub const ESCAPE: &[u8] = b"\x1b";

/// Adding escape in front of most key codes gives its alt variant.
/// ALT can be used instead of ESCAPE for clarity.
pub const ALT: &[u8] = ESCAPE;

pub const TAB: &[u8] = b"\x09";
pub const RETURN: &[u8] = b"\x0d";
pub const BACKSPACE: &[u8] = b"\x7f";
pub const SHIFT_TAB: &[u8] = b"\x1b[Z";
pub const CTRL_BACKSPACE: &[u8] = b"\x08";

pub const HOME: [&[u8]; 2] = [b"\x1b[1", b"\x1b[H"];
pub const END: [&[u8]; 2] = [b"\x1b[4", b"\x1b[F"];
pub const PG_UP: &[u8] = b"\x1b[5~";
pub const PG_DOWN: &[u8] = b"\x1b[6~";
pub const DELETE: &[u8] = b"\x1b[3~";
pub const INSERT: &[u8] = b"\x1b[2~";

// Arrows

pub const ARROW_UP: &[u8] = b"\x1b[A";
pub const ARROW_DOWN: &[u8] = b"\x1b[B";
pub const ARROW_LEFT: &[u8] = b"\x1b[D";
pub const ARROW_RIGHT: &[u8] = b"\x1b[C";
pub const SHIFT_ARROW_UP: &[u8] = b"\x1b[1;2A";
pub const SHIFT_ARROW_DOWN: &[u8] = b"\x1b[1;2B";
pub const SHIFT_ARROW_LEFT: &[u8] = b"\x1b[1;2D";
pub const SHIFT_ARROW_RIGHT: &[u8] = b"\x1b[1;2C";
pub const CTRL_ARROW_UP: &[u8] = b"\x1b[1;5A";
pub const CTRL_ARROW_DOWN: &[u8] = b"\x1b[1;5B";
pub const CTRL_ARROW_LEFT: &[u8] = b"\x1b[1;5D";
pub const CTRL_ARROW_RIGHT: &[u8] = b"\x1b[1;5C";
pub const CTRL_SHIFT_ARROW_UP: &[u8] = b"\x1b[1;6A";
pub const CTRL_SHIFT_ARROW_DOWN: &[u8] = b"\x1b[1;6B";
pub const CTRL_SHIFT_ARROW_LEFT: &[u8] = b"\x1b[1;6D";
pub const CTRL_SHIFT_ARROW_RIGHT: &[u8] = b"\x1b[1;6C";

// Function keys

pub const F1: [&[u8]; 2] = [b"\x1bOP", b"\x1b[11~"];
pub const F2: [&[u8]; 2] = [b"\x1bOQ", b"\x1b[12~"];
pub const F3: [&[u8]; 2] = [b"\x1bOR", b"\x1b[13~"];
pub const F4: [&[u8]; 2] = [b"\x1bOS", b"\x1b[14~"];
pub const F5: &[u8] = b"\x1b[15~";
pub const F6: &[u8] = b"\x1b[17~";
pub const F7: &[u8] = b"\x1b[18~";
pub const F8: &[u8] = b"\x1b[19~";
pub const F9: &[u8] = b"\x1b[20~";
pub const F10: &[u8] = b"\x1b[21~";
pub const F11: &[u8] = b"\x1b[23~";
pub const F12: &[u8] = b"\x1b[24~";

// Control + letter

pub const CTRL_A: &[u8] = b"\x01";
pub const CTRL_B: &[u8] = b"\x02";
pub const CTRL_C: &[u8] = b"\x03";
pub const CTRL_D: &[u8] = b"\x04";
pub const CTRL_E: &[u8] = b"\x05";
pub const CTRL_F: &[u8] = b"\x06";
pub const CTRL_G: &[u8] = b"\x07";
pub const CTRL_H: &[u8] = b"\x08";
pub const CTRL_I: &[u8] = b"\x09";
pub const CTRL_J: &[u8] = b"\x0a";
pub const CTRL_K: &[u8] = b"\x0b";
pub const CTRL_L: &[u8] = b"\x0c";
pub const CTRL_M: &[u8] = b"\x0d";
pub const CTRL_N: &[u8] = b"\x0e";
pub const CTRL_O: &[u8] = b"\x0f";
pub const CTRL_P: &[u8] = b"\x10";
pub const CTRL_Q: &[u8] = b"\x11";
pub const CTRL_R: &[u8] = b"\x12";
pub const CTRL_S: &[u8] = b"\x13";
pub const CTRL_T: &[u8] = b"\x14";
pub const CTRL_U: &[u8] = b"\x15";
pub const CTRL_V: &[u8] = b"\x16";
pub const CTRL_W: &[u8] = b"\x17";
pub const CTRL_X: &[u8] = b"\x18";
pub const CTRL_Y: &[u8] = b"\x19";
pub const CTRL_Z: &[u8] = b"\x1a";
