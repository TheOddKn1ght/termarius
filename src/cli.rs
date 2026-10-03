use std::ffi::OsString;

use crate::scenes::Kind;

pub const HELP: &str = "Termarius — eight little worlds for your terminal

Usage: termarius [SCENE|random] [OPTIONS]

Scenes: fireplace, matrix, bubbles, underwater, bonsai, rain, stars, snow
Without a scene, open the picker.

Options:
  --list                 List scenes
  --help, -h             Show help
  --version, -V          Show version
  --ascii                Use ASCII characters only
  --color MODE           auto, 16, 256, or none (default: auto)
  --fps N                Frame rate, 1..60 (default: 30)
  --seed N               Reproducible unsigned 64-bit seed
  --screensaver          Exit on any key; defaults to a random scene

Controls: arrows switch scenes; m picker; Space pause; +/- speed;
          r regenerate; ? help; q/Esc/Ctrl+C quit.";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorMode {
    Auto,
    Ansi16,
    Ansi256,
    None,
}

#[derive(Debug)]
pub struct Options {
    pub scene: Option<Kind>,
    pub random: bool,
    pub ascii: bool,
    pub color: ColorMode,
    pub fps: u32,
    pub seed: Option<u64>,
    pub screensaver: bool,
    pub output: Option<String>,
}

impl Options {
    pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self, String> {
        let mut out = Self {
            scene: None,
            random: false,
            ascii: false,
            color: ColorMode::Auto,
            fps: 30,
            seed: None,
            screensaver: false,
            output: None,
        };
        let mut args = args.into_iter();
        let mut positional = false;
        while let Some(arg) = args.next() {
            let arg = arg
                .into_string()
                .map_err(|_| "Arguments must be valid UTF-8")?;
            match arg.as_str() {
                "--help" | "-h" => out.output = Some(HELP.into()),
                "--version" | "-V" => {
                    out.output = Some(format!("termarius {}", env!("CARGO_PKG_VERSION")))
                }
                "--list" => {
                    out.output = Some(
                        Kind::ALL
                            .iter()
                            .map(|k| k.name())
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                }
                "--ascii" => out.ascii = true,
                "--screensaver" => out.screensaver = true,
                "--fps" | "--seed" | "--color" => {
                    let value = args
                        .next()
                        .ok_or_else(|| format!("{arg} needs a value"))?
                        .into_string()
                        .map_err(|_| "Arguments must be valid UTF-8")?;
                    match arg.as_str() {
                        "--fps" => {
                            out.fps = value
                                .parse()
                                .map_err(|_| "--fps must be an integer from 1 to 60")?;
                            if !(1..=60).contains(&out.fps) {
                                return Err("--fps must be between 1 and 60".into());
                            }
                        }
                        "--seed" => {
                            out.seed = Some(
                                value
                                    .parse()
                                    .map_err(|_| "--seed must be an unsigned 64-bit integer")?,
                            )
                        }
                        _ => {
                            out.color = match value.as_str() {
                                "auto" => ColorMode::Auto,
                                "16" => ColorMode::Ansi16,
                                "256" => ColorMode::Ansi256,
                                "none" => ColorMode::None,
                                _ => return Err("--color must be auto, 16, 256, or none".into()),
                            }
                        }
                    }
                }
                _ if arg.starts_with('-') => return Err(format!("Unknown option: {arg}")),
                _ => {
                    if positional {
                        return Err("Choose only one scene".into());
                    }
                    positional = true;
                    if arg == "random" {
                        out.random = true;
                    } else {
                        out.scene = Some(
                            Kind::parse(&arg)
                                .ok_or_else(|| format!("Unknown scene: {arg}. Use --list."))?,
                        );
                    }
                }
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Options, String> {
        Options::parse(args.iter().map(OsString::from))
    }
    #[test]
    fn valid_options() {
        let o = parse(&[
            "matrix",
            "--fps",
            "60",
            "--seed",
            "0",
            "--ascii",
            "--color",
            "16",
            "--screensaver",
        ])
        .unwrap();
        assert_eq!(o.scene, Some(Kind::Matrix));
        assert_eq!(o.seed, Some(0));
        assert_eq!(o.color, ColorMode::Ansi16);
        assert!(o.ascii && o.screensaver);
        assert!(parse(&["random"]).unwrap().random);
    }
    #[test]
    fn rejects_invalid_arguments() {
        for args in [
            &["--fps", "0"][..],
            &["--fps", "61"],
            &["--fps", "bad"],
            &["--seed", "-1"],
            &["--color", "rgb"],
            &["--seed"],
            &["bogus"],
            &["rain", "snow"],
            &["--wat"],
        ] {
            assert!(parse(args).is_err(), "{args:?}");
        }
    }
}
