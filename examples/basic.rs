use termabc::*;
use termabc::control_sequences::*;

enum PaneType {
    Normal,
    Special,
}

fn main() {
    let paner = Paner::Horizontal(vec![
        (1, Paner::Pane(PaneType::Normal)),
        (1, Paner::Vertical(vec![
            (1, Paner::Pane(PaneType::Normal)),
            (1, Paner::Horizontal(vec![
                (1, Paner::Pane(PaneType::Special)),
                (1, Paner::Pane(PaneType::Normal)),
            ])),
            (1, Paner::Pane(PaneType::Normal)),
            (1, Paner::Pane(PaneType::Normal)),
        ])),
        (1, Paner::Vertical(vec![
            (1, Paner::Pane(PaneType::Normal)),
            (1, Paner::Pane(PaneType::Special)),
        ])),
    ]);

    let mut size = termsize::get().unwrap();
    size.rows -= 1; // reserve last row for shell prompt

    let (rendered_paner, panes) = paner.render(0, 0, size.cols, size.rows,
        &BorderStyle::Connected2 {
            borderh: "-",
            borderv: "|",
        }
    );

    let default_style = Style::new().fg(Color::Red);

    print!("{ERASE_SCREEN}");
    print!("{}", rendered_paner);

    for (panetype, col, row, width, height) in panes {
        let mut canvas = InstructionBuffer::new(width, height, Some(&default_style));

        let (text, style) = match panetype {
            PaneType::Normal => ("example text", None),
            PaneType::Special => ("special text", Some(&Style::new().fg(Color::Green)))
        };

        canvas.addstr(0, 0, text, style);

        print!("{}", canvas.render(col, row));
    }

    printf!("{CUR_SET}", size.rows + 1, 1);
}
