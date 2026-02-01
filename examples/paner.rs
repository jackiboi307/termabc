use termabc::*;
use termabc::{
    control_sequences::*,
    Color::*,
};

// custom id used for panes
enum PaneType {
    Normal,
    Special,
}

fn main() {
    // construct the layout
    let paner = {
        use Paner::*;
        use PaneType::*;
        use PaneSize::*;

        Horizontal(vec![
            (Relative(2), Vertical(vec![
                (Relative(1), Pane(Normal)),
                (Relative(1), Pane(Normal)),
            ])),
            (Relative(3), Vertical(vec![
                (Fixed(10), Horizontal(vec![
                    (Relative(1), Pane(Normal)),
                    (Relative(1), Pane(Special)),
                ])),
                (Fixed(1), Pane(Special)),
                (Relative(1), Pane(Normal)),
                (Relative(1), Pane(Normal)),
            ])),
            (Relative(1), Vertical(vec![
                (Relative(1), Pane(Normal)),
                (Relative(2), Pane(Special)),
            ])),
        ])
    };

    // get terminal size
    let mut size = termsize::get().unwrap();
    size.rows -= 1; // reserve last row for shell prompt

    // render the paner
    let (rendered_borders, panes) = paner.render(0, 0, size.cols, size.rows,
        &BorderStyle::CONNECTED_LIGHT
        // &BorderStyle::DISCONNECTED_LIGHT
        // &BorderStyle::Gap(1)
        // &BorderStyle::connected_from_str("-|+++++++++")
    );

    // print the borders
    print!("{ERASE_SCREEN}{}", rendered_borders);

    let default_style = Style::new().fg(BrightRed).bg(BrightBlack);

    // render individual panes
    for (panetype, col, row, width, height) in panes {
        // create a new canvas
        let mut canvas = InstructionBuffer::new(width, height, Some(&default_style));

        // select text and style based on the PaneType value
        let (text, style) = match panetype {
            PaneType::Normal => ("example text", None),
            PaneType::Special => ("special text", Some(&default_style.with_fg(BrightGreen).bold()))
            // NOTE with_fg() clones the color, which should be avoided in a loop like this
        };

        canvas.addstr(0, 0, text, style);

        // render and print the canvas
        print!("{}", canvas.render(col, row));
    }

    // go to the last row so that the shell prompt will appear there
    printf!("{CUR_SET}{RESET}", size.rows + 1, 1);
}
