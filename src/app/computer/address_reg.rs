use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph}};

use crate::{app::widget::CompositeWidget, types::Backend};

use super::component::Component;

pub struct AddressRegister {
    value: u8,
    input: Option<String>,
}

impl AddressRegister {
    pub fn new() -> Self {
        Self {
            value: 0b_0000_0000,
            input: None,
        }
    }
}

impl CompositeWidget for AddressRegister {
    fn update(&mut self, key: KeyEvent) {
        // TODO: implement 
    }

    fn render(&self, frame: &mut Frame<Backend>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" ASR(111) ");

        // TODO: add input highlighting
        let text = match &self.input {
            Some(input) => format!("{:0<8}", input),
            None => format!("{:0>8b}", self.value),
        };

        let para = Paragraph::new(text);

        frame.render_widget(para, block.inner(area));
        frame.render_widget(block, area);
    }
}

impl Component for AddressRegister {
    fn on_select(self: &mut Self) {
        self.input = Some(format!("{:0>8b}", self.value));
    }

    fn on_deselect(self: &mut Self) {
        if let Some(input) = &self.input {
            self.value = u8::from_str_radix(input, 2).expect("should be valid binary");
        }
        self.input = None;
    }
}
