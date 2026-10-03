use super::{Scene, common::ParticleField};
use crate::render::Frame;

#[derive(Default)]
pub(super) struct Underwater {
    field: ParticleField,
}

impl Scene for Underwater {
    fn reset(&mut self, width: u16, height: u16, seed: u64) {
        self.field = ParticleField::new(
            width,
            height,
            seed,
            (width as usize * height as usize / 150).clamp(6, 35),
            |_, p, rng| {
                p.y = 2.0 + rng.unit() * (height as f32 - 6.0).max(1.0);
            },
        );
    }
    fn update(&mut self, dt: f32) {
        self.field.time += dt;
        let w = self.field.width as f32;
        let h = self.field.height as f32;
        for (i, p) in self.field.particles.iter_mut().enumerate() {
            if i % 3 == 0 {
                p.y -= dt * p.speed * 0.3;
                if p.y < 1.0 {
                    p.y = h - 2.0;
                }
            } else {
                p.x += dt * p.speed * if i % 2 == 0 { 1.0 } else { -1.0 };
                if p.x > w + 8.0 {
                    p.x = -8.0;
                }
                if p.x < -8.0 {
                    p.x = w;
                }
            }
        }
    }
    fn render(&self, frame: &mut Frame, ascii: bool) {
        let floor = self.field.height as i32 - 1;
        for x in 0..self.field.width as i32 {
            frame.put(x, floor, if x % 7 == 0 { ':' } else { '.' }, 94);
            if x % 9 == 0 {
                let height = 3 + (x % 5);
                for j in 1..=height {
                    let sway = (self.field.time + x as f32 * 0.2 + j as f32 * 0.4)
                        .sin()
                        .round() as i32;
                    frame.put(
                        x + sway,
                        floor - j,
                        if ascii { ')' } else { '≈' },
                        if j % 2 == 0 { 28 } else { 34 },
                    );
                }
            }
        }
        for (i, p) in self.field.particles.iter().enumerate() {
            if i % 3 == 0 {
                frame.put(p.x as i32, p.y as i32, if ascii { 'o' } else { '°' }, 24);
            } else {
                let right = i % 2 == 0;
                let fish = match (right, p.size) {
                    (true, 0) => "><>",
                    (false, 0) => "<><",
                    (true, _) => "><(((°>",
                    (false, _) => "<°)))><",
                };
                let fish = if ascii {
                    match (right, p.size) {
                        (true, 0) => "><>",
                        (false, 0) => "<><",
                        (true, _) => "><(((o>",
                        (false, _) => "<o)))><",
                    }
                } else {
                    fish
                };
                frame.text(
                    p.x as i32,
                    (p.y + (self.field.time + p.phase).sin() * 0.6) as i32,
                    fish,
                    [39, 75, 214][p.size],
                );
            }
        }
        for x in 0..self.field.width as i32 {
            if (x as f32 * 0.2 + self.field.time).sin() > 0.75 {
                frame.put(x, 0, '~', 24);
            }
        }
    }
}
