use std::collections::HashMap;

fn print_instruction(_word: u16) {
    
}

pub fn disassemble(bytes: &[u8]) {
    use crate::{CODE_HEADER, DATA_HEADER, SYMBOL_HEADER, FILE_HEADER, LINE_HEADER};
    
    fn read_str<'a>(bytes: &mut &'a[u8], nbytes: u16) -> &'a str {
        let nbytes = nbytes as usize;
        let str = &bytes[..nbytes];
        *bytes = &bytes[nbytes..];
        std::str::from_utf8(str).unwrap()
    }
    
    fn read_word(bytes: &mut &[u8]) -> u16 {
        let b1 = bytes[0] as u16;
        let b2 = bytes[1] as u16;
        let word = b1 << 8 | b2;
        *bytes = &bytes[2..];
        word
    }

    let mut label_addresses = HashMap::new();
    let mut file_names = Vec::new();
    
    let bytes = &mut &*bytes;
    loop {
        let word = read_word(bytes);
        match word {
            CODE_HEADER => {
                let addr = read_word(bytes);
                let ninstructions = read_word(bytes);

                println!(".code");
                println!(".addr {:x}", addr);

                for label in label_addresses.get(&addr).unwrap_or(&Vec::new()) {
                    println!("{}:", label);
                }
                
                for _ in 0..ninstructions {
                    let word = read_word(bytes);
                    print_instruction(word);
                }
            } 
            DATA_HEADER => {
                let addr = read_word(bytes);
                let nwords = read_word(bytes);
                
                println!(".data");
                println!(".addr {:x}", addr);
                
                for label in label_addresses.get(&addr).unwrap_or(&Vec::new()) {
                    println!("{}:", label);
                }
                
                for _ in 0..nwords {
                    let word = read_word(bytes);
                    println!(".fill {}", word);
                }
            } 
            SYMBOL_HEADER => {
                let addr = read_word(bytes);
                let nbytes = read_word(bytes);
                let symbol_name = read_str(bytes, nbytes);
                label_addresses.entry(addr).or_insert(Vec::new()).push(symbol_name);
            } 
            FILE_HEADER => {
                let nbytes = read_word(bytes);
                let file_name = read_str(bytes, nbytes);
                file_names.push(file_name);
                println!("; File index ({}) file: {}", file_names.len() - 1, file_name);
            } 
            LINE_HEADER => {
                let addr = read_word(bytes);
                let line = read_word(bytes);
                let file_index = read_word(bytes);
                println!("; Line number at address: {:x} for line {} in file index {}", addr, line, file_index);
            }
            _ => {
                println!("; GUH ?!?!?! WHAT THE HELL ?!?!?");
            }
        }
    }
}
