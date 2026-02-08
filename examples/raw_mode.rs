use termabc::prelude::*;
use defer_rs::defer;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("waiting for input...");
    flush();

    loop {
        // read 8 bytes
        // depending on what input sequences are needed,
        // you can use a smaller or bigger amount
        let bytes = &*read_bytes::<8>()?;

        print!("{ERASE_SCREEN}{CUR_HOME}");

        match bytes {
            ESCAPE | CTRL_C => break Ok(()),
            RETURN => print!("return pressed"),
            _ => {
                // some keys have multiple sequences
                if HOME.contains(&bytes) {
                    print!("home pressed");

                } else {
                    // parse the input as utf8
                    let string = str::from_utf8(&bytes)?;
                    printf!("bytes: {bytes:?}{CUR_SET}string: '{string}'", 2, 1);
                }
            }
        }

        flush();
    }
}

fn main() {
    // hide cursor
    print!("{CUR_HIDE}");
    init_term().unwrap();

    // restore the terminal when the code finishes or crashes
    defer! {
        let _ = restore_term();
    };

    let res = run();
    res.expect("error occured");
}
