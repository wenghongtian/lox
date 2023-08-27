use lox::lox::Lox;
use std::{
    env::args,
    fs::File,
    io::{self, BufReader, Read, Write},
    process,
};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 3 {
        println!("Usage: lox-ast [script]");
    } else if args.len() == 2 {
        run_file(&args[0]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf);

    let source = String::from_utf8_lossy(&buf).to_string();
    let mut l = Lox::new();

    l.run(source);
    if l.had_error {
        process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush();
        let mut l = Lox::new();
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
        if buf.starts_with("exit\n") {
            break;
        }
        if buf.is_empty() {
            break;
        } else {
            l.run(buf);
            l.had_error = false;
        }
    }
}
