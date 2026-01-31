pub mod control_sequences;
pub mod input_sequences;

use control_sequences::*;

use std::fmt;

/** A sort of dynamic version of formatf, that can be used with control sequence constants.
  * ```
  * formatf!(
  *     "{FG_RGB}{BG_RGB}Yellow text against a red background{RESET}\n",
  *     255, 255, 0,
  *     255, 0, 0
  * );
  * ```
  */
#[macro_export]
macro_rules! formatf {
    ($fmt:expr $(, $arg:expr)*) => {{
        let mut result = format!($fmt);
        $(
            result = result.replacen("{}", &$arg.to_string(), 1);
        )*
        result
    }};
}

/** A sort of dynamic version of printf, that can be used with control sequence constants.
  * ```
  * printf!(
  *     "{FG_RGB}{BG_RGB}Yellow text against a red background{RESET}\n",
  *     255, 255, 0,
  *     255, 0, 0
  * );
  * ```
  */
#[macro_export]
macro_rules! printf {
    ($fmt:expr $(, $arg:expr)*) => {{
        print!("{}", formatf!($fmt $(, $arg)*));
    }};
}

type Cell = u16;
type Str = String;
type Static = &'static str;

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
                        let string = if usize::from(self.width) < string.len() {
                            string.get(..usize::from(self.width)).unwrap()
                        } else { string };
                        result.push_str(string);
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

pub enum BorderStyle {
    Gap(Cell),
    Connected2(Static, Static),
    // Disconnected2(Static, Static),
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
			borders: &BorderStyle) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let (mut string, arr) = self.render_sub(start_col + 1, start_row + 1, width - 2, height - 2, borders, true);
        let mut canvas = InstructionBuffer::new(width, height, None);

        match borders {
            BorderStyle::Connected2(borderh, borderv) => {
                canvas.draw_hbar(1, 0, width - 2, borderh, None);
                canvas.draw_hbar(1, height - 1, width - 2, borderh, None);
                canvas.draw_vbar(0, 0, height, borderv, None);
                canvas.draw_vbar(width - 1, 0, height, borderv, None);
            }
            BorderStyle::Gap(..) => {}
            // BorderStyle::Disconnected2(..) => {}
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
			borders: &BorderStyle,
            first: bool) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let mut arr = Vec::new();
        let mut string = String::new();

        match self {
            Self::Pane(pane) => {
                arr.push((pane, start_col, start_row, width, height));

                match borders {
                    // BorderStyle::Disconnected2(borderh, borderv) => {
                    //     let (width, height) = (width + 2, height + 2);
                    //     let mut canvas = InstructionBuffer::new(width, height, None);
                    //     canvas.draw_hbar(1, 0, width - 2, borderh, None);
                    //     canvas.draw_hbar(1, height - 1, width - 2, borderh, None);
                    //     canvas.draw_vbar(0, 0, height, borderv, None);
                    //     canvas.draw_vbar(width - 1, 0, height, borderv, None);
                    //     string.push_str(&canvas.render(start_col - 1, start_row - 1));
                    // }
                    BorderStyle::Connected2(..) => {}
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

                let gap = match *borders {
                    BorderStyle::Gap(cells) => cells,
                    BorderStyle::Connected2(..) => 1,
                    // BorderStyle::Disconnected2(..) => 2,
                };

                let mut canvas = InstructionBuffer::new(width + 1, height + 1, None);

                let mut i = 0;
                for (j, (size, paner)) in paners.iter().enumerate() {
                    let size = match size {
                        PaneSize::Relative(size) => {
                            let size = ((if horizontal { width } else { height })
                                .saturating_sub(total_fixed) * size / total_rel)
                                .saturating_sub(gap);

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
                        borders,
                        false
                    );
                    string.push_str(&new_string);
                    arr.append(&mut new_arr);

                    if 0 < j {
                        match borders {
                            BorderStyle::Connected2(borderh, borderv) => {
                                let extra = if first { 0 } else { 0 };
                                if horizontal { canvas.draw_vbar(i - 1, 0, height + extra, borderv, None) }
                                else { canvas.draw_hbar(0, i - 1, width + extra, borderh, None) }
                            }
                            BorderStyle::Gap(..) => {}
                            // BorderStyle::Disconnected2(..) => {}
                        }
                    }

                    i += size + gap;
                }

                string.push_str(&canvas.render(start_col, start_row));
            }
        }

        (string, arr)
    }
}
