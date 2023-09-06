mod component;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::*;

use crate::types::Backend;

use self::component::{Component, value8::Value8, list16::List16, value16::Value16};

use super::widget::CompositeWidget;

static COMPONENT_COUNT: u8 = 7;

pub struct Computer {
    instruction_reg: Value16,
    address_reg: Value8,
    prog_counter: Value8,
    prog_memory: List16,

    accumulator_reg: Value16,
    help_reg: Value16,
    registers: List16,

    loop_counter: Value8,

    buss: u16,
    multiplexer: u8,
    
    selection_idx: u8,
}

impl Computer {
    pub fn new() -> Self {
        let mut computer = Self {
            instruction_reg: Value16::new("IR:001"),
            address_reg: Value8::new("ASR:111"),
            prog_counter: Value8::new("PC:011"),
            prog_memory: List16::new("PM:010", 256),

            accumulator_reg: Value16::new("AR:100"),
            help_reg: Value16::new("HR:101"),
            registers: List16::new("GRx:110", 4),

            loop_counter: Value8::new("LC"),

            buss: 0, 
            multiplexer: 0,

            selection_idx: 0,
        };
        computer.get_selected_mut().on_select();
        computer
    }

    fn get_selected_mut(&mut self) -> &mut dyn Component {
        match self.selection_idx {
            0 => &mut self.instruction_reg,
            1 => &mut self.address_reg,
            2 => &mut self.prog_counter,
            3 => &mut self.prog_memory,
            4 => &mut self.accumulator_reg,
            5 => &mut self.help_reg,
            6 => &mut self.registers,
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
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        let column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ])
            .split(layout[0]);

        let row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(column[1]);
        
        self.instruction_reg.render(frame, column[0]);
        self.address_reg.render(frame, row[0]);
        self.prog_counter.render(frame, row[1]);
        self.prog_memory.render(frame, column[2]);

        let column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ])
            .split(layout[1]);

        self.accumulator_reg.render(frame, column[0]);
        self.help_reg.render(frame, column[1]);
        self.registers.render(frame, column[2]);
    }
}
