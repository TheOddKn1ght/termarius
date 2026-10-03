use super::*;
#[test]
fn every_scene_is_reproducible() {
    for kind in Kind::ALL {
        let mut a = create(kind, 80, 24, 0);
        let mut b = create(kind, 80, 24, 0);
        for _ in 0..90 {
            a.update(1.0 / 30.0);
            b.update(1.0 / 30.0);
        }
        let mut fa = Frame::new(80, 24);
        let mut fb = fa.clone();
        a.render(&mut fa, false);
        b.render(&mut fb, false);
        assert_eq!(fa, fb, "{}", kind.name());
        assert!(
            fa.cells.iter().any(|c| c.ch != ' '),
            "{} is blank",
            kind.name()
        );
    }
}
#[test]
fn small_and_resized_frames_are_safe_and_ascii_is_ascii() {
    for kind in Kind::ALL {
        let mut world = create(kind, 80, 24, 42);
        for (w, h) in [(0, 0), (1, 1), (2, 3), (120, 40), (10, 5), (80, 24)] {
            world.reset(w, h, 42);
            for _ in 0..100 {
                world.update(0.1);
            }
            let mut frame = Frame::new(w, h);
            world.render(&mut frame, true);
            assert_eq!(frame.cells.len(), w as usize * h as usize);
            assert!(frame.cells.iter().all(|c| c.ch.is_ascii()));
        }
    }
}
