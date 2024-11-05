#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }
    pub fn report(&self, location: &str) {
        let error = format!(
            "[line {}] - Error {}: {}",
            self.line, location, self.message
        );
        eprintln!("{}", error)
    }
}
