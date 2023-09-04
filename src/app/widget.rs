use crossterm::event::KeyEvent;
use ratatui::prelude::*;

use crate::types::Backend;

pub trait CompositeWidget {
    fn update(self: &mut Self, key: KeyEvent);
    fn render(self: &Self, frame: &mut Frame<Backend>, area: Rect);
}
