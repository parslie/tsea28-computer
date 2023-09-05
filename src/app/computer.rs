mod component;
mod address_reg;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::*;

use crate::types::Backend;

use self::{address_reg::AddressRegister, component::Component};

use super::widget::CompositeWidget;

static COMPONENT_COUNT: u8 = 2;

pub struct Computer {
    address_reg: AddressRegister,
    prog_counter: AddressRegister,
    selection_idx: u8,
}

impl Computer {
    pub fn new() -> Self {
        let mut computer = Self {
            address_reg: AddressRegister::new(),
            prog_counter: AddressRegister::new(), // TODO: add actual PC, just here to test selecting different components
            selection_idx: 0,
        };
        computer.address_reg.on_select();
        computer
    }

    fn get_selected_mut(&mut self) -> &mut dyn Component {
        match self.selection_idx {
            0 => &mut self.address_reg,
            1 => &mut self.prog_counter,
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

    fn render(&self, frame: &mut Frame<Backend>, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);
        
        self.address_reg.render(frame, layout[0]);
        self.prog_counter.render(frame, layout[1]);
    }
}
