#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
}

impl LoxError {
    pub fn error(line: usize, message: &str) -> LoxError {
        LoxError {
            line,
            message: message.to_string(),
        }
    }
    pub fn report(&self, location: &str) {
        let error = format!(
            "[line {}] - Error: {} => at column {}",
            self.line, self.message, location
        );
        eprintln!("{}", error)
    }
}
