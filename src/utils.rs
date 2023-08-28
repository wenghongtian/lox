pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, whe: &str, message: &str) {
    println!("[line {line}] Error {whe}: {message}");
}


