use std::collections::HashMap;
use std::io::{self, Write};

use super::decode::{decode, InvalidInstructionError};
use super::Machine;

// @Todo complete
fn print_instruction(word: u16, trace: &mut dyn Write) -> io::Result<()> {
    let instruction = decode(word, &mut None);
    match instruction {
        Ok(instruction) => writeln!(trace, "\t{}", instruction)?,
        Err(InvalidInstructionError) => writeln!(trace, "\tInvalid Instruction")?,
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct LoadError {
    kind: LoadErrorKind,
    in_section_at_byte: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
enum LoadErrorKind {
    Eof {
        expected_bytes: usize,
        actual_bytes_remaining: usize,
    },
    InvalidAscii,
    InvalidHeader {
        word: u16,
    },
}

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    section_byte: usize,
}

impl<'a> Reader<'a> {
    fn read_str(&mut self, nbytes: u16) -> Result<&'a str, LoadError> {
        let nbytes = nbytes as usize;
        if self.bytes.len() - self.cursor < nbytes {
            return Err(LoadError {
                kind: LoadErrorKind::Eof {
                    expected_bytes: nbytes,
                    actual_bytes_remaining: self.bytes.len() - self.cursor,
                },
                in_section_at_byte: self.section_byte,
            });
        }
        let str = &self.bytes[self.cursor..self.cursor + nbytes];
        self.bytes = &self.bytes[self.cursor + nbytes..];

        std::str::from_utf8(str).map_err(|_| LoadError {
            kind: LoadErrorKind::InvalidAscii,
            in_section_at_byte: self.section_byte,
        })
    }

    fn read_word(&mut self) -> Result<u16, LoadError> {
        if self.bytes.len() - self.cursor < 2 {
            return Err(LoadError {
                kind: LoadErrorKind::Eof {
                    expected_bytes: 2,
                    actual_bytes_remaining: self.bytes.len() - self.cursor,
                },
                in_section_at_byte: self.section_byte,
            });
        }
        let b1 = self.bytes[self.cursor] as u16;
        let b2 = self.bytes[self.cursor + 1] as u16;
        let word = b1 << 8 | b2;
        self.bytes = &self.bytes[self.cursor + 2..];
        Ok(word)
    }
}

// @Todo check code and data addresses
// @Todo proper bounds checking errors
pub(super) fn load(
    bytes: &[u8],
    machine: &mut Machine,
    mut trace: Option<&mut dyn Write>,
) -> Result<(), LoadError> {
    use crate::{CODE_HEADER, DATA_HEADER, FILE_HEADER, LINE_HEADER, SYMBOL_HEADER};

    let mut reader = Reader {
        bytes,
        cursor: 0,
        section_byte: 0,
    };

    let mut label_addresses = HashMap::new();
    let mut file_names = Vec::new();

    let bytes = &mut &*bytes;
    while !bytes.is_empty() {
        let word = if let Ok(word) = reader.read_word() {
            word
        } else {
            break;
        };
        match word {
            CODE_HEADER => {
                let addr = reader.read_word()?;
                let ninstructions = reader.read_word()?;

                if let Some(trace) = trace.as_deref_mut() {
                    let _ = writeln!(trace, ".code");
                    let _ = writeln!(trace, ".addr {:x}", addr);

                    for label in label_addresses.get(&addr).unwrap_or(&Vec::new()) {
                        let _ = writeln!(trace, "{}:", label);
                    }
                }

                for i in 0..ninstructions {
                    let word = reader.read_word()?;
                    machine.memory[(addr + i) as usize] = word;
                    if let Some(trace) = &mut trace {
                        let _ = print_instruction(word, trace);
                    }
                }
            }
            DATA_HEADER => {
                let addr = reader.read_word()?;
                let nwords = reader.read_word()?;

                if let Some(trace) = trace.as_deref_mut() {
                    let _ = writeln!(trace, ".data");
                    let _ = writeln!(trace, ".addr {:x}", addr);

                    for label in label_addresses.get(&addr).unwrap_or(&Vec::new()) {
                        let _ = writeln!(trace, "{}:", label);
                    }
                }

                for i in 0..nwords {
                    let word = reader.read_word()?;
                    machine.memory[(addr + i) as usize] = word;
                    if let Some(trace) = &mut trace {
                        let _ = writeln!(trace, ".fill {}", word);
                    }
                }
            }
            SYMBOL_HEADER => {
                let addr = reader.read_word()?;
                let nbytes = reader.read_word()?;
                let symbol_name = reader.read_str(nbytes)?;
                // label addresses are only used for printing
                if trace.is_some() {
                    label_addresses
                        .entry(addr)
                        .or_insert(Vec::new())
                        .push(symbol_name);
                }
            }
            FILE_HEADER => {
                let nbytes = reader.read_word()?;
                let file_name = reader.read_str(nbytes)?;
                file_names.push(file_name);
                if let Some(trace) = trace.as_deref_mut() {
                    let _ = writeln!(
                        trace,
                        "; File index ({}) file: {}",
                        file_names.len() - 1,
                        file_name
                    );
                }
            }
            LINE_HEADER => {
                let addr = reader.read_word()?;
                let line = reader.read_word()?;
                let file_index = reader.read_word()?;
                if let Some(trace) = trace.as_deref_mut() {
                    let _ = writeln!(
                        trace,
                        "; Line number at address: {:x} for line {} in file index {}",
                        addr, line, file_index
                    );
                }
            }
            _ => {
                if let Some(trace) = trace.as_deref_mut() {
                    let _ = writeln!(trace, "; Error encountered");
                }
                return Err(LoadError {
                    kind: LoadErrorKind::InvalidHeader { word },
                    in_section_at_byte: reader.section_byte,
                });
            }
        }
    }

    Ok(())
}
