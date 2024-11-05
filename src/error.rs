#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }
    pub fn report(&self, location: &str) {
        let error = format!(
            "[line {}] - Error: {} => at column {}",
            self.line, self.message, location
        );
        eprintln!("{}", error)
    }
}
