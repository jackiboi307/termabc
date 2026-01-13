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

pub trait Canvas {
    fn clear(&mut self);
    fn addstr(&mut self, col: Cell, row: Cell, string: &str, style: Option<&Style>);
    fn render(&self, col: Cell, row: Cell, max_width: Option<Cell>, max_height: Option<Cell>) -> Str;
    fn background(&self) -> Option<Color> { Some(Color::Red) }
}

#[derive(Default)]
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
    fn as_string_fg(&self) -> Str {
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

    fn as_string_bg(&self) -> Str {
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

#[derive(Default)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(&mut self, fg: Color) -> &mut Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(&mut self, bg: Color) -> &mut Self {
        self.bg = Some(bg);
        self
    }

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
pub struct SimpleInstructionBuffer {
    instructions: Vec<Instruction>,
}

impl SimpleInstructionBuffer {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

impl Canvas for SimpleInstructionBuffer {
    fn clear(&mut self) {
        self.instructions.clear();
    }

    fn addstr(&mut self, col: Cell, row: Cell, string: &str, style: Option<&Style>) {
        let style = match style {
            Some(style) => style.as_string(),
            None => String::new()
        };
        self.instructions.push((row, col, string.to_string(), style));
    }

    fn render(&self, start_col: Cell, start_row: Cell, max_width: Option<Cell>, max_height: Option<Cell>) -> Str {
        let mut result = String::new();

        for (row, col, string, style) in self.instructions.iter() {
            // truncate if too long
            let string = if let Some(max_width) = max_width {
                if usize::from(max_width) < string.len() {
                    string.get(..usize::from(max_width)).unwrap()
                } else { string }
            } else { string };

            result.push_str(&(
                formatf!("{CUR_SET}", start_row + row + 1, start_col + col + 1)
                + style + &string
            ));

            // skip remaining rows if limit is reached
            if let Some(max_height) = max_height {
                if *row == max_height {
                    break
                }
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

impl<T> Paner<T> {
    // pub fn render(&self, start_col: Cell, start_row: Cell, width: Cell, height: Cell, border: Box<[&str]>) -> Str {

    // }
    
    pub fn render(
            &self,
			start_col: Cell,
			start_row: Cell,
			width: Cell,
			height: Cell,
			border: &[&str]) -> (Str, Vec<(&T, Cell, Cell, Cell, Cell)>) {

        let mut arr = Vec::new();
        let mut string = String::new();

        match self {
            Self::Pane(pane) => {
                // result.push_str(&canvas.render(start_col, start_row, Some(width), Some(height)));
                arr.push((pane, start_col, start_row, width, height));
            },
            Self::Horizontal(paners) | Self::Vertical(paners) => {
                let total: Cell = paners.iter().map(|i| i.0).sum();
                // let total = total - paners.len() as Cell + 1;
                let horizontal = match self {
                    Self::Horizontal(..) => true,
                    _ => false
                };

                let mut i = 0;
                for (j, (size, paner)) in paners.iter().enumerate() {
                    let size = if horizontal { width } else { height } * size / total - 1;

                    // extend the last element if it can not be perfect
                    let size = size + if j == paners.len() - 1 {
                        (if horizontal { width } else { height }) - size - i
                    } else { 0 };

                    // result.push_str(&paner.render(
                    let (new_string, mut new_arr) = paner.render(
                        if horizontal { start_col + i } else { start_col },
                        if horizontal { start_row } else { start_row + i },
                        if horizontal { size } else { width },
                        if horizontal { height } else { size },
                        border
                    );
                    string.push_str(&new_string);
                    arr.append(&mut new_arr);
                    i += size + 1;
                }
            }
        }

        (string, arr)
    }
}
