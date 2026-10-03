use crate::{render::Frame, scenes::Kind};

pub(super) fn draw_picker(frame: &mut Frame, selected: usize, ascii: bool) {
    if frame.height < 8 {
        frame.text(0, 0, Kind::ALL[selected].name(), 81);
        frame.text(0, 1, "j/k choose  enter open  q quit", 244);
        return;
    }
    let start = ((frame.height as i32 - Kind::ALL.len() as i32 - 7) / 2).max(0);
    frame.centered(start, "t e r m a r i u s", 81);
    frame.centered(start + 2, "a little world, inside your terminal", 244);
    let x = (frame.width as i32 / 2 - 9).max(0);
    let rows = (frame.height.saturating_sub(7) as usize).clamp(1, Kind::ALL.len());
    let first = selected
        .saturating_sub(rows / 2)
        .min(Kind::ALL.len() - rows);
    for (row, index) in (first..first + rows).enumerate() {
        let k = Kind::ALL[index];
        frame.text(
            x,
            start + 4 + row as i32,
            &format!(
                "{} {}",
                if index == selected {
                    if ascii { ">" } else { "›" }
                } else {
                    " "
                },
                k.name()
            ),
            if index == selected { 81 } else { 250 },
        );
    }
    let footer = start + 5 + rows as i32;
    frame.centered(footer, Kind::ALL[selected].description(), 244);
    frame.centered(
        footer + 2,
        "j/k or arrows choose   enter opens   q quits",
        244,
    );
}

pub(super) fn draw_help(frame: &mut Frame) {
    let lines = [
        "CONTROLS",
        "",
        "arrows   previous / next scene",
        "m        scene picker",
        "Space    pause / resume",
        "+ / -    animation speed",
        "r        regenerate",
        "? / Esc  close help",
        "q        quit",
    ];
    let width = 36.min(frame.width as i32);
    let x = (frame.width as i32 - width) / 2;
    let y = ((frame.height as i32 - lines.len() as i32 - 2) / 2).max(0);
    for row in 0..lines.len() as i32 + 2 {
        for col in 0..width {
            frame.put(x + col, y + row, ' ', 7);
        }
    }
    for (i, line) in lines.iter().enumerate() {
        frame.text(x + 2, y + 1 + i as i32, line, if i == 0 { 81 } else { 250 });
    }
}

pub(super) fn draw_status(frame: &mut Frame, kind: Kind, paused: bool, speed: f32) {
    let status = format!(
        " {}  {}{:.2}x   m scenes  ? help  q quit",
        kind.name(),
        if paused { "paused " } else { "" },
        speed
    );
    let row = frame.height as i32 - 1;
    for x in 0..frame.width {
        frame.put(x as i32, row, ' ', 7);
    }
    frame.text(0, row, &status, 244);
}
