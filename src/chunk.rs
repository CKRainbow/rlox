use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::OpConstant,
            1 => OpCode::OpReturn,
            _ => panic!("Unkown opcode: {}", byte),
        }
    }
}

pub type Code = Vec<u8>;

pub struct Chunk {
    code: Code,
    lines: Vec<usize>, // that's memory consuming
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }
    
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_op_code(&mut self, op_code: OpCode, line: usize) {
        self.write(op_code as u8, line);
    }

    // return the index of the constant
    // easy to locate later
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn get_constants(&self) -> &ValueArray {
        &self.constants
    }

    pub fn get_lines(&self) -> &Vec<usize> {
        &self.lines
    }
}
