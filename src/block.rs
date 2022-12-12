use crate::asm_instruction::InstructionWithLabel;
use std::borrow::Cow;

#[derive(Debug)]
pub enum Data<'a> {
    Block(u16),
    Stringz(Cow<'a, str>),
    Word(i16),
}

#[derive(Debug)]
pub enum BlockType<'a> {
    Code(Vec<InstructionWithLabel<'a>>),
    Data(Vec<Data<'a>>),
}

#[derive(Debug)]
pub struct Block<'a> {
    pub addr: Option<u16>,
    pub aligned: bool,
    pub labels: Vec<&'a str>,
    pub ty: BlockType<'a>,
}

impl<'a> Block<'a> {
    pub fn is_empty(&self) -> bool {
        self.addr.is_none()
            && self.labels.is_empty()
            && match &self.ty {
                BlockType::Code(instructions) => instructions.is_empty(),
                BlockType::Data(data) => data.is_empty(),
            }
    }

    pub fn size(&self) -> u16 {
        match &self.ty {
            BlockType::Code(instructions) => instructions.len() as u16,
            BlockType::Data(data) => {
                let mut size = 0;
                for datum in data {
                    let new = match datum {
                        Data::Block(s) => *s,
                        Data::Word(_) => 1,
                        Data::Stringz(s) => s.len() as u16 + 1,
                    };
                    size += new;
                }
                size
            }
        }
    }
}
