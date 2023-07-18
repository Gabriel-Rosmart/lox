#[allow(unused)]
use crate::bytecode::{Chunk, OpCode};

#[allow(unused)]
pub enum InterpretResult {
    InterpretOk,
    RuntimeError,
    CompileError,
}

#[allow(unused)]
pub struct VM {
    chunck: Option<Chunk>,
    ip: usize,
}

impl VM {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            chunck: None,
            ip: 0,
        }
    }

    #[allow(unused)]
    pub fn interpret(&mut self, chunck: Chunk) {}
}
