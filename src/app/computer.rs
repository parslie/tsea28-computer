mod component;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::*;

use crate::types::Backend;

use self::component::{Component, value8::Value8, list16::List16, value16::Value16, list7::List7, value7::Value7, list25::List25, value5::Value5};

use super::widget::CompositeWidget;

static COMPONENT_COUNT: u8 = 14;

pub struct Computer {
    address_reg: Value8,
    prog_counter: Value8,
    prog_memory: List16,

    accumulator_reg: Value16,
    help_reg: Value16,
    registers: List16,

    instruction_reg: Value16,
    k1: List7,
    k2: List7,
    micro_counter: Value7,
    saved_micro_counter: Value7,

    micro_memory: List25,

    flags: Value5,
    loop_counter: Value8,

    buss: u16,
    multiplexer: u8,

    selection_idx: u8,
}

impl Computer {
    pub fn new() -> Self {
        let mut computer = Self {
            address_reg: Value8::new("ASR:111"),
            prog_counter: Value8::new("PC:011"),
            prog_memory: List16::new("PM:010", 256),

            accumulator_reg: Value16::new("AR:100"),
            help_reg: Value16::new("HR:101"),
            flags: Value5::new("ZNCOL"),
            loop_counter: Value8::new("LC"),
            registers: List16::new("GRx:110", 4),

            instruction_reg: Value16::new("IR:001"),
            k1: List7::new("K1", 16),
            k2: List7::new("K2", 4),
            micro_counter: Value7::new("µPC"),
            saved_micro_counter: Value7::new("SµPC"),

            micro_memory: List25::new("µM:111", 128),

            buss: 0, 
            multiplexer: 0,

            selection_idx: 0,
        };
        computer.get_selected_mut().on_select();
        computer
    }

    fn get_selected_mut(&mut self) -> &mut dyn Component {
        match self.selection_idx {
            0 => &mut self.address_reg,
            1 => &mut self.prog_counter,
            2 => &mut self.prog_memory,
            3 => &mut self.accumulator_reg,
            4 => &mut self.help_reg,
            5 => &mut self.flags,
            6 => &mut self.loop_counter,
            7 => &mut self.registers,
            8 => &mut self.instruction_reg,
            9 => &mut self.k1,
            10 => &mut self.k2,
            11 => &mut self.micro_counter,
            12 => &mut self.saved_micro_counter,
            13 => &mut self.micro_memory,
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
                Constraint::Min(3),
            ])
            .split(layout[0]);

        let row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(column[0]);
        
        self.address_reg.render(frame, row[0]);
        self.prog_counter.render(frame, row[1]);
        self.prog_memory.render(frame, column[1]);

        let column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ])
            .split(layout[1]);

        let row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50), // TODO
            ])
            .split(column[2]);

        self.accumulator_reg.render(frame, column[0]);
        self.help_reg.render(frame, column[1]);
        self.flags.render(frame, row[0]);
        self.loop_counter.render(frame, row[1]);
        self.registers.render(frame, column[3]);

        let column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(layout[2]);

        let row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(column[1]);

        let row_2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(column[2]);

        self.instruction_reg.render(frame, column[0]);
        self.k1.render(frame, row[0]);
        self.k2.render(frame, row[1]);
        self.micro_counter.render(frame, row_2[0]);
        self.saved_micro_counter.render(frame, row_2[1]);

        self.micro_memory.render(frame, layout[3]);
    }
}
