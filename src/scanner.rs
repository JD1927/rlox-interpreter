use crate::{error::LoxError, object::Object, token::*};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => e.report_column(&self.column.to_string()),
            }
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            Object::Nil,
            self.line,
        ));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let _char = self.advance();

        match _char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '?' => self.add_token(TokenType::Question),
            '!' => {
                if self.match_next_with('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next_with('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_next_with('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next_with('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_next_with('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next_with('*') {
                    // Block comment start
                    self.scan_block_comment()?
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            '"' => match self.add_string() {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            _ => {
                if _char.is_ascii_digit() {
                    self.add_number();
                } else if _char.is_ascii_alphabetic() || _char == '_' {
                    self.add_identifier();
                } else {
                    return Err(LoxError::lexical_error(self.line, "Unexpected character."));
                }
            }
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_next_with(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn scan_block_comment(&mut self) -> Result<(), LoxError> {
        while !self.is_at_end() {
            if self.match_next_with('*') && self.match_next_with('/') {
                // End of current block comment */
                return Ok(());
            } else if self.match_next_with('/') && self.match_next_with('*') {
                // Found nested block comment */,
                self.scan_block_comment()?;
            } else if self.advance() == '\n' {
                // Advances with the next char
                self.line += 1;
                self.column = 0;
            }
        }
        // Unclosed block comment error
        Err(LoxError::lexical_error(
            self.line,
            "Unterminated block comment.",
        ))
    }

    fn peek(&self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.source[self.current],
        }
    }

    fn peek_next(&self) -> char {
        match self.current + 1 >= self.source.len() {
            true => '\0',
            false => self.source[self.current + 1],
        }
    }

    fn advance(&mut self) -> char {
        let _char = if self.current >= self.source.len() {
            self.source[self.current - 1]
        } else {
            self.source[self.current]
        };
        self.current += 1;
        self.column += 1;
        _char
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Object) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, lexeme, Object::Nil, self.line));
    }

    fn add_string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::lexical_error(self.line, "Unterminated string."));
        }
        // The closing quote "
        self.advance();
        // Trim the surrounding quotes.
        // TODO: Handle escape sequence
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, Object::String(value));
        Ok(())
    }

    fn add_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value: f64 = String::from_iter(&self.source[self.start..self.current])
            .parse::<f64>()
            .unwrap();
        self.add_token_literal(TokenType::Number, Object::Number(value));
    }

    fn add_identifier(&mut self) {
        while self.is_alphanumeric() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let token_type = self.get_keyword(&value);

        match token_type {
            Some(t_type) => match t_type {
                TokenType::True => self.add_token_literal(t_type, Object::Bool(true)),
                TokenType::False => self.add_token_literal(t_type, Object::Bool(false)),
                _ => self.add_token(t_type),
            },
            None => self.add_token_literal(TokenType::Identifier, Object::String(value)),
        }
    }

    fn is_alphanumeric(&self) -> bool {
        self.peek().is_ascii_alphabetic() || self.peek().is_ascii_digit() || self.peek() == '_'
    }

    fn get_keyword(&self, word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
