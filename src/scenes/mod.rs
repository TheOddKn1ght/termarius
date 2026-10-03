use crate::render::Frame;

mod bonsai;
mod bubbles;
mod common;
mod fireplace;
mod matrix;
mod rain;
mod snow;
mod stars;
mod underwater;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Fireplace,
    Matrix,
    Bubbles,
    Underwater,
    Bonsai,
    Rain,
    Stars,
    Snow,
}
impl Kind {
    pub const ALL: [Self; 8] = [
        Self::Fireplace,
        Self::Matrix,
        Self::Bubbles,
        Self::Underwater,
        Self::Bonsai,
        Self::Rain,
        Self::Stars,
        Self::Snow,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Fireplace => "fireplace",
            Self::Matrix => "matrix",
            Self::Bubbles => "bubbles",
            Self::Underwater => "underwater",
            Self::Bonsai => "bonsai",
            Self::Rain => "rain",
            Self::Stars => "stars",
            Self::Snow => "snow",
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Fireplace => "Warm flames, glowing logs, drifting embers",
            Self::Matrix => "Green code cascading through the dark",
            Self::Bubbles => "Little bubbles floating up and away",
            Self::Underwater => "Fish, seaweed, and a quiet ocean floor",
            Self::Bonsai => "A small tree taking its time to grow",
            Self::Rain => "Rain streaks and ripples on the ground",
            Self::Stars => "A twinkling sky with shooting stars",
            Self::Snow => "Snowflakes drifting over a winter landscape",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.name() == s)
    }
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|k| *k == self).unwrap()
    }
}

pub trait Scene {
    fn reset(&mut self, width: u16, height: u16, seed: u64);
    fn update(&mut self, dt: f32);
    fn render(&self, frame: &mut Frame, ascii: bool);
}

pub fn create(kind: Kind, width: u16, height: u16, seed: u64) -> Box<dyn Scene> {
    let mut scene: Box<dyn Scene> = match kind {
        Kind::Fireplace => Box::new(fireplace::Fireplace::default()),
        Kind::Matrix => Box::new(matrix::Matrix::default()),
        Kind::Bubbles => Box::new(bubbles::Bubbles::default()),
        Kind::Underwater => Box::new(underwater::Underwater::default()),
        Kind::Bonsai => Box::new(bonsai::Bonsai::default()),
        Kind::Rain => Box::new(rain::Rain::default()),
        Kind::Stars => Box::new(stars::Stars::default()),
        Kind::Snow => Box::new(snow::Snow::default()),
    };
    scene.reset(width, height, seed);
    scene
}
