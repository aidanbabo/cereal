pub struct Id {}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,
}

pub struct Binary {
    pub op: BinaryOp,
    pub left: Id,
    pub right: Id,
}

pub enum UnaryOp {
    Minus,
    Not,
}

pub struct Unary {
    pub op: UnaryOp,
    pub id: Id,
}

pub struct Call {
    pub args: Vec<Id>,
}

pub struct Load {
    pub id: Id,
}

pub struct Store {
    pub id: Id,
}

pub enum InstructionType {
    Binary(Binary),
    Unary(Unary),
    Call(Call),
    Load(Load),
    Store(Store),
}

pub struct Instruction {
    pub id: Id,
    pub ty: InstructionType,
}
