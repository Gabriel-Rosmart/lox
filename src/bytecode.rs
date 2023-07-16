#[derive(Clone)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    /// For now we'll only allow to store f64
    pub constant_pool: Vec<f64>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constant_pool: Vec::new(),
        }
    }
}
