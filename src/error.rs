#[derive(Debug)]
pub struct LoxError {
    pub msg: String,
    pub line: usize,
}

impl LoxError {
    pub fn new(line: usize, msg: String) -> Self {
        Self { msg, line }
    }
}
