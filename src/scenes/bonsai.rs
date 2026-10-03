use super::Scene;
use crate::{render::Frame, rng::Rng};

struct Branch {
    x: i32,
    y: i32,
    ch: char,
    color: u8,
}

#[derive(Default)]
pub(super) struct Bonsai {
    width: u16,
    height: u16,
    rng: Rng,
    branches: Vec<Branch>,
    growth: f32,
}
impl Bonsai {
    fn tree(&mut self, x: f32, y: f32, angle: f32, length: f32, depth: u8) {
        if depth == 0 || length < 1.0 {
            for dy in -1..=1 {
                for dx in -2..=2 {
                    if self.rng.range(4) != 0 {
                        self.branches.push(Branch {
                            x: x as i32 + dx,
                            y: y as i32 + dy,
                            ch: '*',
                            color: [28, 34, 40, 82][self.rng.range(4)],
                        });
                    }
                }
            }
            return;
        }
        let steps = (length * 2.0).ceil() as usize;
        let end_x = x + angle.cos() * length * 1.8;
        let end_y = y + angle.sin() * length;
        let ch = if angle.cos().abs() < 0.25 {
            '|'
        } else if angle.cos() > 0.0 {
            '/'
        } else {
            '\\'
        };
        for i in 0..=steps {
            let t = i as f32 / steps.max(1) as f32;
            self.branches.push(Branch {
                x: (x + (end_x - x) * t) as i32,
                y: (y + (end_y - y) * t) as i32,
                ch,
                color: 130,
            });
        }
        let spread = 0.35 + self.rng.unit() * 0.35;
        let shrink = 0.62 + self.rng.unit() * 0.1;
        self.tree(end_x, end_y, angle - spread, length * shrink, depth - 1);
        self.tree(end_x, end_y, angle + spread, length * shrink, depth - 1);
    }
}

impl Scene for Bonsai {
    fn reset(&mut self, width: u16, height: u16, seed: u64) {
        self.width = width;
        self.height = height;
        self.rng = Rng::new(seed);
        self.growth = 0.0;
        self.branches.clear();
        let length = (height as f32 * 0.26).min(width as f32 * 0.12).max(1.0);
        self.tree(
            width as f32 / 2.0,
            height as f32 - 3.0,
            -std::f32::consts::FRAC_PI_2,
            length,
            5,
        );
    }
    fn update(&mut self, dt: f32) {
        self.growth += dt * 45.0;
    }
    fn render(&self, frame: &mut Frame, ascii: bool) {
        let visible = (self.growth as usize).min(self.branches.len());
        for b in &self.branches[..visible] {
            frame.put(
                b.x,
                b.y,
                if !ascii && b.ch == '*' { '♣' } else { b.ch },
                b.color,
            );
        }
        let x = self.width as i32 / 2;
        let y = self.height as i32 - 2;
        frame.text(x - 6, y, "\\__________/", 94);
        frame.text(x - 5, y + 1, "\\________/", 130);
    }
}
