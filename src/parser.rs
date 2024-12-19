use crate::{error::*, expr::*, object::*, stmt::*, token::*};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
}

static mut UUID: usize = 0;

pub fn next_uid() -> usize {
    unsafe {
        UUID += 1;
        UUID
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            if let Some(statement) = self.declaration() {
                statements.push(statement);
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let declaration = if self.matches(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.matches(&[TokenType::Fun]) {
            self.function_declaration("function")
        } else if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        // Has a parse error
        match declaration {
            Ok(statement) => Some(statement),
            Err(err) => {
                err.report();
                self.had_error = true;
                self.synchronize();
                None
            }
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, LoxErrorResult> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;
        self.consume(TokenType::LeftBrace, "Expect '{{' before class body.")?;

        let mut methods: Vec<Stmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function_declaration("method")?);
        }

        self.consume(TokenType::RightBrace, "Expect '}}' after class body.")?;

        Ok(Stmt::Class(ClassStmt { name, methods }))
    }

    fn function_declaration(&mut self, kind: &str) -> Result<Stmt, LoxErrorResult> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {kind} name."))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind} name."),
        )?;

        let mut params: Vec<Token> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    LoxErrorResult::parse_error(
                        self.peek(),
                        "Cannot have more than 255 parameters.",
                    );
                }

                params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' after before {kind} body."),
        )?;
        let body: Vec<Stmt> = self.block()?;

        Ok(Stmt::Function(FunctionStmt { name, params, body }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxErrorResult> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        if self.matches(&[TokenType::Break]) {
            return self.break_statement();
        }
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.matches(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }
        self.expression_statement()
    }

    fn break_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
        Ok(Stmt::Break(BreakStmt { keyword }))
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        // Initializer
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        // Condition
        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        // Increment
        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        // Get body
        let mut body = self.statement()?;

        // Check increment
        if let Some(value) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![
                    body,
                    Stmt::Expression(ExpressionStmt {
                        expression: Box::new(value),
                    }),
                ],
            })
        }

        // Check condition
        let while_condition = if let Some(result) = condition {
            result
        } else {
            Expr::Literal(LiteralExpr {
                uid: next_uid(),
                value: Object::Bool(true),
            })
        };
        body = Stmt::While(WhileStmt {
            condition: Box::new(while_condition),
            body: Box::new(body),
        });

        // Check initializer
        if let Some(init_statement) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![init_statement, body],
            })
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = Box::new(self.expression()?);
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = match self.matches(&[TokenType::Else]) {
            true => Some(Box::new(self.statement()?)),
            false => None,
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after 'print' value.")?;
        Ok(Stmt::Print(PrintStmt {
            expression: Box::new(value),
        }))
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        let keyword = self.previous();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(ReturnStmt { keyword, value }))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(WhileStmt {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxErrorResult> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(statement) = self.declaration() {
                statements.push(statement);
            }
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxErrorResult> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(ExpressionStmt {
            expression: Box::new(expr),
        }))
    }

    fn expression(&mut self) -> Result<Expr, LoxErrorResult> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxErrorResult> {
        let expr = self.ternary()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(variable) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: variable.name,
                    value: Box::new(value),
                    uid: next_uid(),
                }));
            }
            return Err(LoxErrorResult::parse_error(
                equals,
                "Invalid assignment target.",
            ));
        }
        Ok(expr)
    }

    // Add ternary support with '?' and ':'
    fn ternary(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.logic_or()?;

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
                uid: next_uid(),
            })
        }
        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.logic_and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.logic_and()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                uid: next_uid(),
            });
        }

        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                uid: next_uid(),
            });
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                uid: next_uid(),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxErrorResult> {
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
                uid: next_uid(),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                uid: next_uid(),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                uid: next_uid(),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxErrorResult> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right,
                uid: next_uid(),
            }));
        }
        self.call()
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> Result<Expr, LoxErrorResult> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    LoxErrorResult::parse_error(
                        self.peek(),
                        "Cannot have more than 255 arguments.",
                    );
                }
                arguments.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(CallExpr {
            callee,
            paren,
            arguments,
            uid: next_uid(),
        }))
    }

    fn call(&mut self) -> Result<Expr, LoxErrorResult> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(Box::new(expr))?;
            } else if self.matches(&[TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get(GetExpr {
                    uid: next_uid(),
                    object: Box::new(expr),
                    name,
                })
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, LoxErrorResult> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::Bool(false),
                uid: next_uid(),
            }));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::Bool(true),
                uid: next_uid(),
            }));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::Nil,
                uid: next_uid(),
            }));
        }
        if self.matches(&[TokenType::Number, TokenType::String]) {
            let value = self.previous();
            return Ok(Expr::Literal(LiteralExpr {
                value: value.literal,
                uid: next_uid(),
            }));
        }

        if self.matches(&[TokenType::Identifier]) {
            let name = self.previous();
            return Ok(Expr::Variable(VariableExpr {
                name,
                uid: next_uid(),
            }));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expression = Box::new(self.expression()?);
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression,
                uid: next_uid(),
            }));
        }
        Err(LoxErrorResult::parse_error(
            self.peek(),
            "Expect expression.",
        ))
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxErrorResult> {
        match self.check(&token_type) {
            true => Ok(self.advance()),
            false => Err(LoxErrorResult::parse_error(self.peek(), message)),
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
