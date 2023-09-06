mod component;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::*;

use crate::types::Backend;

use self::component::{Component, value8::Value8, list16::List16};

use super::widget::CompositeWidget;

static COMPONENT_COUNT: u8 = 3;

pub struct Computer {
    address_reg: Value8,
    prog_counter: Value8,
    prog_memory: List16,
    selection_idx: u8,
}

impl Computer {
    pub fn new() -> Self {
        let mut computer = Self {
            address_reg: Value8::new("ASR:111"),
            prog_counter: Value8::new("PC:011"),
            prog_memory: List16::new("PM:010", 256),
            selection_idx: 0,
        };
        computer.address_reg.on_select();
        computer
    }

    fn get_selected_mut(&mut self) -> &mut dyn Component {
        match self.selection_idx {
            0 => &mut self.address_reg,
            1 => &mut self.prog_counter,
            2 => &mut self.prog_memory,
            _ => &mut self.address_reg, // Needed to satisfy return value
        }
    }
}

impl CompositeWidget for Computer {
    fn update(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Tab {
            self.get_selected_mut().on_deselect();
            self.selection_idx = if self.selection_idx == COMPONENT_COUNT - 1 {
                0
            } else {
                self.selection_idx + 1
            };
            self.get_selected_mut().on_select();
        } else {
            self.get_selected_mut().update(key)
        }
    }

    fn render(&mut self, frame: &mut Frame<Backend>, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(area);
        
        self.address_reg.render(frame, layout[0]);
        self.prog_counter.render(frame, layout[1]);
        self.prog_memory.render(frame, layout[2]);
    }
}
