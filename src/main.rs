//! # [Ratatui] List example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

mod app;
mod sshconfig;
mod sshconfigfile;
mod tui;

use clap::Parser;
use sshconfigfile::{parse, save_config};
use std::{error::Error, io::{BufReader, BufWriter}, path::PathBuf};

fn default_in_file() -> PathBuf {
    if let Some(x) = home::home_dir() {
        x.join(".ssh").join("config")
    } else {
        panic!("Unable to get home directory")
    }
}

fn default_out_file() -> PathBuf {
    if let Some(x) = home::home_dir() {
        x.join(".ssh").join("config.new")
    } else {
        panic!("Unable to get home directory")
    }
}

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Input config file
    #[arg(short, long, default_value_os_t = default_in_file())]
    in_file: PathBuf,

    /// Out file
    #[arg(short, long, default_value_os_t = default_out_file())]
    out_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Args = Args::parse();

    let file = std::fs::File::open(opts.in_file)?;

    let buf_reader = BufReader::new(file);

    let config = parse(buf_reader)?;

    tui::init_error_hooks()?;
    let terminal = tui::init_terminal()?;

    let mut app = app::App::with_config(config);

    app.run(terminal)?;

    tui::restore_terminal()?;

    let writer = std::fs::File::create(opts.out_file)?;
    let mut buf_writer = BufWriter::new(writer);

    save_config(&mut buf_writer, app.config())?;

    Ok(())
}
