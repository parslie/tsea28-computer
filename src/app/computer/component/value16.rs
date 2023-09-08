use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{prelude::*, widgets::{Borders, Block, Paragraph}};

use crate::{app::widget::CompositeWidget, types::Backend};

use super::Component;

pub struct Value16 {
    pub value: u16,
    cursor: u8,
    is_selected: bool,
    title: &'static str,
}

impl Value16 {
    pub fn new(title: &'static str) -> Self {
        Self {
            value: 0x0000,
            cursor: 15,
            is_selected: false,
            title: title,
        }
    }
}

impl CompositeWidget for Value16 {
    fn update(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Left && self.cursor < 15 {
            self.cursor += 1;
        } else if key.code == KeyCode::Right && self.cursor > 0 {
            self.cursor -= 1;
        } else if key.code == KeyCode::Char('0') {
            self.insert_zero();
        } else if key.code == KeyCode::Char('1') {
            self.insert_one();
        } else if key.code == KeyCode::Backspace {
            self.remove_one();
        }
    }

    fn render(&mut self, frame: &mut Frame<Backend>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title);

        let text = format!("{:0>16b}", self.value);

        let mut spans = Vec::new();
        let mut idx: u8 = 16;
        for c in text.chars() {
            let style = if idx - 1 == self.cursor && self.is_selected {
                Style::default().bg(Color::White).fg(Color::Black)
            } else {
                Style::default()
            };
            let span = Span::from(c.to_string()).set_style(style);
            spans.push(span);
            idx -= 1;
        }
        let para = Paragraph::new(Line::from(spans));

        frame.render_widget(para, block.inner(area));
        frame.render_widget(block, area);
    }
}

impl Component for Value16 {
    fn on_select(&mut self) {
        self.is_selected = true;
    }

    fn on_deselect(&mut self) {
        self.is_selected = false;
    }

    fn insert_one(&mut self) {
        let operand: u16 = 1 << self.cursor;
        self.value |= operand;
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn insert_zero(&mut self) {
        let operand: u16 = 0xffff ^ (1 << self.cursor);
        self.value &= operand;
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn remove_one(&mut self) {
        let operand: u16 = 0xffff ^ (1 << self.cursor);
        self.value &= operand;
        // TODO: shift all LSBs left?
        if self.cursor < 7 {
            self.cursor += 1;
        }
    }
}