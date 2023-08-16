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