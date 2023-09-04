use std::io::Stdout;

use ratatui::backend::CrosstermBackend;

pub type Backend = CrosstermBackend<Stdout>;
