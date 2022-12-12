use std::collections::HashMap;

use crate::asm_instruction::{InstructionType, InstructionWithLabel};
use crate::block::{Block, BlockType, Data};
use crate::{number_fits, CODE_HEADER, DATA_HEADER, SYMBOL_HEADER};

pub fn link(
    blocks: &mut [Block],
    constants: &HashMap<&str, i32>,
    debug_info: bool,
) -> Result<Vec<u8>, ()> {
    // println!("PRINTED:");
    // printer::print_blocks(blocks, constants).unwrap();

    if let Err(errors) = expand_psuedo_instructions(blocks, constants) {
        for error in errors {
            println!("ERROR: {}", error);
        }
        return Err(());
    }

    // println!("EXPANDED:");
    // printer::print_blocks(blocks, constants).unwrap();

    let labels = match patch(blocks) {
        Ok(labels) => labels,
        Err(errors) => {
            for error in errors {
                println!("ERROR: {}", error);
            }
            return Err(());
        }
    };

    // println!("PATCHED:");
    // printer::print_blocks(blocks, constants).unwrap();

    let bytes = write_object_code(&*blocks, &labels, debug_info);
    Ok(bytes)
}

fn expand_psuedo_instructions(
    blocks: &mut [Block],
    constants: &HashMap<&str, i32>,
) -> Result<(), Vec<String>> {
    let mut errors = vec![];

    for block in blocks {
        if let BlockType::Code(instructions) = &mut block.ty {
            for i in (0..instructions.len()).rev() {
                let mut instruction = &mut instructions[i];
                match instruction.ty {
                    InstructionType::Lea => {
                        instruction.ty = InstructionType::Const;
                        let mut instruction = *instruction;
                        instruction.ty = InstructionType::Hiconst;
                        instructions.insert(i + 1, instruction);
                    }
                    InstructionType::Lc => {
                        let value = if let Some(value) = constants.get(instruction.label.unwrap()) {
                            value
                        } else {
                            errors.push(format!("No such label '{}'.", instruction.label.unwrap()));
                            continue;
                        };

                        instruction.ty = InstructionType::Const;
                        instruction.immediate = value & 0x000001ff;
                        instruction.label = None;

                        let high_bits = (value & 0x0000ff00) >> 8;
                        if high_bits == 0 || high_bits == 0xff {
                            continue;
                        }

                        let mut instruction = *instruction;
                        instruction.ty = InstructionType::Hiconst;
                        instruction.immediate = high_bits;
                        instructions.insert(i + 1, instruction);
                    }
                    InstructionType::Ret => {
                        instruction.ty = InstructionType::Jmpr;
                        instruction.rs = 7;
                    }
                    _ => {}
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn patch<'a>(blocks: &mut [Block<'a>]) -> Result<HashMap<&'a str, u16>, Vec<String>> {
    struct Region<'s> {
        label: &'s str,
        start: u16,
        end: u16,
    }

    let mut addresses = HashMap::new();
    let mut errors = Vec::new();
    let mut code_addr = 0;
    let mut data_addr = 0x2000;

    // we could probably make this n*log(n) instead of n^2
    let mut regions = Vec::new();

    for block in &mut *blocks {
        let size = block.size();
        let addr = match &block.ty {
            BlockType::Code(_) => &mut code_addr,
            BlockType::Data(_) => &mut data_addr,
        };

        if let Some(a) = block.addr {
            *addr = a;
        }

        if block.aligned && *addr & 0xf != 0 {
            *addr |= 0xf;
            *addr += 1;
        }

        block.addr = Some(*addr);

        for label in &block.labels {
            if let Some(old_addr) = addresses.insert(*label, *addr) {
                errors.push(format!(
                    "Label '{}' is already defined at address {:x}.",
                    label, old_addr
                ));
            }
        }

        let end = *addr + size;
        let label = block.labels.first().unwrap_or(&"Unlabeled");
        let region = Region {
            label,
            start: *addr,
            end,
        };

        for &Region { label, start, end } in &regions {
            if region.end <= start || end <= region.start {
                continue;
            }
            errors.push(format!(
                "Overlapping blocks: Block {} is {:x}-{:x} and block {} is {:x}-{:x}.",
                label, start, end, region.label, region.start, region.end
            ));
        }

        match &block.ty {
            BlockType::Code(_) => {
                let in_user = region.end <= 0x2000;
                let in_os = region.start >= 0x8000 && region.end <= 0xA000;
                if !(in_user || in_os) {
                    errors.push(format!("Code block labeled {} is not in the correct section of memory. Range {:x}-{:x}.", region.label, region.start, region.end));
                }
            }
            BlockType::Data(_) => {
                let in_user = region.start >= 0x2000 && region.end <= 0x8000;
                let in_os = region.start >= 0xA000;
                if !(in_user || in_os) {
                    errors.push(format!("Data block labeled {} is not in the correct section of memory. Range {:x}-{:x}.", region.label, region.start, region.end));
                }
            }
        }

        regions.push(region);

        *addr = end;
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    use InstructionType::*;
    for block in blocks {
        let BlockType::Code(instructions) = &mut block.ty else { continue };
        let top_addr = block.addr.unwrap() as i32;
        for (i, instruction) in instructions.iter_mut().enumerate() {
            let Some(label) = instruction.label else { continue };
            instruction.label = None;
            let Some(address) = addresses.get(&label) else {
                errors.push(format!("Label '{}' is not defined.", label));
                continue;
            };

            let current = top_addr + i as i32;
            match instruction.ty {
                Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp | Jmp => {
                    instruction.immediate = (*address) as i32 - current - 1;
                    if !number_fits(
                        instruction.immediate,
                        true,
                        if matches!(instruction.ty, Jmp) { 11 } else { 9 },
                    ) {
                        errors.push(format!("Jump to label '{}' is too far.", label));
                        continue;
                    }
                }
                Jsr => {
                    if address & 0x0f != 0 {
                        errors.push(format!(
                            "Cannot jump to subroutine of not aligned label '{}'.",
                            label
                        ));
                        continue;
                    }
                    let address = address >> 4;
                    instruction.immediate = address as i32;
                    if !number_fits(instruction.immediate, true, 11) {
                        errors.push(format!("Jump to subroutine to label '{}' is too far. You cannot jump to subroutines in user/os space if you are in os/user space.", label));
                        continue;
                    }
                }
                Const => instruction.immediate = (*address as i32) & 0x1ff,
                Hiconst => instruction.immediate = ((*address as i32) & 0xff00) >> 8,
                _ => {}
            }
        }
    }

    if errors.is_empty() {
        Ok(addresses)
    } else {
        Err(errors)
    }
}

fn write_object_code(blocks: &[Block], labels: &HashMap<&str, u16>, debug_info: bool) -> Vec<u8> {
    fn write_be(bytes: &mut Vec<u8>, short: u16) {
        bytes.push(((short & 0xff00) >> 8) as u8);
        bytes.push((short & 0xff) as u8);
    }

    fn write_be_signed(bytes: &mut Vec<u8>, short: i16) {
        bytes.push(((short >> 8) & 0xff) as u8);
        bytes.push((short & 0xff) as u8);
    }

    let mut bytes = Vec::new();

    if debug_info {
        for (label, address) in labels {
            write_be(&mut bytes, SYMBOL_HEADER);
            write_be(&mut bytes, *address);
            write_be(&mut bytes, label.len() as u16);
            bytes.extend_from_slice(label.as_bytes());
        }
    }

    for block in blocks {
        let address = block.addr.unwrap();
        let size = block.size();
        match &block.ty {
            BlockType::Code(instructions) => {
                if instructions.is_empty() {
                    continue;
                }
                write_be(&mut bytes, CODE_HEADER);
                write_be(&mut bytes, address);
                write_be(&mut bytes, size);
                bytes.reserve(size as usize * 2);

                for instruction in instructions {
                    let encoded = encode_instruction(instruction);
                    write_be(&mut bytes, encoded);
                }
            }
            BlockType::Data(data) => {
                if data.is_empty() {
                    continue;
                }
                write_be(&mut bytes, DATA_HEADER);
                write_be(&mut bytes, address);
                write_be(&mut bytes, size);
                bytes.reserve(size as usize * 2);

                for datum in data {
                    match datum {
                        Data::Block(s) => {
                            for _ in 0..*s {
                                write_be(&mut bytes, 0);
                            }
                        }
                        Data::Word(w) => {
                            write_be_signed(&mut bytes, *w);
                        }
                        Data::Stringz(s) => {
                            for b in s.as_bytes() {
                                write_be(&mut bytes, *b as u16);
                            }
                            write_be(&mut bytes, 0);
                        }
                    }
                }
            }
        }
    }

    bytes
}

fn encode_instruction(instruction: &InstructionWithLabel) -> u16 {
    use InstructionType::*;

    let mut encoded = instruction.ty.encoding_base();
    match instruction.ty {
        Nop | Rti => {}
        Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp => {
            encoded |= instruction.immediate as u16 & 0x1ff;
        }
        Mul | Sub | Div | Mod | Or | Xor => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= instruction.rt as u16;
        }
        Add | And => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            if instruction.rt != -1 {
                encoded |= instruction.rt as u16;
            } else {
                encoded |= (instruction.immediate & 0x1f) as u16;
                encoded |= 1 << 5;
            }
        }
        Not => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
        }
        Ldr => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.immediate & 0x3f) as u16;
        }
        Str => {
            encoded |= (instruction.rt as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.immediate & 0x3f) as u16;
        }
        Const => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.immediate & 0x1ff) as u16;
        }
        Hiconst => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.immediate & 0xff) as u16;
        }
        Cmp | Cmpu => {
            encoded |= (instruction.rs as u16) << 9;
            encoded |= instruction.rt as u16;
        }
        Cmpi | Cmpiu => {
            encoded |= (instruction.rs as u16) << 9;
            encoded |= (instruction.immediate & 0x7f) as u16;
        }
        Sll | Sra | Srl => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.immediate & 0xf) as u16;
        }
        Jsrr | Jmpr => {
            encoded |= (instruction.rs as u16) << 6;
        }
        Jsr | Jmp => {
            encoded |= (instruction.immediate & 0x7ff) as u16;
        }
        Trap => {
            encoded |= (instruction.immediate & 0xff) as u16;
        }
        _ => unreachable!(),
    }
    encoded
}
