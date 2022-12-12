use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct InstructionWithLabel<'a> {
    pub ty: InstructionType,
    pub rd: i8,
    pub rs: i8,
    pub rt: i8,
    pub immediate: i32,
    pub label: Option<&'a str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstructionType {
    Nop,
    Brp,
    Brz,
    Brzp,
    Brn,
    Brnp,
    Brnz,
    Brnzp,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    And,
    Not,
    Or,
    Xor,
    Ldr,
    Str,
    Const,
    Hiconst,
    Cmp,
    Cmpu,
    Cmpi,
    Cmpiu,
    Sll,
    Sra,
    Srl,
    Jsrr,
    Jsr,
    Jmpr,
    Jmp,
    Trap,
    Rti,
    Ret,
    Lea,
    Lc,
}

impl InstructionType {
    pub fn encoding_base(self) -> u16 {
        use InstructionType::*;
        match self {
            Nop => 0x0000,
            Brp => 0x0200,
            Brz => 0x0400,
            Brzp => 0x0600,
            Brn => 0x0800,
            Brnp => 0x0a00,
            Brnz => 0x0c00,
            Brnzp => 0x0e00,
            Add => 0x1000,
            Mul => 0x1008,
            Sub => 0x1010,
            Div => 0x1018,
            Mod => 0xa030,
            And => 0x5000,
            Not => 0x5008,
            Or => 0x5010,
            Xor => 0x5018,
            Ldr => 0x6000,
            Str => 0x7000,
            Const => 0x9000,
            Hiconst => 0xd000,
            Cmp => 0x2000,
            Cmpu => 0x2080,
            Cmpi => 0x2100,
            Cmpiu => 0x2180,
            Sll => 0xa000,
            Sra => 0xa010,
            Srl => 0xa020,
            Jsrr => 0x4000,
            Jsr => 0x4800,
            Jmpr => 0xc000,
            Jmp => 0xc800,
            Trap => 0xf000,
            Rti => 0x8000,
            Ret | Lea | Lc => panic!(
                "Internal error: {} should never get to the code generation stage!",
                self
            ),
        }
    }
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InstructionType::*;

        let s = match *self {
            Nop => "nop",
            Brp => "brp",
            Brz => "brz",
            Brzp => "brzp",
            Brn => "brn",
            Brnp => "brnp",
            Brnz => "brnz",
            Brnzp => "brnzp",
            Add => "add",
            Mul => "mul",
            Sub => "sub",
            Div => "div",
            Mod => "mod",
            And => "and",
            Not => "not",
            Or => "or",
            Xor => "xor",
            Ldr => "ldr",
            Str => "str",
            Const => "const",
            Hiconst => "hiconst",
            Cmp => "cmp",
            Cmpu => "cmpu",
            Cmpi => "cmpi",
            Cmpiu => "cmpiu",
            Sll => "sll",
            Sra => "sra",
            Srl => "srl",
            Jsrr => "jsrr",
            Jsr => "jsr",
            Jmpr => "jmpr",
            Jmp => "jmp",
            Trap => "trap",
            Rti => "rti",
            Ret => "ret",
            Lea => "lea",
            Lc => "lc",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Reg {
    Rd,
    Rs,
    Rt,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Operand {
    Register {
        register: Reg,
    },
    Immediate {
        signed: bool,
        bits: u8,
    },
    Label,
    RegisterOrImmediate {
        register: Reg,
        signed: bool,
        bits: u8,
    },
}

pub(crate) fn instruction_operands(
    instruction_type: InstructionType,
    ops: &mut [Operand],
) -> &[Operand] {
    use InstructionType::*;
    use Operand::*;
    use Reg::*;
    let specs: &'static [Operand] = match instruction_type {
        Nop | Ret | Rti => &[],
        Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp | Jsr | Jmp => &[Label],
        Lea | Lc => &[Register { register: Rd }, Label],
        And | Add => &[
            Register { register: Rd },
            Register { register: Rs },
            RegisterOrImmediate {
                register: Rt,
                signed: true,
                bits: 5,
            },
        ],
        Mul | Sub | Div | Mod | Or | Xor => &[
            Register { register: Rd },
            Register { register: Rs },
            Register { register: Rt },
        ],
        Sll | Sra | Srl => &[
            Register { register: Rd },
            Register { register: Rs },
            Immediate {
                signed: false,
                bits: 4,
            },
        ],
        Not => &[Register { register: Rd }, Register { register: Rs }],
        Ldr => &[
            Register { register: Rd },
            Register { register: Rs },
            Immediate {
                signed: true,
                bits: 6,
            },
        ],
        Str => &[
            Register { register: Rt },
            Register { register: Rs },
            Immediate {
                signed: true,
                bits: 6,
            },
        ],
        Const => &[
            Register { register: Rd },
            Immediate {
                signed: true,
                bits: 9,
            },
        ],
        Hiconst => &[
            Register { register: Rd },
            Immediate {
                signed: false,
                bits: 8,
            },
        ],
        Cmp | Cmpu => &[Register { register: Rs }, Register { register: Rt }],
        Cmpi => &[
            Register { register: Rs },
            Immediate {
                signed: true,
                bits: 7,
            },
        ],
        Cmpiu => &[
            Register { register: Rs },
            Immediate {
                signed: false,
                bits: 7,
            },
        ],
        Jsrr | Jmpr => &[Register { register: Rs }],
        Trap => &[Immediate {
            signed: false,
            bits: 8,
        }],
    };
    ops[..specs.len()].copy_from_slice(specs);
    &ops[..specs.len()]
}

pub mod insn {
    use super::*;
    use InstructionType::*;

    pub fn add(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Add,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn sub(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Sub,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn mul(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Mul,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn div(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Div,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn mod_(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Mod,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn addi(rd: i8, rs: i8, immediate: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Add,
            rd,
            rs,
            rt: -1,
            immediate,
            label: None,
        }
    }

    pub fn and(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: And,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn not(rd: i8, rs: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Not,
            rd,
            rs,
            rt: -1,
            immediate: -1,
            label: None,
        }
    }

    pub fn xor(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Xor,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn or(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Or,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn ldr(value: i8, addr: i8, offset: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Ldr,
            rd: value,
            rs: addr,
            rt: -1,
            immediate: offset,
            label: None,
        }
    }

    pub fn str(value: i8, addr: i8, offset: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Str,
            rd: -1,
            rs: addr,
            rt: value,
            immediate: offset,
            label: None,
        }
    }

    pub fn konst(dest: i8, value: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Const,
            rd: dest,
            rs: -1,
            rt: -1,
            immediate: value,
            label: None,
        }
    }

    pub fn hiconst(dest: i8, value: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Hiconst,
            rd: dest,
            rs: -1,
            rt: -1,
            immediate: value,
            label: None,
        }
    }

    pub fn jsr(dest: &str) -> InstructionWithLabel<'_> {
        InstructionWithLabel {
            ty: Jsr,
            rd: -1,
            rs: -1,
            rt: -1,
            immediate: -1,
            label: Some(dest),
        }
    }

    pub fn jmpr(dest: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Jmpr,
            rd: -1,
            rs: dest,
            rt: -1,
            immediate: -1,
            label: None,
        }
    }

    pub fn lea(reg: i8, label: &str) -> InstructionWithLabel<'_> {
        InstructionWithLabel {
            ty: Lea,
            rd: reg,
            rs: -1,
            rt: -1,
            immediate: -1,
            label: Some(label),
        }
    }
}
