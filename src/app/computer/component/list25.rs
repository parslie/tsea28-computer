use std::ops::Range;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{prelude::*, widgets::*};

use crate::{app::widget::CompositeWidget, types::Backend};

use super::Component;

pub struct List25 {
    pub values: Vec<u32>,
    size: usize,
    offset: usize,
    cursor: (u8, usize),
    is_selected: bool,
    title: &'static str,
}

impl List25 {
    pub fn new(title: &'static str, size: usize) -> Self {
        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(0x00000000);
        }

        Self {
            values: values,
            size: size,
            offset: 0,
            cursor: (24, 0),
            is_selected: false,
            title: title,
        }
    }

    fn adjust_offset(&mut self, height: usize) {
        if self.cursor.1 < self.offset {
            self.offset = self.cursor.1;
        } else if self.cursor.1 >= self.offset + height {
            self.offset = self.cursor.1 + 1 - height;
        }
    }

    fn build_line(&self, width: usize, idx: usize) -> Line {
        let address = idx;
        let value = self.values[idx];

        let mut spans = Vec::with_capacity(3);
        spans.push(Span::from(format!("0x{:0>2x}", address)));

        if width > 4 + 25 {
            spans.push(Span::from(" ".repeat(width - 4 - 25)));
        }

        let value_text = format!("{:0>25b}", value);
        if self.is_selected && idx == self.cursor.1 {
            for (ch_idx, ch) in value_text.chars().enumerate() {
                let style = if 24 - ch_idx as u8 == self.cursor.0 {
                    Style::default().bg(Color::White).fg(Color::Black)
                } else {
                    Style::default()
                };
                let span = Span::from(ch.to_string()).set_style(style);
                spans.push(span);
            }
        } else {
            spans.push(Span::from(value_text));
        }
        
        Line::from(spans)
    }

    fn value_range(&self, height: usize) -> Range<usize> {
        let min_idx = self.offset;
        let max_idx = usize::min(min_idx + height, self.size);
        min_idx..max_idx
    }
}

impl CompositeWidget for List25 {
    fn update(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Left && self.cursor.0 < 24 {
            self.cursor.0 += 1;
        } else if key.code == KeyCode::Right && self.cursor.0 > 0 {
            self.cursor.0 -= 1;
        } else if key.code == KeyCode::Up && self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if key.code == KeyCode::Down && self.cursor.1 < self.size - 1 {
            self.cursor.1 += 1;
        } else if key.code == KeyCode::Char('0') {
            self.insert_zero();
        } else if key.code == KeyCode::Char('1') {
            self.insert_one();
        } else if key.code == KeyCode::Backspace {
            self.remove_one();
        } else if key.code == KeyCode::Enter && self.cursor.1 < self.size - 1 {
            self.cursor.1 += 1;
            self.cursor.0 = 24;
        }
    }

    fn render(&mut self, frame: &mut Frame<Backend>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title);
        let inner_area = block.inner(area);

        self.adjust_offset(inner_area.height as usize);

        let mut lines = Vec::new();
        for idx in self.value_range(inner_area.height as usize) {
            lines.push(self.build_line(inner_area.width as usize, idx));
        }

        frame.render_widget(Paragraph::new(lines), inner_area);
        frame.render_widget(block, area);
    }
}

impl Component for List25 {
    fn on_select(&mut self) {
        self.is_selected = true;
    }

    fn on_deselect(&mut self) {
        self.is_selected = false;
    }

    fn insert_one(&mut self) {
        let operand = 1 << self.cursor.0;
        self.values[self.cursor.1] |= operand;
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
        }
    }

    fn insert_zero(&mut self) {
        let operand = 0xffffffff ^ (1 << self.cursor.0);
        self.values[self.cursor.1] &= operand;
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
        }
    }

    fn remove_one(&mut self) {
        let operand = 0xffffffff ^ (1 << self.cursor.0);
        self.values[self.cursor.1] &= operand;
        // TODO: shift all LSBs left?
        if self.cursor.0 < 24 {
            self.cursor.0 += 1;
        }
    }
}
