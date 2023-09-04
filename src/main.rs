mod app;
mod types;

use std::{io::stdout, error::Error};

use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    execute,
};
use ratatui::Terminal;

use self::types::Backend;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let backend = Backend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    app::run(&mut terminal)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
