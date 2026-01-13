pub mod control_sequences;
pub mod input_sequences;

use control_sequences::*;

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
    fn addstr(&mut self, col: Cell, row: Cell, string: &str, style: Option<&Style>);
    fn render(&self, col: Cell, row: Cell) -> Str;

    fn draw_hbar(&mut self, col: Cell, row: Cell, length: Cell, ch: &str, style: Option<&Style>) {
        self.addstr(col, row, &ch.repeat(length.into()), style);
    }

    fn draw_vbar(&mut self, col: Cell, row: Cell, length: Cell, ch: &str, style: Option<&Style>) {
        // TODO use CUR_DOWN_ONE instead! see todo below
        for i in 0..length {
            self.addstr(col, row + i, ch, style);
        }
    }

    // TODO create required addtext and addcmd, then derive addstr from those
    // this would make it possible to add initial methods that utilize commands
    // other than CUR_SET and style commands
    // to do this, Instruction will have to be replaced with an enum
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
    Color256(u8),
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
            Color::Color256(..) | Color::True(..) => {
                match self {
                    Color::Color256(id) => return formatf!("{FG_ID}", id),
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
            Color::Color256(..) | Color::True(..) => {
                match self {
                    Color::Color256(id) => return formatf!("{BG_ID}", id),
                    Color::True(r, g, b) => return formatf!("{BG_RGB}", r, g, b),
                    _ => unreachable!()
                }
            }
        }.to_string()
    }
}

#[derive(Clone)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
}

// TODO this thing is pretty ugly.
static EMPTY_STYLE: Style = Style::new();

impl Style {
    pub const EMPTY: &'static Style = &EMPTY_STYLE;

    pub const fn new() -> Self {
        Self {
            fg: Some(Color::Default),
            bg: Some(Color::Default),
        }
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn get_fg(&self) -> Option<&Color> {
        self.fg.as_ref()
    }

    pub fn get_bg(&self) -> Option<&Color> {
        self.bg.as_ref()
    }

    // TODO create with_(fg|bg) that clones self and replaces fg or bg

    pub fn as_string(&self) -> Str {
        let mut string = String::new();

        if let Some(fg) = &self.fg { string.push_str(&fg.as_string_fg()) }
        if let Some(bg) = &self.bg { string.push_str(&bg.as_string_bg()) }

        string
    }
}

// (row, col, string, style)
type Instruction = (Cell, Cell, Str, Str);

/// Simply remembers the order of addstr() calls.
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

impl Canvas for InstructionBuffer<'_> {
    fn clear(&mut self) {
        self.instructions.clear();
    }

    fn addstr(&mut self, col: Cell, row: Cell, string: &str, style: Option<&Style>) {
        let style = match style {
            Some(style) => style,
            None => self.default_style
        };
        self.instructions.push((row, col, string.to_string(), style.as_string()));
    }

    fn render(&self, start_col: Cell, start_row: Cell) -> Str {
        let mut result = String::new();

        for (row, col, string, style) in self.instructions.iter() {
            // truncate if too long
            let string = if usize::from(self.width) < string.len() {
                string.get(..usize::from(self.width)).unwrap()
            } else { string };

            result.push_str(&(formatf!(
                "{CUR_SET}{{}}{{}}{{}}",
                start_row + row + 1, start_col + col + 1,
                style, &string, self.default_style.as_string()
            )));

            // skip remaining rows if limit is reached
            if *row == self.height {
                break
            }
        }

        result
    }
}

pub enum Paner<T> {
    Pane(T),
    Horizontal(Vec<(Cell, Paner<T>)>),
    Vertical(Vec<(Cell, Paner<T>)>),
}

pub enum BorderStyle {
    Gap(Cell),
    Connected2 { borderh: Static, borderv: Static },
}

impl<T> Paner<T> {
    pub fn render(
            &self,
			mut start_col: Cell,
			mut start_row: Cell,
			mut width: Cell,
			mut height: Cell,
			borders: &BorderStyle) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let (mut string, arr) = self.render_sub(start_col + 1, start_row + 1, width - 2, height - 2, borders, true);
        let mut canvas = InstructionBuffer::new(width, height, None);

        match borders {
            BorderStyle::Connected2 { borderh, borderv } => {
                canvas.draw_hbar(1, 0, width - 2, borderh, None);
                canvas.draw_hbar(1, height - 1, width - 2, borderh, None);
                canvas.draw_vbar(0, 0, height, borderv, None);
                canvas.draw_vbar(width - 1, 0, height, borderv, None);
            }
            BorderStyle::Gap(..) => {}
        }

        string.push_str(&canvas.render(0, 0));
        (string, arr)
    }

    fn render_sub(
            &self,
			mut start_col: Cell,
			mut start_row: Cell,
			mut width: Cell,
			mut height: Cell,
			borders: &BorderStyle,
            first: bool) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let mut arr = Vec::new();
        let mut string = String::new();

        match self {
            Self::Pane(pane) => {
                arr.push((pane, start_col, start_row, width, height));
            }
            Self::Horizontal(paners) | Self::Vertical(paners) => {
                let total: Cell = paners.iter().map(|i| i.0).sum();
                let horizontal = match self {
                    Self::Horizontal(..) => true,
                    _ => false
                };

                let gap = match *borders {
                    BorderStyle::Gap(cells) => cells,
                    BorderStyle::Connected2 { .. } => 1,
                };

                // let (init_start_col, init_start_row) = (start_col, start_row);

                // if horizontal {
                //     start_col += gap;
                //     width -= gap;
                //     // height -= 1;
                // } else {
                //     start_row += gap;
                //     height -= gap;
                //     // width -= 1;
                // }

                let mut canvas = InstructionBuffer::new(width + 1, height + 1, None);

                let mut i = 0;
                for (j, (size, paner)) in paners.iter().enumerate() {
                    let size = if horizontal { width } else { height } * size / total - gap;

                    // extend the last element if it can not be perfect
                    // TODO improve this and make it even
                    let size = size + if j == paners.len() - 1 {
                        (if horizontal { width } else { height }) - size - i
                    } else { 0 };

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
                            BorderStyle::Connected2 { borderh, borderv } => {
                                let extra = if first { 0 } else { 0 };
                                if horizontal { canvas.draw_vbar(i - 1, 0, height + extra, borderv, None) }
                                else { canvas.draw_hbar(0, i - 1, width + extra, borderh, None) }
                            }
                            BorderStyle::Gap(..) => {}
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
