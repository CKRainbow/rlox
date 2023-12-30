mod chunk;
mod debug;
mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_op_code(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);
    chunk.write_op_code(OpCode::OpReturn, 123);
    chunk.disassemble("test chunk");
}
