use crate::chunk::{Chunk, OpCode};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;
    while offset < self.get_code().len() {
        disassemble_instruction(&self, &mut offset);
    }
    }
}


pub fn disassemble_instruction(chunk: &Chunk, offset: &mut usize) {
    print!("{:04} ", offset);

    if *offset > 0 && chunk.get_lines()[*offset] == chunk.get_lines()[*offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.get_lines()[*offset]);
    }

    let instruction: &u8 = &chunk.get_code()[*offset];
    match instruction {
        0 => {
            constant_instruction(OpCode::OpConstant, chunk, offset);
        }
        1 => {
            simple_instruction(OpCode::OpReturn, offset);
        }
        _ => {
            println!("Unkown instruction: {}", instruction);
            *offset += 1;
        }
    }
    
}

fn simple_instruction(instruction: OpCode, offset: &mut usize) {
    // print the name of op code
    print!("{:?}", instruction);
    *offset += 1;
}

fn constant_instruction(instruction: OpCode, chunk: &Chunk, offset: &mut usize) {
    let constant: &u8 = &chunk.get_code()[*offset + 1];
    print!("{:-16?} {:4} '", instruction, constant);
    println!("{}'", chunk.get_constants()[*constant as usize]);
    *offset += 2;
}