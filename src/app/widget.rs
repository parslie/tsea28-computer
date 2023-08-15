use crossterm::event::KeyEvent;
use ratatui::prelude::*;

trait CompositeWidget {
    fn update(&self, key: KeyEvent);
    fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}
