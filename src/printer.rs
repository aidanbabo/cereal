use std::collections::HashMap;
use std::io::{self, Write};

use crate::instruction_operands;
use crate::{Block, BlockType, Data, InstructionWithLabel, Operand, Reg};

fn write_instruction(writer: &mut dyn Write, instruction: &InstructionWithLabel) -> io::Result<()> {
    write!(writer, "\t{}", instruction.ty)?;
    let ops = &mut [Operand::Label; 3];
    let ops = instruction_operands(instruction.ty, &mut ops[..]);

    for (i, op) in ops.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        } else {
            write!(writer, " ")?;
        }

        match op {
            Operand::Label => match instruction.label {
                Some(label) => write!(writer, "{}", label)?,
                None => write!(writer, "#{}", instruction.immediate)?,
            },
            Operand::Register { register } => {
                let number = match register {
                    Reg::Rd => instruction.rd,
                    Reg::Rs => instruction.rs,
                    Reg::Rt => instruction.rt,
                };
                write!(writer, "r{}", number)?;
            }
            Operand::Immediate { .. } => write!(writer, "#{}", instruction.immediate)?,
            Operand::RegisterOrImmediate { register, .. } => {
                if instruction.immediate == i32::MAX {
                    let number = match register {
                        Reg::Rd => instruction.rd,
                        Reg::Rs => instruction.rs,
                        Reg::Rt => instruction.rt,
                    };
                    write!(writer, "r{}", number)?;
                } else {
                    write!(writer, "#{}", instruction.immediate)?;
                }
            }
        }
    }

    writeln!(writer)?;

    Ok(())
}

fn write_instructions(
    writer: &mut dyn Write,
    instructions: &[InstructionWithLabel],
) -> io::Result<()> {
    for instruction in instructions {
        write_instruction(writer, instruction)?;
    }
    Ok(())
}

fn write_data(writer: &mut dyn Write, data: &[Data]) -> io::Result<()> {
    for datum in data {
        match datum {
            Data::Block(size) => writeln!(writer, ".blkw x{:x}", size)?,
            Data::Stringz(string) => writeln!(writer, ".stringz \"{}\"", string)?,
            Data::Word(word) => writeln!(writer, ".fill #{}", word)?,
        }
    }
    Ok(())
}

fn write_block(
    writer: &mut dyn Write,
    block: &Block,
    constants: &HashMap<&str, i32>,
) -> io::Result<()> {
    if let Some(addr) = block.addr {
        if let BlockType::Code(_) = block.ty {
            writeln!(writer, ".code")?;
        } else {
            writeln!(writer, ".data")?;
        }
        writeln!(writer, ".addr x{:x}", addr)?;
    }
    if block.aligned {
        writeln!(writer, ".falign")?;
    }
    for label in &block.labels {
        if let Some(&c) = constants.get(label) {
            if c < 0 {
                writeln!(writer, "{} .const #{}", label, c)?;
            } else {
                writeln!(writer, "{} .uconst #{}", label, c)?;
            }
        } else {
            writeln!(writer, "{}:", label)?;
        }
    }

    match &block.ty {
        BlockType::Code(instructions) => write_instructions(writer, instructions)?,
        BlockType::Data(data) => write_data(writer, data)?,
    }

    Ok(())
}

pub fn write_blocks(
    writer: &mut dyn Write,
    blocks: &[Block],
    constants: &HashMap<&str, i32>,
) -> io::Result<()> {
    for (i, block) in blocks.iter().enumerate() {
        if i != 0 {
            writeln!(writer)?;
        }
        write_block(writer, block, constants)?;
    }

    Ok(())
}

pub fn print_blocks(blocks: &[Block], constants: &HashMap<&str, i32>) -> io::Result<()> {
    write_blocks(&mut io::stdout().lock(), blocks, constants)
}
