use termabc::*;
use termabc::{
    control_sequences::*,
    Color::*,
};

enum PaneType {
    Normal,
    Special,
}

fn main() {
    let paner = {
        use Paner::*;
        use PaneType::*;

        Horizontal(vec![
            (1, Pane(Normal)),
            (1, Vertical(vec![
                (1, Pane(Normal)),
                (1, Horizontal(vec![
                    (1, Pane(Special)),
                    (1, Pane(Normal)),
                ])),
                (1, Pane(Normal)),
                (1, Pane(Normal)),
            ])),
            (1, Vertical(vec![
                (1, Pane(Normal)),
                (1, Pane(Special)),
            ])),
        ])
    };

    let mut size = termsize::get().unwrap();
    size.rows -= 1; // reserve last row for shell prompt

    let (rendered_paner, panes) = paner.render(0, 0, size.cols, size.rows, &BorderStyle::Connected2("-", "|"));

    let default_style = Style::new().fg(BrightRed).bg(BrightBlack);

    print!("{ERASE_SCREEN}{}", rendered_paner);

    for (panetype, col, row, width, height) in panes {
        let mut canvas = InstructionBuffer::new(width, height, Some(&default_style));

        let (text, style) = match panetype {
            PaneType::Normal => ("example text", None),
            PaneType::Special => ("special text", Some(&default_style.with_fg(BrightGreen).bold()))
            // NOTE with_fg() clones the color, which should be avoided in a loop like this
        };

        canvas.addstr(0, 0, text, style);
        print!("{}", canvas.render(col, row));
    }

    printf!("{CUR_SET}{RESET}", size.rows + 1, 1);
}
