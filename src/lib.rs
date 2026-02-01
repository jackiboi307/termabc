pub mod prelude;
pub mod control_sequences;
pub mod input_sequences;
pub mod utils;
mod macros;

use unicode_segmentation::UnicodeSegmentation;
use control_sequences::*;

use std::fmt;

type Cell = u16;
type Str = String;

/// Exists to make the documentation easier to follow, might be removed
pub type Static = &'static str;

pub trait Canvas {
    fn clear(&mut self);
    fn render(&self, col: Cell, row: Cell) -> Str;
    fn addtext(&mut self, string: &str);
    fn addcmd(&mut self, cmd: &str);
    fn setstyle(&mut self, style: Option<&Style>);
    fn setcursor(&mut self, col: Cell, row: Cell);

    fn addstr(&mut self, col: Cell, row: Cell, string: &str, style: Option<&Style>) {
        self.setcursor(col, row);
        self.setstyle(style);
        self.addtext(string);
    }

    fn draw_hbar(&mut self, col: Cell, row: Cell, length: Cell, ch: &str, style: Option<&Style>) {
        self.addstr(col, row, &ch.repeat(length.into()), style);
    }

    fn draw_vbar(&mut self, col: Cell, row: Cell, length: Cell, ch: &str, style: Option<&Style>) {
        self.setstyle(style);
        for i in 0..length {
            self.setcursor(col, row + i);
            self.addtext(ch);
        }
    }

    fn draw_box(&mut self, col: Cell, row: Cell, width: Cell, height: Cell,
            border: &BorderStyle, style: Option<&Style>) {

        let (h, v, tl, tr, bl, br) = match *border {
            BorderStyle::Gap(..) => ("", "", "", "", "", ""),
            BorderStyle::Connected([h, v, tl, tr, bl, br, _, _, _, _, _]) |
            BorderStyle::Disconnected([h, v, tl, tr, bl, br]) => (h, v, tl, tr, bl, br),
        };

        // draw sides
        self.draw_hbar(col + 1, row, width.saturating_sub(2), h, style);
        self.draw_hbar(col + 1, row + height.saturating_sub(1), width.saturating_sub(2), h, style);
        self.draw_vbar(col, row + 1, height.saturating_sub(2), v, style);
        self.draw_vbar(col + width.saturating_sub(1), row + 1, height.saturating_sub(2), v, style);

        draw_corners(self, col, row, width, height, tl, tr, bl, br, style);
    }
}

// NOTE these are also used in Paner, hence the ugly separation from Canvas.draw_box

fn draw_corners(
        canvas: &mut (impl Canvas + ?Sized),
        col: Cell,
        row: Cell,
        width: Cell,
        height: Cell,
        tl: Static,
        tr: Static,
        bl: Static,
        br: Static,
        style: Option<&Style>) {

    canvas.setstyle(style);
    canvas.setcursor(col, row);
    canvas.addtext(tl);
    canvas.setcursor(col + width.saturating_sub(1), row);
    canvas.addtext(tr);
    canvas.setcursor(col, row + height.saturating_sub(1));
    canvas.addtext(bl);
    canvas.setcursor(col + width.saturating_sub(1), row + height.saturating_sub(1));
    canvas.addtext(br);
}

pub enum BorderStyle {
    /// An empty gap of some size
    Gap(Cell),
    Connected([Static; 11]),
    Disconnected([Static; 6]),
}

impl BorderStyle {
    pub const CONNECTED_LIGHT: Self =
        BorderStyle::Connected(["─", "│", "┌", "┐", "└", "┘", "├", "┤", "┬", "┴", "┼"]);
    pub const DISCONNECTED_LIGHT: Self =
        BorderStyle::Disconnected(["─", "│", "┌", "┐", "└", "┘"]);

    pub const CONNECTED_HEAVY: Self =
        BorderStyle::Connected(["━", "┃", "┏", "┓", "┗", "┛", "┣", "┫", "┳", "┻", "╋"]);
    pub const DISCONNECTED_HEAVY: Self =
        BorderStyle::Disconnected(["━", "┃", "┏", "┓", "┗", "┛"]);

    pub const CONNECTED_DOUBLE: Self =
        BorderStyle::Connected(["═", "║", "╔", "╗", "╚", "╝", "╠", "╣", "╦", "╩", "╬"]);
    pub const DISCONNECTED_DOUBLE: Self =
        BorderStyle::Disconnected(["═", "║", "╔", "╗", "╚", "╝"]);
    

    pub fn connected_from_str(string: Static) -> Self {
        let vec: Vec<_> = string.graphemes(true).collect();
        Self::Connected(vec.try_into().expect("expected 11 characters"))
    }

    pub fn disconnected_from_str(string: Static) -> Self {
        let vec: Vec<_> = string.graphemes(true).collect();
        Self::Connected(vec.try_into().expect("expected 6 characters"))
    }

    fn gap(&self) -> Cell {
        match self {
            Self::Gap(cells) => *cells,
            Self::Connected(..) => 1,
            Self::Disconnected(..) => 2,
        }
    }
}

#[derive(Default, Clone)]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Ansi(u8),
    True(u8, u8, u8),
}

impl Color {
    pub fn as_string_fg(&self) -> Str {
        match self {
            Color::Default => control_sequences::FG_DEFAULT,
            Color::Black => control_sequences::FG_BLACK,
            Color::Red => control_sequences::FG_RED,
            Color::Green => control_sequences::FG_GREEN,
            Color::Yellow => control_sequences::FG_YELLOW,
            Color::Blue => control_sequences::FG_BLUE,
            Color::Magenta => control_sequences::FG_MAGENTA,
            Color::Cyan => control_sequences::FG_CYAN,
            Color::White => control_sequences::FG_WHITE,
            Color::BrightBlack => control_sequences::FG_BLACK_B,
            Color::BrightRed => control_sequences::FG_RED_B,
            Color::BrightGreen => control_sequences::FG_GREEN_B,
            Color::BrightYellow => control_sequences::FG_YELLOW_B,
            Color::BrightBlue => control_sequences::FG_BLUE_B,
            Color::BrightMagenta => control_sequences::FG_MAGENTA_B,
            Color::BrightCyan => control_sequences::FG_CYAN_B,
            Color::BrightWhite => control_sequences::FG_WHITE_B,
            Color::Ansi(..) | Color::True(..) => {
                match self {
                    Color::Ansi(id) => return formatf!("{FG_ID}", id),
                    Color::True(r, g, b) => return formatf!("{FG_RGB}", r, g, b),
                    _ => unreachable!()
                }
            }
        }.to_string()
    }

    pub fn as_string_bg(&self) -> Str {
        match self {
            Color::Default => control_sequences::BG_DEFAULT,
            Color::Black => control_sequences::BG_BLACK,
            Color::Red => control_sequences::BG_RED,
            Color::Green => control_sequences::BG_GREEN,
            Color::Yellow => control_sequences::BG_YELLOW,
            Color::Blue => control_sequences::BG_BLUE,
            Color::Magenta => control_sequences::BG_MAGENTA,
            Color::Cyan => control_sequences::BG_CYAN,
            Color::White => control_sequences::BG_WHITE,
            Color::BrightBlack => control_sequences::BG_BLACK_B,
            Color::BrightRed => control_sequences::BG_RED_B,
            Color::BrightGreen => control_sequences::BG_GREEN_B,
            Color::BrightYellow => control_sequences::BG_YELLOW_B,
            Color::BrightBlue => control_sequences::BG_BLUE_B,
            Color::BrightMagenta => control_sequences::BG_MAGENTA_B,
            Color::BrightCyan => control_sequences::BG_CYAN_B,
            Color::BrightWhite => control_sequences::BG_WHITE_B,
            Color::Ansi(..) | Color::True(..) => {
                match self {
                    Color::Ansi(id) => return formatf!("{BG_ID}", id),
                    Color::True(r, g, b) => return formatf!("{BG_RGB}", r, g, b),
                    _ => unreachable!()
                }
            }
        }.to_string()
    }
}

#[derive(Clone)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub strike: bool,
}

static EMPTY_STYLE: Style = Style::new();

macro_rules! gen_methods {
    ($($name:ident),*) => {
        $(
            pub const fn $name(mut self) -> Self {
                self.$name = true;
                self
            }
        )*
    }
}

impl Style {
    pub const EMPTY: &'static Style = &EMPTY_STYLE;

    pub const fn new() -> Self {
        Self {
            fg: Some(Color::Default),
            bg: Some(Color::Default),
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            strike: false,
        }
    }

    gen_methods!(bold, dim, italic, underline, blink, strike);

    pub const fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    pub const fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    // pub fn get_fg(&self) -> Option<&Color> {
    //     self.fg.as_ref()
    // }

    // pub fn get_bg(&self) -> Option<&Color> {
    //     self.bg.as_ref()
    // }

    pub fn with_fg(&self, fg: Color) -> Self {
        self.clone().fg(fg)
    }

    pub fn with_bg(&self, bg: Color) -> Self {
        self.clone().bg(bg)
    }

    pub fn as_string(&self) -> Str {
        // NOTE always including RESET might be wasteful?
        let mut string = String::from(RESET);

        if let Some(fg) = &self.fg { string.push_str(&fg.as_string_fg()) }
        if let Some(bg) = &self.bg { string.push_str(&bg.as_string_bg()) }

        if self.bold { string.push_str(BOLD); }
        if self.dim { string.push_str(DIM); }
        if self.italic { string.push_str(ITALIC); }
        if self.underline { string.push_str(UNDERLINE); }
        if self.blink { string.push_str(BLINK); }
        if self.strike { string.push_str(STRIKE); }

        string
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_string())
    }
}

#[derive(Clone)]
enum Instruction {
    // TODO add more instructions for moving the cursor, so cursor position can be kept 
    // aware of and it is possible to ensure you can not draw outside canvases
    Text(String),
    Style(String),
    SetCursor(Cell, Cell),
    Command(String),
}

#[derive(Clone)]
pub struct InstructionBuffer<'a> {
    instructions: Vec<Instruction>,
    default_style: &'a Style,
    width: Cell,
    height: Cell,
}

impl<'a> InstructionBuffer<'a> {
    pub fn new(width: Cell, height: Cell, default_style: Option<&'a Style>) -> Self {
        Self {
            instructions: Vec::new(),
            default_style: default_style.unwrap_or_else(|| &Style::EMPTY),
            width,
            height,
        }
    }
}

impl<'a> Canvas for InstructionBuffer<'a> {
    fn clear(&mut self) {
        self.instructions.clear();
    }

    fn addtext(&mut self, string: &str) {
        self.instructions.push(Instruction::Text(string.to_string()));
    }

    fn addcmd(&mut self, cmd: &str) {
        self.instructions.push(Instruction::Command(cmd.to_string()));
    }

    fn setcursor(&mut self, col: Cell, row: Cell) {
        self.instructions.push(Instruction::SetCursor(col, row));
    }

    fn setstyle(&mut self, style: Option<&Style>) {
        self.instructions.push(Instruction::Style(
            style.unwrap_or_else(|| self.default_style).as_string()
        ));
    }

    fn render(&self, start_col: Cell, start_row: Cell) -> Str {
        let mut result = String::new();
        let mut row = start_row;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Text(string) => {
                    if row < self.height {
                        // truncate if too long
                        let string: String = string.graphemes(true).take(self.width.into()).collect();
                        result.push_str(&string);
                    }
                }
                Instruction::SetCursor(col, new_row) => {
                    result.push_str(&formatf!("{CUR_SET}", start_row + new_row + 1, start_col + col + 1));
                    row = *new_row;
                }
                Instruction::Command(string) | Instruction::Style(string) => {
                    result.push_str(string);
                }
            }
        }

        result
    }
}

pub enum PaneSize {
    Fixed(Cell),
    Relative(Cell),
}

pub enum Paner<T> {
    Pane(T),
    Horizontal(Vec<(PaneSize, Paner<T>)>),
    Vertical(Vec<(PaneSize, Paner<T>)>),
}

impl<T> Paner<T> {
    pub fn render(
            &self,
			start_col: Cell,
			start_row: Cell,
			width: Cell,
			height: Cell,
			border: &BorderStyle) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let (mut string, arr) = self.render_sub(
            start_col + 1,
            start_row + 1,
            width - 2,
            height - 2,
            border,
            (start_col, start_row, width, height),
            false
        );

        let mut canvas = InstructionBuffer::new(width, height, None);

        match border {
            BorderStyle::Gap(..) => {}
            BorderStyle::Disconnected(..) => {}
            BorderStyle::Connected([_, _, tl, tr, bl, br, _, _, _, _, _]) => {
                // NOTE this is probably very inefficient!
                let (corners, _) = self.render_sub(
                    start_col + 1,
                    start_row + 1,
                    width - 2,
                    height - 2,
                    border,
                    (start_col, start_row, width, height),
                    true
                );

                string.push_str(&corners);
                draw_corners(&mut canvas, start_col, start_row, width, height, tl, tr, bl, br, None);
            }
        }

        string.push_str(&canvas.render(0, 0));
        (string, arr)
    }

    fn render_sub(
            &self,
			start_col: Cell,
			start_row: Cell,
			width: Cell,
			height: Cell,
			border: &BorderStyle,
            original: (Cell, Cell, Cell, Cell),
            corners: bool) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let mut arr = Vec::new();
        let mut string = String::new();

        match self {
            Self::Pane(pane) => {
                arr.push((pane, start_col, start_row, width, height));

                match border {
                    BorderStyle::Disconnected(..) => {
                        let (width, height) = (width + 2, height + 2);
                        let mut canvas = InstructionBuffer::new(width, height, None);
                        canvas.draw_box(0, 0, width, height, border, None);
                        string.push_str(&canvas.render(start_col - 1, start_row - 1));
                    }
                    BorderStyle::Connected(..) |
                    BorderStyle::Gap(..) => {}
                }
            }
            Self::Horizontal(paners) | Self::Vertical(paners) => {
                let total_rel: Cell = paners.iter()
                    .map(|i| if let PaneSize::Relative(size) = i.0 { size } else { 0 }).sum();
                let total_fixed: Cell = paners.iter()
                    .map(|i| if let PaneSize::Fixed(size) = i.0 { size } else { 0 }).sum();

                let horizontal = match self {
                    Self::Horizontal(..) => true,
                    _ => false
                };

                let gap = border.gap();

                let mut canvas = InstructionBuffer::new(width + 2, height + 2, None);

                let mut i = 0;
                for (j, (size, paner)) in paners.iter().enumerate() {
                    let size = match size {
                        PaneSize::Relative(size) => {
                            let size = ((if horizontal { width } else { height })
                                .saturating_sub(total_fixed) * size / total_rel)
                                .saturating_sub(gap) - 1;

                            // extend the last element if it can not be perfect
                            // TODO improve this and make it even
                            let size = size + if j == paners.len() - 1 {
                                (if horizontal { width } else { height }).saturating_sub(size + i)
                            } else { 0 };

                            size
                        }
                        PaneSize::Fixed(size) => *size
                    };

                    let (new_string, mut new_arr) = paner.render_sub(
                        if horizontal { start_col + i } else { start_col },
                        if horizontal { start_row } else { start_row + i },
                        if horizontal { size } else { width },
                        if horizontal { height } else { size },
                        border,
                        original,
                        corners
                    );
                    string.push_str(&new_string);
                    arr.append(&mut new_arr);

                    let (old_width, old_height) = (width, height);

                    let (col, row, width, height) = (
                        if horizontal { i } else { 0 },
                        if horizontal { 0 } else { i },
                        if horizontal { size } else { width } + 2,
                        if horizontal { height } else { size } + 2,
                    );

                    match border {
                        BorderStyle::Connected([h, v, _, _, _, _, right, left, down, up, _]) => {
                            if !corners {
                                if horizontal {
                                    canvas.draw_vbar(i, 0, old_height + 1, v, None);
                                } else {
                                    canvas.draw_hbar(0, i, old_width + 1, h, None);
                                }

                                if j == paners.len() - 1 {
                                    if horizontal {
                                        canvas.draw_vbar(i + size + 1, 0, old_height + 1, v, None);
                                    } else {
                                        canvas.draw_hbar(0, i + size + 1, old_width + 1, h, None);
                                    }
                                }

                            } else {
                                draw_corners(
                                    &mut canvas,
                                    col,
                                    row,
                                    width,
                                    height,
                                    right,
                                    left,
                                    up,
                                    down,
                                    None
                                );
                            }
                        }
                        BorderStyle::Disconnected(..) |
                        BorderStyle::Gap(..) => {}
                    }

                    if corners {
                        match border {
                            BorderStyle::Connected(border) => {
                                if start_col + col - 1 == original.0 {
                                    canvas.addstr(col, row, border[6], None);
                                }

                                if start_col + col + width - 1 == original.2 {
                                    canvas.addstr(col + width - 1, row, border[7], None);
                                }

                                if start_row + row - 1 == original.1 {
                                    canvas.addstr(col, row, border[8], None);
                                }

                                if start_row + row + height - 1 == original.3 {
                                    canvas.addstr(col, row + height - 1, border[9], None);
                                }
                            }
                            _ => {}
                        }
                    }

                    i += size + gap;
                }

                string.push_str(&canvas.render(start_col - 1, start_row - 1));
            }
        }

        (string, arr)
    }
}
