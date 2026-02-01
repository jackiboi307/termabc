use termabc::prelude::*;

fn main() {
    // using escape sequences
    println!("{FG_RED}{BOLD}This is bold, red text{RESET}");

    // rgb using escape sequences
    printf!("{FG_RGB}This is orange text{RESET}\n", 255, 127, 0);

    // using the Style struct
    let italic_yellow = Style::new().fg(BrightYellow).italic();
    println!("{italic_yellow}This is italic, bright yellow text{RESET}");
}
