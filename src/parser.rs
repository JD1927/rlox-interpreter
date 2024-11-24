use crate::{
    error::LoxError,
    expr::{
        BinaryExpr, CommaExpr, Expr, GroupingExpr, LiteralExpr, TernaryExpr, UnaryExpr,
        VariableExpr,
    },
    object::*,
    stmt::{PrintStmt, Stmt, VarStmt},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let declaration = if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        // Has a parse error
        if declaration.is_err() {
            self.synchronize();
        }
        declaration
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.matches(&[TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(LiteralExpr { value: Object::Nil })
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(VarStmt {
            name,
            initializer: Box::new(initializer),
        }))
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt {
            expression: Box::new(value),
        }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt {
            expression: Box::new(expr),
        }))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.comma()
    }

    // Add comma operator
    fn comma(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.ternary()?;

        // Allow multiple comma-separated expressions!
        while self.matches(&[TokenType::Comma]) {
            let right = self.ternary()?;
            expr = Expr::Comma(CommaExpr {
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    // Add ternary support with '?' and ':'
    fn ternary(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        // Check for "?" to begin a ternary expression
        while self.matches(&[TokenType::Question]) {
            let then_branch = self.expression()?; // "Then" expression
            self.consume(
                TokenType::Colon,
                "Expect ':' after then branch of ternary operator.",
            )?;
            let else_branch = self.ternary()?; // "Else expression with right-associativity"

            expr = Expr::Ternary(TernaryExpr {
                condition: Box::new(expr),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            })
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::Bool(false),
            }));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::Bool(true),
            }));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr { value: Object::Nil }));
        }
        if self.matches(&[TokenType::Number, TokenType::String]) {
            let value = self.previous();
            return Ok(Expr::Literal(LiteralExpr {
                value: value.literal,
            }));
        }

        if self.matches(&[TokenType::Identifier]) {
            let name = self.previous();
            return Ok(Expr::Variable(VariableExpr { name }));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        Err(LoxError::parse_error(self.peek(), "Expect expression."))
    }
    // HELPERS
    fn matches(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxError> {
        match self.check(&token_type) {
            true => Ok(self.advance()),
            false => Err(LoxError::parse_error(self.peek(), message)),
        }
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        match self.is_at_end() {
            true => false,
            false => self.peek().is(token_type.to_owned()),
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is(TokenType::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}
