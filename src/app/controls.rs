use super::App;
use crate::scenes::Kind;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    pub(super) fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.screensaver
            || key.code == KeyCode::Char('q')
            || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
        {
            return true;
        }
        if key.code == KeyCode::Esc {
            if self.help {
                self.help = false;
            } else {
                return true;
            }
        } else if self.picker {
            self.picker_key(key.code);
        } else {
            self.viewer_key(key.code);
        }
        false
    }

    fn picker_key(&mut self, key: KeyCode) {
        let count = Kind::ALL.len();
        match key {
            KeyCode::Up | KeyCode::Char('k') => self.selected = (self.selected + count - 1) % count,
            KeyCode::Down | KeyCode::Char('j') => self.selected = (self.selected + 1) % count,
            KeyCode::Enter => {
                self.change_scene(Kind::ALL[self.selected]);
                self.picker = false;
                self.paused = false;
                self.help = false;
            }
            _ => {}
        }
    }

    fn viewer_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('?') => self.help = !self.help,
            KeyCode::Char('m') => {
                self.picker = true;
                self.selected = self.kind.index();
                self.help = false;
            }
            KeyCode::Char(' ') => self.paused = !self.paused,
            KeyCode::Char('+') | KeyCode::Char('=') => self.speed = (self.speed * 1.25).min(4.0),
            KeyCode::Char('-') => self.speed = (self.speed / 1.25).max(0.25),
            KeyCode::Char('r') => {
                self.seed = self.random.next();
                self.reset_scene();
            }
            KeyCode::Right | KeyCode::Down | KeyCode::Left | KeyCode::Up => {
                let count = Kind::ALL.len();
                let delta = if matches!(key, KeyCode::Right | KeyCode::Down) {
                    1
                } else {
                    count - 1
                };
                self.change_scene(Kind::ALL[(self.kind.index() + delta) % count]);
                self.help = false;
            }
            _ => {}
        }
    }
}
