use crate::cli::ColorMode;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub color: u8,
    pub background: u8,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            color: 7,
            background: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}
impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width as usize * height as usize],
        }
    }
    pub fn put(&mut self, x: i32, y: i32, ch: char, color: u8) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.cells[y as usize * self.width as usize + x as usize] = Cell {
                ch,
                color,
                background: 0,
            };
        }
    }
    pub fn halves(&mut self, x: i32, y: i32, top: u8, bottom: u8) {
        self.put(x, y, '▀', top);
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.cells[y as usize * self.width as usize + x as usize].background = bottom;
        }
    }
    pub fn text(&mut self, x: i32, y: i32, text: &str, color: u8) {
        for (i, ch) in text.chars().enumerate() {
            self.put(x + i as i32, y, ch, color);
        }
    }
    pub fn centered(&mut self, y: i32, text: &str, color: u8) {
        self.text(
            ((self.width as i32 - text.chars().count() as i32) / 2).max(0),
            y,
            text,
            color,
        );
    }
}

pub struct Renderer {
    previous: Option<Frame>,
    mode: ColorMode,
}
impl Renderer {
    pub fn new(mode: ColorMode) -> Self {
        Self {
            previous: None,
            mode,
        }
    }
    pub fn draw(&mut self, out: &mut impl Write, frame: Frame) -> io::Result<()> {
        let mut buffer = Vec::new();
        let resized = self
            .previous
            .as_ref()
            .is_none_or(|p| p.width != frame.width || p.height != frame.height);
        queue!(buffer, Print("\x1b[40m"))?;
        if resized {
            queue!(buffer, Clear(ClearType::All))?;
        }
        let mut active_color = None;
        let mut active_background = 0;
        for (i, cell) in frame.cells.iter().enumerate() {
            if !resized && self.previous.as_ref().is_some_and(|p| p.cells[i] == *cell) {
                continue;
            }
            queue!(
                buffer,
                MoveTo(
                    (i % frame.width as usize) as u16,
                    (i / frame.width as usize) as u16
                )
            )?;
            if self.mode != ColorMode::None && active_color != Some(cell.color) {
                self.write_color(&mut buffer, cell.color, false)?;
                active_color = Some(cell.color);
            }
            if self.mode != ColorMode::None && active_background != cell.background {
                self.write_color(&mut buffer, cell.background, true)?;
                active_background = cell.background;
            }
            queue!(buffer, Print(cell.ch))?;
        }
        queue!(buffer, ResetColor)?;
        out.write_all(&buffer)?;
        out.flush()?;
        self.previous = Some(frame);
        Ok(())
    }
    fn write_color(&self, buffer: &mut Vec<u8>, index: u8, background: bool) -> io::Result<()> {
        let color = self.color(index);
        if self.mode == ColorMode::Ansi16 {
            let code = match color {
                Color::Black => 30,
                Color::DarkRed => 31,
                Color::DarkGreen => 32,
                Color::DarkYellow => 33,
                Color::DarkBlue => 34,
                Color::DarkMagenta => 35,
                Color::DarkCyan => 36,
                Color::Grey => 37,
                Color::DarkGrey => 90,
                Color::Red => 91,
                Color::Green => 92,
                Color::Yellow => 93,
                Color::Blue => 94,
                Color::Magenta => 95,
                Color::Cyan => 96,
                _ => 97,
            } + if background { 10 } else { 0 };
            write!(buffer, "\x1b[{code}m")?;
        } else if background {
            queue!(buffer, SetBackgroundColor(color))?;
        } else {
            queue!(buffer, SetForegroundColor(color))?;
        }
        Ok(())
    }
    fn color(&self, index: u8) -> Color {
        if self.mode != ColorMode::Ansi16 {
            return Color::AnsiValue(index);
        }
        match index {
            0 => Color::Black,
            1 | 52 | 88 | 124 | 160 => Color::DarkRed,
            2 | 22 | 28 => Color::DarkGreen,
            3 | 94 | 130 => Color::DarkYellow,
            4 | 17 | 18 | 19 | 24 => Color::DarkBlue,
            5 | 54 | 90 => Color::DarkMagenta,
            6 | 23 | 30 => Color::DarkCyan,
            8 | 236..=244 => Color::DarkGrey,
            9 | 196 | 202 | 203 => Color::Red,
            10 | 34 | 40 | 46 | 82 => Color::Green,
            11 | 166 | 172 | 208 | 214 | 220 | 226 | 227 | 228 | 229 => Color::Yellow,
            12 | 25 | 27 | 33 | 39 | 69 | 75 => Color::Blue,
            13 | 135 | 141 | 177 | 213 => Color::Magenta,
            14 | 37 | 44 | 45 | 51 | 81 | 117 | 159 => Color::Cyan,
            15 | 230 | 255 => Color::White,
            _ => Color::Grey,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn half_cells_update_background_and_restore_black_for_text() {
        let mut renderer = Renderer::new(ColorMode::Ansi16);
        let mut frame = Frame::new(2, 1);
        frame.halves(0, 0, 214, 160);
        frame.put(1, 0, 'x', 7);
        let mut output = vec![];
        renderer.draw(&mut output, frame.clone()).unwrap();
        let text = String::from_utf8(output.clone()).unwrap();
        assert!(text.contains("\x1b[41m▀"));
        assert!(text.contains("\x1b[40mx"));
        output.clear();
        frame.halves(0, 0, 214, 28);
        renderer.draw(&mut output, frame).unwrap();
        assert!(String::from_utf8(output).unwrap().contains("\x1b[42m▀"));
    }
    #[test]
    fn redraws_changes_and_clears_on_resize() {
        let mut renderer = Renderer::new(ColorMode::None);
        let mut frame = Frame::new(5, 2);
        frame.put(1, 0, 'x', 7);
        let mut output = vec![];
        renderer.draw(&mut output, frame.clone()).unwrap();
        assert!(output.windows(4).any(|s| s == b"\x1b[2J"));
        output.clear();
        renderer.draw(&mut output, frame.clone()).unwrap();
        assert_eq!(output, b"\x1b[40m\x1b[0m");
        output.clear();
        frame.put(1, 0, ' ', 7);
        renderer.draw(&mut output, frame).unwrap();
        assert_eq!(output, b"\x1b[40m\x1b[1;2H \x1b[0m");
        output.clear();
        renderer.draw(&mut output, Frame::new(0, 0)).unwrap();
        assert!(output.windows(4).any(|s| s == b"\x1b[2J"));
    }
    #[test]
    fn sixteen_color_palette_uses_named_colors() {
        let renderer = Renderer::new(ColorMode::Ansi16);
        for index in 0..=255 {
            assert!(!matches!(
                renderer.color(index),
                Color::AnsiValue(_) | Color::Rgb { .. }
            ));
        }
        let mut renderer = Renderer::new(ColorMode::Ansi16);
        let mut frame = Frame::new(1, 1);
        frame.put(0, 0, '*', 46);
        let mut output = vec![];
        renderer.draw(&mut output, frame).unwrap();
        assert!(output.windows(5).any(|s| s == b"\x1b[92m"));
        assert!(!output.windows(7).any(|s| s == b"\x1b[38;5;"));
    }
}
