pub struct ProgramInstruction {
    pub op: u8,
    pub grx: u8,
    pub m: u8,
    pub adr: u8,
}

impl ProgramInstruction {
    pub fn new(binary: u16) -> Self {
        Self {
            op: ((binary & 0xf000) >> 12) as u8,
            grx: ((binary & 0x0c00) >> 8) as u8,
            m: ((binary & 0x0300) >> 8) as u8,
            adr: (binary & 0x00ff) as u8,
        }
    }
}