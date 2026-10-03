use crate::rng::Rng;

pub(super) struct Particle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub phase: f32,
    pub size: usize,
}
impl Particle {
    pub fn random(rng: &mut Rng, width: u16, height: u16) -> Self {
        Self {
            x: rng.unit() * width as f32,
            y: rng.unit() * height as f32,
            speed: 1.0 + rng.unit() * 5.0,
            phase: rng.unit() * std::f32::consts::TAU,
            size: rng.range(3),
        }
    }
}

#[derive(Default)]
pub(super) struct ParticleField {
    pub width: u16,
    pub height: u16,
    pub rng: Rng,
    pub time: f32,
    pub particles: Vec<Particle>,
}
impl ParticleField {
    pub fn new(
        width: u16,
        height: u16,
        seed: u64,
        count: usize,
        mut configure: impl FnMut(usize, &mut Particle, &mut Rng),
    ) -> Self {
        let mut rng = Rng::new(seed);
        let mut particles = Vec::with_capacity(count);
        for index in 0..count {
            let mut particle = Particle::random(&mut rng, width, height);
            configure(index, &mut particle, &mut rng);
            particles.push(particle);
        }
        Self {
            width,
            height,
            rng,
            time: 0.0,
            particles,
        }
    }
}
