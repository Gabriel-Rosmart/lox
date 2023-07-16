use crate::bytecode::{Chunk, OpCode};

pub enum InterpretResult {
    InterpretOk,
    RuntimeError,
    CompileError,
}

pub struct VM {
    chunck: Option<Chunk>,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunck: None,
            ip: 0,
        }
    }

    pub fn interpret(&mut self, chunck: Chunk) {}
}
