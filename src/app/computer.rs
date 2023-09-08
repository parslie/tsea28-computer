mod component;
mod data;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::*;

use crate::types::Backend;

use self::{component::{Component, value8::Value8, list16::List16, value16::Value16, list7::List7, value7::Value7, list25::List25, value5::Value5}, data::{ProgramInstruction, MicroInstruction, Flags}};

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

    fn process_s(&mut self, prog_inst: &ProgramInstruction, micro_inst: &MicroInstruction) {
        self.multiplexer = if micro_inst.s == 0 {
            prog_inst.grx
        } else {
            prog_inst.m
        }
    }

    fn process_tb(&mut self, micro_inst: &MicroInstruction) {
        
    }

    fn process_fb(&mut self, micro_inst: &MicroInstruction) {
        
    }

    fn process_seq(&mut self, prog_inst: &ProgramInstruction, micro_inst: &MicroInstruction, flags: &Flags) {
        let mut set_on_condition = |condition: bool| {
            if condition {
                self.micro_counter.value = micro_inst.uadr;
            } else {
                self.micro_counter.value += 1;
            }
        };

        match micro_inst.seq {
            0b0000 => self.micro_counter.value += 1,
            0b0001 => self.micro_counter.value = self.k1.values[prog_inst.op as usize],
            0b0010 => self.micro_counter.value = self.k2.values[prog_inst.m as usize],
            0b0011 => self.micro_counter.value = 0,
            0b0100 => set_on_condition(flags.z == 0),
            0b0101 => self.micro_counter.value = micro_inst.uadr,
            0b0110 => {
                self.saved_micro_counter.value = self.micro_counter.value;
                self.micro_counter.value = micro_inst.uadr;
            },
            0b0111 => self.micro_counter.value = self.saved_micro_counter.value,
            0b1000 => set_on_condition(flags.z == 1),
            0b1001 => set_on_condition(flags.n == 1),
            0b1010 => set_on_condition(flags.c == 1),
            0b1011 => set_on_condition(flags.o == 1),
            0b1100 => set_on_condition(flags.l == 1),
            0b1101 => set_on_condition(flags.c == 0),
            0b1110 => set_on_condition(flags.o == 0),
            0b1111 => self.micro_counter.value = 0, // TODO: avbryt exekvering
            _ => (),
        }
    }

    fn process_alu(&mut self, micro_inst: &MicroInstruction) {
        
    }

    fn process_lc(&mut self, micro_inst: &MicroInstruction) {
        match micro_inst.lc {
            0b01 => self.loop_counter.value -= 1,
            0b10 => self.loop_counter.value = self.buss as u8,
            0b11 => self.loop_counter.value = 0x7f | micro_inst.uadr,
            _ => (),
        }

        if self.loop_counter.value == 0 {
            self.flags.value &= 0b11110;
        }
    }

    fn process_p(&mut self, micro_inst: &MicroInstruction) {
        if micro_inst.p == 1 {
            self.prog_counter.value += 1;
        }
    }
    
    fn perform_cycle(&mut self) {
        let prog_inst = ProgramInstruction::new(self.instruction_reg.value);
        let micro_inst = MicroInstruction::new(self.micro_memory.values[self.micro_counter.value as usize]);
        let flags = Flags::new(self.flags.value);

        self.process_s(&prog_inst, &micro_inst);
        self.process_tb(&micro_inst);
        self.process_fb(&micro_inst);
        self.process_seq(&prog_inst, &micro_inst, &flags); // TODO: påverkas flaggor före eller efter? (process_lc och process_alu påverkar)
        self.process_alu(&micro_inst);
        self.process_lc(&micro_inst);
        self.process_p(&micro_inst); // TODO: om man sätter PC med bussen, vad händer?
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
        } else if key.code == KeyCode::Char(' ') { 
            self.perform_cycle();
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
