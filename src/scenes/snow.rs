use super::{Scene, common::ParticleField};
use crate::render::Frame;

#[derive(Default)]
pub(super) struct Snow {
    field: ParticleField,
}

impl Scene for Snow {
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
            p.y += dt * p.speed * 0.5;
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
        let floor = self.field.height as i32 - 1;
        for x in 0..self.field.width as i32 {
            let depth = 1 + ((x as f32 * 0.12).sin() * 1.5 + 1.5) as i32;
            for j in 0..depth {
                frame.put(x, floor - j, if ascii { '_' } else { '░' }, 250);
            }
        }
        for p in &self.field.particles {
            let x = (p.x + (self.field.time * 0.5 + p.phase).sin() * 3.0) as i32;
            frame.put(
                x,
                p.y as i32,
                match p.size {
                    0 => {
                        if ascii {
                            '.'
                        } else {
                            '·'
                        }
                    }
                    1 => '*',
                    _ => '+',
                },
                [240, 250, 255][p.size],
            );
        }
    }
}
