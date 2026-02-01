// requires some features to be enabled

use termabc::utils::*;
use std::{
    thread::sleep,
    time::Duration,
};

fn main() {
    let _ = init_term();

    for i in 0..5 {
        let i = 5 - i;
        print!("\rRaw mode has been activated! (exiting in {i} secs)");

        flush(); // flushing is required in raw mode!
        sleep(Duration::from_secs(1));
    }

    let _ = restore_term();
}
