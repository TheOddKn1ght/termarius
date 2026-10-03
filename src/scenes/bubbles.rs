use super::{Scene, common::ParticleField};
use crate::render::Frame;

#[derive(Default)]
pub(super) struct Bubbles {
    field: ParticleField,
}

impl Scene for Bubbles {
    fn reset(&mut self, width: u16, height: u16, seed: u64) {
        self.field = ParticleField::new(
            width,
            height,
            seed,
            (width as usize * height as usize / 100).clamp(5, 80),
            |_, _, _| {},
        );
    }
    fn update(&mut self, dt: f32) {
        self.field.time += dt;
        for p in &mut self.field.particles {
            p.y -= p.speed * dt;
            if p.y < -4.0 {
                p.y = self.field.height as f32 + 2.0;
                p.x = self.field.rng.unit() * self.field.width as f32;
            }
        }
    }
    fn render(&self, frame: &mut Frame, ascii: bool) {
        for p in &self.field.particles {
            let x = (p.x + (self.field.time * 0.7 + p.phase).sin() * 2.0) as i32;
            let y = p.y as i32;
            let color = [45, 81, 159][p.size];
            match p.size {
                0 => frame.put(x, y, if ascii { 'o' } else { '°' }, color),
                1 => {
                    frame.text(x, y, "()", color);
                }
                _ => {
                    frame.text(x, y - 1, " .-. ", color);
                    frame.text(x, y, "(   )", color);
                    frame.text(x, y + 1, " `-' ", color);
                }
            }
        }
    }
}
