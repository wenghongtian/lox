use crate::{error::LoxError, scanner::Scanner};

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Self { had_error: false }
    }

    pub fn error(line: usize, message: String) -> LoxError {
        LoxError::new(line, message)
    }

    pub fn report(err: LoxError, loc: String) {
        eprintln!("[line {}] Error {loc}: {}", err.line, err.msg);
    }

    pub fn run(&mut self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:#?}", token);
        }

        Ok(())
    }
}
