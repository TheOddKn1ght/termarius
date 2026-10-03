use super::{Scene, common::ParticleField};
use crate::render::Frame;

#[derive(Default)]
pub(super) struct Stars {
    field: ParticleField,
}

impl Scene for Stars {
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
    }
    fn render(&self, frame: &mut Frame, ascii: bool) {
        for p in &self.field.particles {
            let light = (self.field.time * p.speed * 0.25 + p.phase).sin();
            let ch = if light > 0.7 {
                '+'
            } else if light > 0.1 {
                '*'
            } else if ascii {
                '.'
            } else {
                '·'
            };
            frame.put(
                p.x as i32,
                p.y as i32,
                ch,
                if light > 0.7 {
                    255
                } else if light > 0.1 {
                    250
                } else {
                    240
                },
            );
        }
        let cycle = self.field.time % 12.0;
        if cycle < 1.6 {
            let x = (cycle * self.field.width as f32 * 0.8) as i32;
            let y = (cycle * self.field.height as f32 * 0.25) as i32;
            frame.put(x, y, '*', 255);
            for i in 1..6 {
                frame.put(
                    x - i * 2,
                    y - i,
                    if ascii { '.' } else { '·' },
                    250 - i as u8 * 2,
                );
            }
        }
    }
}
