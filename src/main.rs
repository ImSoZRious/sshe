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
mod tui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tui::init_error_hooks()?;
    let terminal = tui::init_terminal()?;

    let mut app = app::App::default();
    app.run(terminal)?;

    tui::restore_terminal()?;
    Ok(())
}
