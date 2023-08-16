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