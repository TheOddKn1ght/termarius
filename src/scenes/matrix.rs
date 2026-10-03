use super::{Scene, common::ParticleField};
use crate::render::Frame;

#[derive(Default)]
pub(super) struct Matrix {
    field: ParticleField,
}

impl Scene for Matrix {
    fn reset(&mut self, width: u16, height: u16, seed: u64) {
        self.field =
            ParticleField::new(width, height, seed, width as usize / 2, |index, p, rng| {
                p.x = (index * 2) as f32;
                p.speed = 5.0 + rng.unit() * 12.0;
            });
    }
    fn update(&mut self, dt: f32) {
        self.field.time += dt;
        for p in &mut self.field.particles {
            p.y += p.speed * dt;
            if p.y > self.field.height as f32 + 15.0 {
                p.y = -self.field.rng.unit() * self.field.height as f32;
            }
        }
    }
    fn render(&self, frame: &mut Frame, _ascii: bool) {
        const CODE: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ:;<>+=*";
        for (i, p) in self.field.particles.iter().enumerate() {
            let head = p.y as i32;
            let length = 5 + p.size as i32 * 4;
            for tail in 0..length {
                let y = head - tail;
                let hash = (i as u64).wrapping_mul(7919)
                    ^ (y as u64).wrapping_mul(104729)
                    ^ (self.field.time * 8.0) as u64;
                frame.put(
                    p.x as i32,
                    y,
                    CODE[(hash % CODE.len() as u64) as usize] as char,
                    if tail == 0 {
                        255
                    } else if tail < 3 {
                        46
                    } else if tail < length / 2 {
                        34
                    } else {
                        22
                    },
                );
            }
        }
    }
}
