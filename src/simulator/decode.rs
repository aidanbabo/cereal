use super::{InstructionType, Instruction};

fn sext(word: u16, bit: u8) -> i16 {
    let bit = 1 << (bit - 1);
    let pos_mask = bit - 1;
    let neg_mask = !pos_mask;

    if (word & bit) > 0 {
        (word | neg_mask) as i16
    } else {
        (word & pos_mask) as i16
    }
}

fn branch(word: u16) -> Instruction {
    let op = (word & 0x0e00) >> 9;
    let immediate = sext(word & 0x01ff, 9);
    let ty = match op {
        0b000 => InstructionType::Nop,
        0b001 => InstructionType::Brp,
        0b010 => InstructionType::Brz,
        0b011 => InstructionType::Brzp,
        0b100 => InstructionType::Brn,
        0b101 => InstructionType::Brnp,
        0b110 => InstructionType::Brnz,
        0b111 => InstructionType::Brnzp,
        _ => unreachable!("Branch op code"),
    };
    Instruction {
        ty,
        rd: 0,
        rs: 0,
        rt: 0,
        immediate,
    }
}

fn arithmetic(word: u16) -> Instruction {
    let op = (word & 0x0038) >> 3;
    let rd = ((word & 0x0e00) >> 9) as u8;
    let rs = ((word & 0x01c0) >> 6) as u8;
    let ty = match op {
        0b000 => InstructionType::Add,
        0b001 => InstructionType::Mul,
        0b010 => InstructionType::Sub,
        0b011 => InstructionType::Div,
        _ => {
            let immediate = sext(word & 0x001f, 5);
            return Instruction {
                ty: InstructionType::Addi,
                rd,
                rs,
                rt: 0,
                immediate,
            }
        }
    };
    
    let rt = (word & 0x0007) as u8;
    Instruction {
        ty,
        rd,
        rs,
        rt,
        immediate: 0,
    }
}

fn cmp(word: u16) -> Instruction {
    let op = (word & 0x0180) >> 7;
    let rd = ((word & 0x0e00) >> 9) as u8;
    let rs = ((word & 0x01c0) >> 6) as u8;
    match op {
        0b00 => {
            let rt = (word & 0x0007) as u8;
            Instruction {
                ty: InstructionType::Cmp,
                rd,
                rs,
                rt,
                immediate: 0,
            }
        }
        0b01 => {
            let rt = (word & 0x0007) as u8;
            Instruction {
                ty: InstructionType::Cmpu,
                rd,
                rs,
                rt,
                immediate: 0,
            }
        }
        0b10 => {
            let immediate = sext(word & 0x007f, 7);
            Instruction {
                ty: InstructionType::Cmpi,
                rd,
                rs,
                rt: 0,
                immediate,
            }
        }
        0b11 => {
            let immediate = (word & 0x007f) as i16;
            Instruction {
                ty: InstructionType::Cmpiu,
                rd,
                rs,
                rt: 0,
                immediate,
            }
        }
        _ => unreachable!("Cmp op code"),
    }
}

fn subroutine(word: u16) -> Instruction {
    if (word & 0x0800) == 0 {
        let rt = ((word & 0x01c0) >> 6) as u8;
        Instruction {
            ty: InstructionType::Jsrr,
            rd: 0,
            rs: 0,
            rt,
            immediate: 0,
        }
    } else {
        let immediate = sext(word & 0x07ff, 11);
        Instruction {
            ty: InstructionType::Jsr,
            rd: 0,
            rs: 0,
            rt: 0,
            immediate,
        }
    }
}

fn logical(word: u16) -> Instruction {
    let op = (word & 0x0038) >> 3;
    let rd = ((word & 0x0e00) >> 9) as u8;
    let rs = ((word & 0x01c0) >> 6) as u8;
    let ty = match op {
        0b000 => InstructionType::And,
        0b001 => InstructionType::Not,
        0b010 => InstructionType::Or,
        0b011 => InstructionType::Xor,
        _ => {
            let immediate = sext(word & 0x001f, 5);
            return Instruction {
                ty: InstructionType::Andi,
                rd,
                rs,
                rt: 0,
                immediate,
            }
        }
    };
    
    // @Todo should we zero out rt for NOT ?
    let rt = (word & 0x0007) as u8;
    Instruction {
        ty,
        rd,
        rs,
        rt,
        immediate: 0,
    }
}

fn load(word: u16) -> Instruction {
    let rd = ((word & 0x0e00) >> 9) as u8;
    let rs = ((word & 0x01c0) >> 6) as u8;
    let immediate = sext(word & 0x003f, 6);
    Instruction {
        ty: InstructionType::Ldr,
        rd,
        rs,
        rt: 0,
        immediate,
    }
}

fn store(word: u16) -> Instruction {
    let rd = ((word & 0x0e00) >> 9) as u8;
    let rt = ((word & 0x01c0) >> 6) as u8;
    let immediate = sext(word & 0x003f, 6);
    Instruction {
        ty: InstructionType::Str,
        rd,
        rs: 0,
        rt,
        immediate,
    }
}

fn rti(_: u16) -> Instruction {
    Instruction {
        ty: InstructionType::Rti,
        rd: 0,
        rs: 0,
        rt: 0,
        immediate: 0,
    }
}

fn konst(word: u16) -> Instruction {
    let rd = ((word & 0x0e00) >> 9) as u8;
    let immediate = sext(word & 0x01ff, 9);
    Instruction {
        ty: InstructionType::Const,
        rd,
        rs: 0,
        rt: 0,
        immediate,
    }
}

fn shift(word: u16) -> Instruction {
    let op = (word & 0x0030) >> 4;
    let rd = ((word & 0x0e00) >> 9) as u8;
    let rs = ((word & 0x01c0) >> 6) as u8;
    let ty = match op {
        0b00 => InstructionType::Sll,
        0b01 => InstructionType::Sra,
        0b10 => InstructionType::Srl,
        0b11 => {
            let rt = (word & 0x0007) as u8;
            return Instruction {
                ty: InstructionType::Mod,
                rd,
                rs,
                rt,
                immediate: 0,
            };
        }
        _ => unreachable!("Shift op code")
    };
    
    let immediate = (word & 0x000f) as i16;
    Instruction {
        ty,
        rd,
        rs,
        rt: 0,
        immediate,
    }
}

fn jump(word: u16) -> Instruction {
    if (word & 0x0800) == 0 {
        let rt = ((word & 0x01c0) >> 6) as u8;
        Instruction {
            ty: InstructionType::Jmpr,
            rd: 0,
            rs: 0,
            rt,
            immediate: 0,
        }
    } else {
        let immediate = sext(word & 0x07ff, 11);
        Instruction {
            ty: InstructionType::Jmp,
            rd: 0,
            rs: 0,
            rt: 0,
            immediate,
        }
    }
}

fn hiconst(word: u16) -> Instruction {
    let rd = ((word & 0x0e00) >> 9) as u8;
    let immediate = (word & 0x00ff) as i16;
    Instruction {
        ty: InstructionType::Hiconst,
        rd,
        rs: 0,
        rt: 0,
        immediate,
    }
}

fn trap(word: u16) -> Instruction {
    let immediate = (word & 0x00ff) as i16;
    Instruction {
        ty: InstructionType::Trap,
        rd: 0,
        rs: 0,
        rt: 0,
        immediate,
    }
}

pub(super) fn decode(word: u16) -> Instruction {
    let op_code = word >> 12;
    match op_code {
        0x0 => branch(word),
        0x1 => arithmetic(word),
        0x2 => cmp(word),
        0x3 => unreachable!(),
        0x4 => subroutine(word),
        0x5 => logical(word),
        0x6 => load(word),
        0x7 => store(word),
        0x8 => rti(word),
        0x9 => konst(word),
        0xA => shift(word),
        0xB => unreachable!(),
        0xC => jump(word),
        0xD => hiconst(word),
        0xE => unreachable!(),
        0xF => trap(word),
        _ => unreachable!("Top-level op code"),
    }
}
