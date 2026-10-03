use crate::cli::ColorMode;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    style::{Color, ResetColor, SetBackgroundColor},
    terminal::{self, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

pub(crate) fn configure_color(mode: ColorMode) -> ColorMode {
    let color = resolve_color(mode);
    crossterm::style::force_color_output(color != ColorMode::None);
    color
}

fn resolve_color(mode: ColorMode) -> ColorMode {
    if mode != ColorMode::Auto {
        return mode;
    }
    if std::env::var_os("NO_COLOR").is_some() || std::env::var("TERM").is_ok_and(|v| v == "dumb") {
        return ColorMode::None;
    }
    if std::env::var("TERM").is_ok_and(|v| v.contains("256color"))
        || std::env::var_os("COLORTERM").is_some()
    {
        ColorMode::Ansi256
    } else {
        ColorMode::Ansi16
    }
}

pub(crate) struct Terminal;
impl Terminal {
    pub(crate) fn enter() -> io::Result<Self> {
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            Self::restore();
            old_hook(info);
        }));
        let guard = Self;
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            DisableLineWrap,
            Hide,
            SetBackgroundColor(Color::Black)
        )?;
        Ok(guard)
    }
    fn restore() {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            ResetColor,
            Show,
            EnableLineWrap,
            LeaveAlternateScreen
        );
    }
}
impl Drop for Terminal {
    fn drop(&mut self) {
        Self::restore();
    }
}
