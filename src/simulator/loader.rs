use std::collections::HashMap;
use std::io::{self, Write};

use super::Machine;
use super::decode::{decode, InvalidInstructionError};

// @Todo complete
fn print_instruction(word: u16, trace: &mut dyn Write) -> io::Result<()> {
    let instruction = decode(word, None);
    match instruction {
        Ok(instruction) => writeln!(trace, "\t{}", instruction.ty.to_mnemonic())?,
        Err(InvalidInstructionError) => writeln!(trace, "\tInvalid Instruction")?,
    }
    Ok(())
}

// @Todo check code and data addresses
// @Todo failure to load do to malformed file, bad header, not enough bytes left in file, etc.
// @Todo proper error handling on malformed utf-8 strings
// @Todo proper bounds checking errors
pub(super) fn load(bytes: &[u8], machine: &mut Machine, mut trace: Option<&mut dyn Write>) -> io::Result<()> {
    use crate::{CODE_HEADER, DATA_HEADER, SYMBOL_HEADER, FILE_HEADER, LINE_HEADER};
    
    fn read_str<'a>(bytes: &mut &'a[u8], nbytes: u16) -> &'a str {
        let nbytes = nbytes as usize;
        assert!(bytes.len() >= nbytes);
        let str = &bytes[..nbytes];
        *bytes = &bytes[nbytes..];
        std::str::from_utf8(str).expect("Ascii symbol in object file is well formed utf-8")
    }
    
    fn read_word(bytes: &mut &[u8]) -> u16 {
        assert!(bytes.len() >= 2);
        let b1 = bytes[0] as u16;
        let b2 = bytes[1] as u16;
        let word = b1 << 8 | b2;
        *bytes = &bytes[2..];
        word
    }

    let mut label_addresses = HashMap::new();
    let mut file_names = Vec::new();
    
    let bytes = &mut &*bytes;
    while bytes.len() > 0 {
        let word = read_word(bytes);
        match word {
            CODE_HEADER => {
                let addr = read_word(bytes);
                let ninstructions = read_word(bytes);

                if let Some(trace) = trace.as_deref_mut() {
                    writeln!(trace, ".code")?;
                    writeln!(trace, ".addr {:x}", addr)?;

                    for label in label_addresses.get(&addr).unwrap_or(&Vec::new()) {
                        writeln!(trace, "{}:", label)?;
                    }
                }

                for i in 0..ninstructions {
                    let word = read_word(bytes);
                    machine.memory[(addr + i) as usize] = word;
                    if let Some(trace) = &mut trace {
                        print_instruction(word, trace)?;
                    }
                }
            } 
            DATA_HEADER => {
                let addr = read_word(bytes);
                let nwords = read_word(bytes);
                
                if let Some(trace) = trace.as_deref_mut() {
                    writeln!(trace, ".data")?;
                    writeln!(trace, ".addr {:x}", addr)?;
                
                    for label in label_addresses.get(&addr).unwrap_or(&Vec::new()) {
                        writeln!(trace, "{}:", label)?;
                    }
                }
                
                for i in 0..nwords {
                    let word = read_word(bytes);
                    machine.memory[(addr + i) as usize] = word;
                    if let Some(trace) = &mut trace {
                        writeln!(trace, ".fill {}", word)?;
                    }
                }
            } 
            SYMBOL_HEADER => {
                let addr = read_word(bytes);
                let nbytes = read_word(bytes);
                let symbol_name = read_str(bytes, nbytes);
                // label addresses are only used for printing
                if trace.is_some() {
                    label_addresses.entry(addr).or_insert(Vec::new()).push(symbol_name);
                }
            } 
            FILE_HEADER => {
                let nbytes = read_word(bytes);
                let file_name = read_str(bytes, nbytes);
                file_names.push(file_name);
                if let Some(trace) = trace.as_deref_mut() {
                    writeln!(trace, "; File index ({}) file: {}", file_names.len() - 1, file_name)?;
                }
            } 
            LINE_HEADER => {
                let addr = read_word(bytes);
                let line = read_word(bytes);
                let file_index = read_word(bytes);
                if let Some(trace) = trace.as_deref_mut() {
                    writeln!(trace, "; Line number at address: {:x} for line {} in file index {}", addr, line, file_index)?;
                }
            }
            _ => {
                if let Some(trace) = trace.as_deref_mut() {
                    writeln!(trace, "; GUH ?!?!?! WHAT THE HELL ?!?!?")?;
                }
            }
        }
    }
    
    Ok(())
}
