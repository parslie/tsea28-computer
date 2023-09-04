use crossterm::event::KeyEvent;
use ratatui::prelude::*;

use super::widget::CompositeWidget;

pub struct Computer {
    
}

impl Computer {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl CompositeWidget for Computer {
    fn update(&self, key: KeyEvent) {
        // TODO: update self & components here
    }

    fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        // TODO: render components in a layout here
    }
}
