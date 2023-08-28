use super::Computer;

pub fn equ_buss(computer: &mut Computer) {
    computer.accumulator_reg = computer.buss;
}

pub fn equ_buss_ones_comp(_computer: &mut Computer) {
    todo!("implement ones' complement"); // TODO: implement
}

pub fn equ_zero(computer: &mut Computer) {
    computer.accumulator_reg = 0;
    computer.flags |= 0b10000;
    computer.flags &= 0b10111;
}

pub fn plus_buss(computer: &mut Computer, update_flags: bool) {
    let ar = computer.accumulator_reg as i16 as i32 as u32;
    let buss = computer.buss as i16 as i32 as u32;
    let result = ar + buss;

    if update_flags {
        if result & 0x00010000 != 0 {
            computer.flags |= 0b00100;
        } else {
            computer.flags &= 0b11011;
        }
    }

    let ar = ar as u16;
    let buss: u16 = buss as u16;
    let result = result as u16;

    if update_flags {
        if ar & 0x8000 == buss & 0x8000 && ar & 0x8000 != result & 0x8000 {
            computer.flags |= 0b00010;
        } else {
            computer.flags &= 0b11101;
        }
    
        if result == 0 {
            computer.flags |= 0b10000;
        } else {
            computer.flags &= 0b01111;
        }
    
        if result & 0x8000 != 0 {
            computer.flags |= 0b01000;
        } else {
            computer.flags &= 0b10111;
        }
    }

    computer.accumulator_reg = result;
}

pub fn minus_buss(computer: &mut Computer) {
    let ar = computer.accumulator_reg as i16 as i32 as u32;
    let buss = (computer.buss as i16 as i32 * -1) as u32;
    let result = ar - buss;

    if result & 0x00010000 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    let ar = ar as u16;
    let buss: u16 = buss as u16;
    let result = result as u16;

    if ar & 0x8000 == buss & 0x8000 && ar & 0x8000 != result & 0x8000 {
        computer.flags |= 0b00010;
    } else {
        computer.flags &= 0b11101;
    }

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result;
}

pub fn and_buss(computer: &mut Computer) {
    let result = computer.accumulator_reg & computer.buss;

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }
    
    computer.accumulator_reg = result;
}

pub fn or_buss(computer: &mut Computer) {
    let result = computer.accumulator_reg | computer.buss;

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }
    
    computer.accumulator_reg = result;
}

pub fn logic_shift_left_16(computer: &mut Computer) {
    let result = (computer.accumulator_reg as u32) << 1;

    if result & 0x00010000 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    let result = result as u16;

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result;
}

pub fn logic_shift_left_32(computer: &mut Computer) {
    let result = (computer.help_reg as u32) << 16;
    let result = result + computer.accumulator_reg as u32;

    // TODO: ska detta utföras med u64 istället? förmodligen inte?
    if result & 0x00010000 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }
    
    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x80000000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result as u16;
    computer.help_reg = (result >> 16) as u16;
}

pub fn arithmetic_shift_right_16(computer: &mut Computer) {
    let result = computer.accumulator_reg as i16;

    if result & 0x1 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    let result = (result >> 1) as u16;

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result;
}

pub fn arithmetic_shift_right_32(computer: &mut Computer) {
    let result = (computer.help_reg as u32) << 16;
    let result = (result + computer.accumulator_reg as u32) as i32;

    if result & 0x1 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    let result = (result >> 1) as u32;

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x80000000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result as u16;
    computer.help_reg = (result >> 16) as u16;
}

pub fn logic_shift_right_16(computer: &mut Computer) {
    let result = computer.accumulator_reg;

    if result & 0x1 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    let result = result >> 1;

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result;
}

pub fn rotate_left_16(computer: &mut Computer) {
    let result = (computer.accumulator_reg >> 15) + (computer.accumulator_reg << 1);

    if result & 0x0001 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result;
}

pub fn rotate_left_32(computer: &mut Computer) {
    let result = (computer.help_reg as u32) << 16;
    let result = result + computer.accumulator_reg as u32;
    let result = (result >> 31) + (result << 1);

    if result & 0x00000001 != 0 {
        computer.flags |= 0b00100;
    } else {
        computer.flags &= 0b11011;
    }

    if result == 0 {
        computer.flags |= 0b10000;
    } else {
        computer.flags &= 0b01111;
    }

    if result & 0x8000 != 0 {
        computer.flags |= 0b01000;
    } else {
        computer.flags &= 0b10111;
    }

    computer.accumulator_reg = result as u16;
    computer.help_reg = (result >> 16) as u16;
}