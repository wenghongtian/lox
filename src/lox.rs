use crate::{error::LoxError, interpreter::Interpreter, parser::Parser, scanner::Scanner};

pub struct Lox {
    pub had_error: bool,
    scanner: Scanner,
    interpreter: Interpreter,
    parser: Parser,
}

impl Lox {
    pub fn new() -> Lox {
        Self {
            had_error: false,
            scanner: Scanner::new(),
            interpreter: Interpreter::new(),
            parser: Parser::new(),
        }
    }

    pub fn error(line: usize, message: String) -> LoxError {
        LoxError::new_compile(message, line)
    }

    pub fn report(err: LoxError, loc: String) {
        match err {
            LoxError::Compile(err) => {
                eprintln!("[line {}] Error {loc}: {}", err.line, err.msg)
            }
            LoxError::Runtime(err) => {
                eprintln!("[runtime error] Error {loc}: {}", err.msg)
            }
        }
    }

    pub fn run(&mut self, source: String) -> Result<(), LoxError> {
        let tokens = self.scanner.scan_tokens(source)?.clone();
        let stmts = self.parser.parse(tokens)?;
        self.interpreter.interpret(&stmts)?;
        Ok(())
    }
}
