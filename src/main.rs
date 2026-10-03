mod app;
mod cli;
mod render;
mod rng;
mod scenes;
mod terminal;

use cli::Options;
use std::io::{self, IsTerminal};

fn main() {
    if let Err(error) = run() {
        eprintln!("termarius: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse(std::env::args_os().skip(1))?;
    if let Some(output) = &options.output {
        println!("{output}");
        return Ok(());
    }
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("Interactive playback needs a terminal on both stdin and stdout. Use --help or --list for plain output.".into());
    }
    let color = terminal::configure_color(options.color);
    let _terminal = terminal::Terminal::enter()?;
    app::run(options, color)?;
    Ok(())
}
