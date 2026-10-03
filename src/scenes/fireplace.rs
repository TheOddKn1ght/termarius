use super::{Scene, common::Particle};
use crate::{render::Frame, rng::Rng};

#[derive(Default)]
pub(super) struct Fireplace {
    width: u16,
    height: u16,
    rng: Rng,
    particles: Vec<Particle>,
    heat: Vec<f32>,
    fire_steps: f32,
    fire_tick: u64,
}
impl Fireplace {
    fn fire_color(heat: f32) -> u8 {
        match heat {
            h if h < 0.07 => 0,
            h if h < 0.12 => 52,
            h if h < 0.18 => 88,
            h if h < 0.27 => 124,
            h if h < 0.36 => 160,
            h if h < 0.46 => 196,
            h if h < 0.56 => 202,
            h if h < 0.66 => 208,
            h if h < 0.76 => 214,
            h if h < 0.85 => 220,
            h if h < 0.93 => 227,
            _ => 230,
        }
    }
    fn fire_step(&mut self) {
        let width = self.width as usize;
        let base = self.height.saturating_sub(3) as usize * 2;
        if width == 0 || base == 0 {
            return;
        }
        let t = self.fire_tick as f32 / 30.0;
        for y in 0..base.saturating_sub(1) {
            for x in 0..width {
                let wind =
                    (t * 1.7 - y as f32 * 0.16).sin() + (x as f32 * 0.18 + t * 0.8).sin() * 0.7;
                let source_x = (x as i32 + wind.round() as i32).clamp(0, width as i32 - 1) as usize;
                let below = (y + 1) * width;
                let further = (y + 2).min(base - 1) * width;
                let heat = self.heat[below + source_x] * 0.55
                    + self.heat[below + source_x.saturating_sub(1)] * 0.15
                    + self.heat[below + (source_x + 1).min(width - 1)] * 0.15
                    + self.heat[further + source_x] * 0.15;
                let curl = (x as f32 * 0.32 + y as f32 * 0.20 - t * 2.7).sin()
                    * (x as f32 * 0.13 - y as f32 * 0.31 + t * 1.8).sin();
                let cooling =
                    (1.2 + curl * 0.65) / (base as f32).max(8.0) + self.rng.unit() * 0.012;
                self.heat[y * width + x] = (heat - cooling).max(0.0);
            }
        }
        for x in 0..width {
            let nx = (x as f32 - width as f32 * 0.5) / (width as f32 * 0.27).max(1.0);
            let envelope = (1.0 - nx * nx).max(0.0);
            let tongues = (x as f32 * 0.38 + t * 0.8).sin().powi(2) * 0.5
                + (x as f32 * 0.21 - t * 1.3).sin() * 0.15;
            self.heat[(base - 1) * width + x] =
                (envelope * (0.52 + tongues + self.rng.unit() * 0.18)).clamp(0.0, 1.0);
        }
        for p in &mut self.particles {
            p.y -= p.speed / 30.0;
            p.x += ((t * 1.1 + p.phase).sin() * 1.8 + 0.4) / 30.0;
            if p.y < self.height as f32 * 0.12 {
                p.y = self.height as f32 - 3.0;
                p.x = width as f32 * (0.35 + self.rng.unit() * 0.3);
                p.speed = 2.0 + self.rng.unit() * 4.0;
            }
        }
        self.fire_tick += 1;
    }
}

impl Scene for Fireplace {
    fn reset(&mut self, width: u16, height: u16, seed: u64) {
        self.width = width;
        self.height = height;
        self.rng = Rng::new(seed);
        self.fire_steps = 0.0;
        self.fire_tick = 0;
        self.heat = vec![0.0; width as usize * height as usize * 2];
        self.particles.clear();
        for _ in 0..(width as usize / 6).clamp(3, 30) {
            let mut p = Particle::random(&mut self.rng, width, height);
            p.x = width as f32 * (0.35 + self.rng.unit() * 0.3);
            p.y = height as f32 - 3.0 - self.rng.unit() * height as f32 * 0.55;
            p.speed = 2.0 + self.rng.unit() * 4.0;
            self.particles.push(p);
        }
        for _ in 0..60 {
            self.fire_step();
        }
    }
    fn update(&mut self, dt: f32) {
        self.fire_steps += dt * 30.0;
        let iterations = self.fire_steps.floor() as usize;
        self.fire_steps -= iterations as f32;
        for _ in 0..iterations {
            self.fire_step();
        }
    }
    fn render(&self, frame: &mut Frame, ascii: bool) {
        let width = self.width as usize;
        let base = self.height.saturating_sub(3) as usize;
        let glyphs = [' ', '.', ':', '*', 'x', '%', '#', '@'];
        for y in 0..base {
            for x in 0..width {
                let top = self.heat[(y * 2) * width + x];
                let bottom = self.heat[(y * 2 + 1) * width + x];
                if ascii {
                    let heat = top.max(bottom);
                    if heat >= 0.07 {
                        let level = (heat * 7.0).clamp(1.0, 7.0) as usize;
                        frame.put(x as i32, y as i32, glyphs[level], Self::fire_color(heat));
                    }
                } else {
                    let top = Self::fire_color(top);
                    let bottom = Self::fire_color(bottom);
                    if top != 0 || bottom != 0 {
                        frame.halves(x as i32, y as i32, top, bottom);
                    }
                }
            }
        }
        let center = self.width as i32 / 2;
        let half = (self.width as i32 * 27 / 100).max(1);
        let floor = self.height as i32 - 1;
        for row in 0..2 {
            let offset = if row == 0 { -2 } else { 2 };
            let left = center - half + offset;
            let right = center + half + offset;
            for x in left..=right {
                let glow = ((self.fire_tick as f32 * 0.09 + x as f32 * 1.7).sin() + 1.0) * 0.5;
                let (ch, color) = if x == left || x == right {
                    (
                        if ascii {
                            if x == left { '(' } else { ')' }
                        } else {
                            '●'
                        },
                        94,
                    )
                } else if (x + row * 3).rem_euclid(7) == 0 {
                    (
                        if ascii { '#' } else { '▓' },
                        if glow > 0.5 { 214 } else { 160 },
                    )
                } else {
                    (
                        if ascii { '=' } else { '━' },
                        if glow > 0.8 { 130 } else { 94 },
                    )
                };
                frame.put(x, floor - 2 + row, ch, color);
            }
        }
        for x in center - half - 3..=center + half + 3 {
            let glow = (x as f32 * 2.3 + self.fire_tick as f32 * 0.06).sin();
            frame.put(
                x,
                floor,
                if ascii { '.' } else { '·' },
                if glow > 0.4 {
                    202
                } else if glow > -0.4 {
                    88
                } else {
                    52
                },
            );
        }
        for p in &self.particles {
            let x = p.x as i32;
            let y = p.y as i32;
            if x < 0 || x >= self.width as i32 || y < 0 || y >= base as i32 {
                continue;
            }
            if self.heat[y as usize * 2 * width + x as usize] > 0.35 {
                continue;
            }
            let color = if p.y < self.height as f32 * 0.28 {
                88
            } else if p.size == 2 {
                220
            } else {
                202
            };
            if p.size == 2 {
                frame.put(x, y + 1, if ascii { '.' } else { '·' }, 88);
            }
            frame.put(x, y, if ascii { '.' } else { '·' }, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flame_simulation_is_independent_of_frame_rate() {
        let mut a = Fireplace::default();
        let mut b = Fireplace::default();
        a.reset(80, 24, 42);
        b.reset(80, 24, 42);
        for _ in 0..30 {
            a.update(1.0 / 30.0);
        }
        for _ in 0..60 {
            b.update(1.0 / 60.0);
        }
        assert_eq!(a.heat, b.heat);
    }
}
