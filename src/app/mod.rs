mod controls;
mod ui;

use crate::{
    cli::{ColorMode, Options},
    render::{Frame, Renderer},
    rng::Rng,
    scenes::{self, Kind, Scene},
};
use crossterm::{
    event::{self, Event, KeyEventKind},
    terminal,
};
use std::{
    io,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct App {
    seed: u64,
    random: Rng,
    kind: Kind,
    scene: Box<dyn Scene>,
    size: (u16, u16),
    picker: bool,
    selected: usize,
    paused: bool,
    help: bool,
    speed: f32,
    ascii: bool,
    screensaver: bool,
}

impl App {
    fn new(options: &Options, size: (u16, u16)) -> Self {
        let seed = options.seed.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        });
        let mut random = Rng::new(seed);
        let mut kind = options.scene.unwrap_or(Kind::Fireplace);
        if options.random || (options.screensaver && options.scene.is_none()) {
            kind = Kind::ALL[random.range(Kind::ALL.len())];
        }
        let height = size
            .1
            .saturating_sub(if options.screensaver { 0 } else { 1 });
        Self {
            seed,
            random,
            kind,
            size,
            scene: scenes::create(kind, size.0, height, seed),
            picker: options.scene.is_none() && !options.random && !options.screensaver,
            selected: kind.index(),
            paused: false,
            help: false,
            speed: 1.0,
            ascii: options.ascii,
            screensaver: options.screensaver,
        }
    }

    fn scene_height(&self) -> u16 {
        self.size
            .1
            .saturating_sub(if self.screensaver { 0 } else { 1 })
    }

    fn reset_scene(&mut self) {
        self.scene
            .reset(self.size.0, self.scene_height(), self.seed);
    }

    fn change_scene(&mut self, kind: Kind) {
        self.kind = kind;
        self.scene = scenes::create(kind, self.size.0, self.scene_height(), self.seed);
    }

    fn resize(&mut self, size: (u16, u16)) {
        if size != self.size {
            self.size = size;
            self.reset_scene();
        }
    }

    fn frame(&self, color: ColorMode) -> Frame {
        let mut frame = Frame::new(self.size.0, self.size.1);
        if self.picker {
            ui::draw_picker(&mut frame, self.selected, self.ascii);
        } else {
            self.scene
                .render(&mut frame, self.ascii || color == ColorMode::None);
            if !self.screensaver {
                ui::draw_status(&mut frame, self.kind, self.paused, self.speed);
            }
            if self.help {
                ui::draw_help(&mut frame);
            }
        }
        frame
    }
}

pub(crate) fn run(options: Options, color: ColorMode) -> io::Result<()> {
    let mut app = App::new(&options, terminal::size()?);
    let mut renderer = Renderer::new(color);
    let mut out = io::stdout();
    let interval = Duration::from_secs_f64(1.0 / options.fps as f64);
    let mut last = Instant::now();
    loop {
        let started = Instant::now();
        let dt = started.duration_since(last).as_secs_f32().min(0.25);
        last = started;
        app.resize(terminal::size()?);
        if !app.picker && !app.paused && !app.help {
            app.scene.update(dt * app.speed);
        }
        renderer.draw(&mut out, app.frame(color))?;

        loop {
            let remaining = interval.saturating_sub(started.elapsed());
            if !event::poll(remaining)? {
                break;
            }
            match event::read()? {
                Event::Key(key) if key.kind != KeyEventKind::Release => {
                    if app.handle_key(key) {
                        return Ok(());
                    }
                    break;
                }
                Event::Resize(_, _) => break,
                _ => {
                    if started.elapsed() >= interval {
                        break;
                    }
                }
            }
        }
    }
}
