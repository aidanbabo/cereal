pub mod loader;
pub mod decode;

#[derive(Copy, Clone)]
#[repr(u8)]
enum InstructionType {
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
    Addi,
    Mod,
    And,
    Not,
    Or,
    Xor,
    Andi,
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
}

impl InstructionType {
    pub fn to_mnemonic(self) -> &'static str {
        use InstructionType::*;
        match self {
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
            Addi => "add",
            Mod => "mod",
            And => "and",
            Not => "not",
            Or => "or",
            Xor => "xor",
            Andi => "and",
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
        }
    }
}

struct Instruction {
    ty: InstructionType,
    rd: u8,
    rs: u8,
    rt: u8,
    immediate: i16,
}

const MEMORY_SIZE: usize = 1 << 16;

pub struct Machine {
    pc: u16,
    psr: u16,
    registers: [u16; 8],
    memory: Box<[u16; MEMORY_SIZE]>,
}

impl Machine {
    pub fn new() -> Self {
        let memory = {
            let vec = vec![0; MEMORY_SIZE];
            let boxed_slice: Box<[u16]> = vec.into_boxed_slice();
            let ptr = Box::into_raw(boxed_slice) as *mut [u16; MEMORY_SIZE];
            // Is it worth having unsave to save a single usize on the Machine ?
            unsafe { Box::from_raw(ptr) }
        };
        Machine {
            pc: 0x8200,
            psr: 0x8002,
            registers: Default::default(),
            memory,
        }
    }
    
    pub fn step(&mut self) -> Result<(), ()> {
        Ok(())
    }
}