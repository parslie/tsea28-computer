mod program_instruction;
mod micro_instruction;
mod flags;
mod alu;

use program_instruction::ProgramInstruction;
use micro_instruction::MicroInstruction;
use flags::Flags;

pub struct Computer {
    buss: u16,
    pub address_reg: u8,
    pub program_counter: u8,
    pub program_memory: [u16; 256],
    pub instruction_reg: u16,
    pub k1: [u8; 16],
    pub k2: [u8; 4],
    pub micro_counter: u8,
    pub saved_micro_counter: u8,
    pub micro_memory: [u32; 128],
    pub accumulator_reg: u16,
    pub help_reg: u16,
    pub loop_counter: u8,
    pub flags: u8,
    pub registers: [u16; 4],
    multiplexer: u8,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            buss: 0,
            address_reg: 0,
            program_counter: 0,
            program_memory: [0; 256],
            instruction_reg: 0,
            k1: [0; 16],
            k2: [0; 4],
            micro_counter: 0,
            saved_micro_counter: 0,
            micro_memory: [0; 128],
            accumulator_reg: 0,
            help_reg: 0,
            loop_counter: 0,
            flags: 0,
            registers: [0; 4],
            multiplexer: 0,
        }
    }

    fn process_seq(&mut self, program: &ProgramInstruction, micro: &MicroInstruction, flags: &Flags) {
        let mut set_on_condition = |condition: bool| {
            if condition {
                self.micro_counter = micro.uadr;
            } else {
                self.micro_counter += 1;
            }
        };

        match micro.seq {
            0b0000 => self.micro_counter += 1,
            0b0001 => self.micro_counter = self.k1[program.op as usize],
            0b0010 => self.micro_counter = self.k2[program.m as usize],
            0b0011 => self.micro_counter = 0,
            0b0100 => set_on_condition(flags.z == 0),
            0b0101 => self.micro_counter = micro.uadr,
            0b0110 => {
                self.saved_micro_counter = self.micro_counter;
                self.micro_counter = micro.uadr;
            },
            0b0111 => self.micro_counter = self.saved_micro_counter,
            0b1000 => set_on_condition(flags.z == 1),
            0b1001 => set_on_condition(flags.n == 1),
            0b1010 => set_on_condition(flags.c == 1),
            0b1011 => set_on_condition(flags.o == 1),
            0b1100 => set_on_condition(flags.l == 1),
            0b1101 => set_on_condition(flags.c == 0),
            0b1110 => set_on_condition(flags.o == 0),
            0b1111 => self.micro_counter = 0, // TODO: avbryt exekvering
            _ => (),
        }
    }

    fn process_s(&mut self, program: &ProgramInstruction, micro: &MicroInstruction) {
        self.multiplexer = if micro.s == 0 {
            program.grx
        } else {
            program.m
        }
    }

    fn process_p(&mut self, micro: &MicroInstruction) {
        if micro.p == 1 {
            self.program_counter += 1;
        }
    }

    fn process_lc(&mut self, micro: &MicroInstruction) {
        match micro.lc {
            0b01 => self.loop_counter -= 1,
            0b10 => self.loop_counter = self.buss as u8,
            0b11 => self.loop_counter = 0x7f | micro.uadr,
            _ => (),
        }

        if self.loop_counter == 0 {
            self.flags &= 0b11111110;
        }
    }

    fn process_tb(&mut self, micro: &MicroInstruction) {
        self.buss = match micro.tb {
            0b001 => self.instruction_reg & 0x00ff,
            0b010 => self.program_memory[self.address_reg as usize],
            0b011 => self.program_counter as u16,
            0b100 => self.accumulator_reg,
            0b101 => self.help_reg,
            0b110 => self.registers[self.multiplexer as usize],
            0b111 => todo!("implement tb = 0b111"),
            _ => self.buss,
        }
    }

    fn process_fb(&mut self, micro: &MicroInstruction) {
        if micro.alu == 0 {
            // TODO: borde det här göras om alu inte är 0b0000?
            match micro.fb {
                0b001 => self.instruction_reg = self.buss,
                0b010 => self.program_memory[self.address_reg as usize] = self.buss,
                0b011 => self.program_counter = self.buss as u8,
                0b101 => self.help_reg = self.buss,
                0b110 => self.registers[self.multiplexer as usize] = self.buss,
                0b111 => self.address_reg = self.buss as u8,
                _ => (),
            }
        }
    }

    fn process_alu(&mut self, micro: &MicroInstruction) {
        match micro.alu {
            0b0001 => alu::equ_buss(self, micro),
            0b0010 => alu::equ_buss_ones_comp(self, micro),
            0b0011 => alu::equ_zero(self, micro),
            0b0100 => alu::plus_buss(self, micro, true),
            0b0101 => alu::minus_buss(self, micro),
            0b0110 => alu::and_buss(self, micro),
            0b0111 => alu::or_buss(self, micro),
            0b1000 => alu::plus_buss(self, micro, false),
            0b1001 => alu::logic_shift_left_16(self, micro),
            0b1010 => alu::logic_shift_left_32(self, micro),
            0b1011 => alu::arithmetic_shift_right_16(self, micro),
            0b1100 => alu::arithmetic_shift_right_32(self, micro),
            0b1101 => alu::logic_shift_right_16(self, micro),
            0b1110 => alu::rotate_left_16(self, micro),
            0b1111 => alu::rotate_left_32(self, micro),
            _ => (),
        }
    }

    pub fn perform_cycle(&mut self) {
        let micro_instruction = self.micro_memory[self.micro_counter as usize];

        let program = ProgramInstruction::new(self.instruction_reg);
        let micro = MicroInstruction::new(micro_instruction);
        let flags = Flags::new(self.flags);

        self.process_s(&program, &micro);
        self.process_tb(&micro);
        self.process_fb(&micro);
        self.process_seq(&program, &micro, &flags); // TODO: påverkas flaggor före eller efter? (process_lc och process_alu påverkar)
        self.process_alu(&micro);
        self.process_lc(&micro);
        self.process_p(&micro); // TODO: om man sätter PC med bussen, vad händer?
    }
}