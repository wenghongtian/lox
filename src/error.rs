#[derive(Debug)]
pub enum LoxError {
    Compile(CompileError),
    Runtime(RuntimeError),
}

impl LoxError {
    pub fn new_compile(msg: String, line: usize) -> LoxError {
        LoxError::Compile(CompileError { msg, line })
    }
    pub fn new_runtime(msg: String) -> LoxError {
        LoxError::Runtime(RuntimeError { msg: msg })
    }
}

#[derive(Debug)]
pub struct CompileError {
    pub msg: String,
    pub line: usize,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
}
