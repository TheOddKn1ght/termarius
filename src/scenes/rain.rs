use super::{Scene, common::ParticleField};
use crate::render::Frame;

#[derive(Default)]
pub(super) struct Rain {
    field: ParticleField,
}

impl Scene for Rain {
    fn reset(&mut self, width: u16, height: u16, seed: u64) {
        self.field = ParticleField::new(
            width,
            height,
            seed,
            (width as usize * height as usize / 25).clamp(8, 800),
            |_, _, _| {},
        );
    }
    fn update(&mut self, dt: f32) {
        self.field.time += dt;
        for p in &mut self.field.particles {
            p.y += dt * p.speed * 5.0;
            p.x -= dt * p.speed * 1.5;
            if p.y > self.field.height as f32 + 1.0 {
                p.y = -3.0;
                p.x = self.field.rng.unit() * self.field.width as f32;
            }
            if p.x < 0.0 {
                p.x += self.field.width as f32;
            }
        }
    }
    fn render(&self, frame: &mut Frame, ascii: bool) {
        let floor = self.field.height as i32 - 2;
        for p in &self.field.particles {
            let x = p.x as i32;
            let y = p.y as i32;
            if y >= floor {
                frame.text(x - 1, floor, if ascii { "_o_" } else { "⌒·⌒" }, 24);
            } else {
                for j in 0..=p.size {
                    frame.put(
                        x + j as i32,
                        y - j as i32,
                        if ascii { '/' } else { '╱' },
                        [24, 33, 75][p.size],
                    );
                }
            }
        }
        for x in 0..self.field.width as i32 {
            if ((x as f32 * 0.4 + self.field.time * 2.0).sin()).abs() > 0.65 {
                frame.put(x, floor + 1, '~', 17);
            }
        }
    }
}
