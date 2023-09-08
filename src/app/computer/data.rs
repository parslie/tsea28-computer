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

pub struct MicroInstruction {
    pub alu: u8,
    pub tb: u8,
    pub fb: u8,
    pub s: u8,
    pub p: u8,
    pub lc: u8,
    pub seq: u8,
    pub uadr: u8,
}

impl MicroInstruction {
    pub fn new(binary: u32) -> Self {
        Self {
            alu: ((binary & 0x01e00000) >> 21) as u8,
            tb: ((binary & 0x001c0000) >> 18) as u8,
            fb: ((binary & 0x00038000) >> 15) as u8,
            s: ((binary & 0x00004000) >> 14) as u8,
            p: ((binary & 0x00002000) >> 13) as u8,
            lc: ((binary & 0x00001800) >> 11) as u8,
            seq: ((binary & 0x00000780) >> 7) as u8,
            uadr: (binary & 0x0000007f) as u8,
        }
    }
}

pub struct Flags {
    pub z: u8,
    pub n: u8,
    pub c: u8,
    pub o: u8,
    pub l: u8,
}

impl Flags {
    pub fn new(binary: u8) -> Self {
        Self {
            z: (binary & 0x10) >> 4,
            n: (binary & 0x08) >> 3,
            c: (binary & 0x04) >> 2,
            o: (binary & 0x02) >> 1,
            l: binary & 0x01,
        }
    }
}
