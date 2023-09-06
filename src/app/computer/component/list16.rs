use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{prelude::*, widgets::*};

use crate::{app::widget::CompositeWidget, types::Backend};

use super::Component;

pub struct List16 {
    values: Vec<u16>,
    size: usize,
    offset: usize,
    cursor: (u8, usize),
    is_selected: bool,
    title: &'static str,
}

impl List16 {
    pub fn new(title: &'static str, size: usize) -> Self {
        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(0x0000);
        }

        Self {
            values: values,
            size: size,
            offset: 0,
            cursor: (15, 0),
            is_selected: false,
            title: title,
        }
    }
}

impl CompositeWidget for List16 {
    fn update(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Left && self.cursor.0 < 15 {
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
            self.cursor.0 = 7;
        }
    }

    fn render(&mut self, frame: &mut Frame<Backend>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title);
        let inner_area = block.inner(area);

        if self.cursor.1 < self.offset {
            self.offset = self.cursor.1;
        } else if self.cursor.1 >= self.offset + inner_area.height as usize {
            self.offset = self.cursor.1 + 1 - inner_area.height as usize;
        }

        let min_idx = self.offset;
        let mut max_idx = min_idx + inner_area.height as usize;
        max_idx = max_idx.clamp(min_idx, self.size);
        let line_count = max_idx - min_idx;

        let mut address_lines = Vec::with_capacity(line_count);
        let mut value_lines = Vec::with_capacity(line_count);

        // TODO: implement highlighting
        for idx in min_idx..max_idx {
            address_lines.push(Line::from(format!("0x{:0>2x}", idx)));
            
            if idx == self.cursor.1 && self.is_selected {
                let mut line_spans = Vec::new();
                let line_string = format!("{:0>16b}", self.values[idx]);
                let mut line_idx: u8 = 16;
                for c in line_string.chars() {
                    let style = if line_idx - 1 == self.cursor.0 {
                        Style::default().bg(Color::White).fg(Color::Black)
                    } else {
                        Style::default()
                    };
                    let line_span = Span::from(c.to_string()).set_style(style);
                    line_spans.push(line_span);
                    line_idx -= 1;
                }
                value_lines.push(Line::from(line_spans));
            } else {
                value_lines.push(Line::from(format!("{:0>16b}", self.values[idx])));
            }
        }
        
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(4),
                Constraint::Length(16),
            ])
            .split(inner_area);

        frame.render_widget(Paragraph::new(address_lines), inner_layout[0]);
        frame.render_widget(Paragraph::new(value_lines), inner_layout[1]);
        frame.render_widget(block, area);
    }
}

impl Component for List16 {
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
        let operand = 0xffff ^ (1 << self.cursor.0);
        self.values[self.cursor.1] &= operand;
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
        }
    }

    fn remove_one(&mut self) {
        let operand = 0xffff ^ (1 << self.cursor.0);
        self.values[self.cursor.1] &= operand;
        // TODO: shift all LSBs left?
        if self.cursor.0 < 15 {
            self.cursor.0 += 1;
        }
    }
}
