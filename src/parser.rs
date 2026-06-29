use std::{collections::HashMap, default};

use crate::{
    ast::{
        BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
        IdentifierToken, IfExpression, InfixExpression, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Keyword, Token},
};

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<std::mem::Discriminant<Token>, PrefixParseFn>,
    infix_parse_fns: HashMap<std::mem::Discriminant<Token>, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF,
            prefix_parse_fns: HashMap::default(),
            infix_parse_fns: HashMap::default(),
        };

        parser.register_prefix(
            Token::IDENTIFIER(Default::default()),
            Parser::parse_identifier,
        );
        parser.register_prefix(
            Token::INT(Default::default()),
            Parser::parse_integer_literal,
        );
        parser.register_prefix(Token::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(Token::MINUS, Parser::parse_prefix_expression);
        parser.register_prefix(Token::LPAREN, Parser::parse_grouped_expression);
        parser.register_prefix(Token::IF, Parser::parse_if_expression);
        parser.register_prefix(Token::TRUE, Parser::parse_boolean);
        parser.register_prefix(Token::FALSE, Parser::parse_boolean);
        parser.register_prefix(Token::FUNCTION, Parser::parse_function_literal);

        parser.register_infix(Token::PLUS, Parser::parse_infix_expression);
        parser.register_infix(Token::MINUS, Parser::parse_infix_expression);
        parser.register_infix(Token::SLASH, Parser::parse_infix_expression);
        parser.register_infix(Token::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(Token::EQ, Parser::parse_infix_expression);
        parser.register_infix(Token::NOTEQ, Parser::parse_infix_expression);
        parser.register_infix(Token::LT, Parser::parse_infix_expression);
        parser.register_infix(Token::GT, Parser::parse_infix_expression);
        parser.register_infix(Token::LPAREN, Parser::parse_call_expression);

        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn register_prefix(&mut self, token: Token, f: PrefixParseFn) {
        self.prefix_parse_fns
            .insert(std::mem::discriminant(&token), f);
    }

    pub fn register_infix(&mut self, token: Token, f: InfixParseFn) {
        self.infix_parse_fns
            .insert(std::mem::discriminant(&token), f);
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(parser: &mut Parser) -> Program {
        let mut statements = vec![];

        while parser.current_token != Token::EOF {
            statements.push(Parser::parse_statement(parser));
            parser.next_token();
        }

        Program { statements }
    }

    pub fn parse_statement(parser: &mut Parser) -> Statement {
        match parser.current_token {
            Token::KEYWORD(Keyword::LET) => {
                Statement::Let(Parser::parse_let_statement(parser).unwrap())
            }
            Token::KEYWORD(Keyword::RETURN) => {
                Statement::Return(Parser::parse_return_statement(parser).unwrap())
            }
            _ => Statement::Expression(Parser::parse_expression_statement(parser)),
        }
    }

    pub fn parse_identifier(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Identifier(IdentifierToken(
            parser.current_token.to_string(),
        )))
    }

    pub fn parse_let_statement(parser: &mut Parser) -> Option<LetStatement> {
        let token = parser.current_token.clone();

        if !parser
            .expect_peek(&Token::IDENTIFIER("".to_owned()))
            .unwrap()
        {
            return None;
        }

        let name = IdentifierToken(parser.current_token.to_string());

        if !parser.expect_peek(&Token::ASSIGN).unwrap() {
            return None;
        }

        parser.next_token();
        let value = Parser::parse_expression(parser, Precedence::Lowest)?;

        if parser.peek_token_is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Some(LetStatement {
            token,
            name: name.clone(),
            value,
        })
    }

    pub fn parse_return_statement(parser: &mut Parser) -> Option<ReturnStatement> {
        let token = parser.current_token.clone();
        parser.next_token();

        let return_value = Parser::parse_expression(parser, Precedence::Lowest)?;

        if parser.peek_token_is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Some(ReturnStatement {
            token,
            return_value,
        })
    }

    pub fn parse_expression_statement(parser: &mut Parser) -> ExpressionStatement {
        let token = parser.current_token.clone();
        let expression = Parser::parse_expression(parser, Precedence::Lowest)
            .expect("Expected expression. Found nothing.");

        if parser.peek_token_is(&Token::SEMICOLON) {
            parser.next_token();
        }

        return ExpressionStatement { token, expression };
    }

    pub fn parse_block_statement(parser: &mut Parser) -> BlockStatement {
        let token = parser.current_token.clone();
        let mut statements = vec![];

        parser.next_token();

        while !parser.current_token_is(&Token::RBRACE) && !parser.current_token_is(&Token::EOF) {
            let statement = Parser::parse_statement(parser);
            statements.push(statement);
            parser.next_token();
        }

        BlockStatement { token, statements }
    }

    pub fn parse_expression(parser: &mut Parser, precedence: Precedence) -> Option<Expression> {
        let key = std::mem::discriminant(&parser.current_token);
        let prefix = parser.prefix_parse_fns.get(&key).expect(&format!(
            "no prefix parse function for {:?} found",
            &parser.current_token.to_string(),
        ));

        let mut left_expression = prefix(parser)?;

        while !parser.peek_token_is(&Token::SEMICOLON) && precedence < parser.peek_precedence() {
            let key = std::mem::discriminant(&parser.peek_token);
            let infix = parser.infix_parse_fns.get(&key).copied();
            if let Some(infix) = infix {
                parser.next_token();
                left_expression = infix(parser, left_expression)?;
            } else {
                return Some(left_expression);
            }
        }

        Some(left_expression)
    }

    pub fn parse_prefix_expression(parser: &mut Parser) -> Option<Expression> {
        let token = parser.current_token.clone();
        let operator = parser.current_token.to_string();
        parser.next_token();

        Some(Expression::PrefixExpression(Box::new(PrefixExpression {
            token,
            operator,
            right: Parser::parse_expression(parser, Precedence::Prefix)?,
        })))
    }

    pub fn parse_infix_expression(parser: &mut Parser, left: Expression) -> Option<Expression> {
        let token = parser.current_token.clone();
        let operator = parser.current_token.to_string();
        let precedence = parser.current_precedence();
        parser.next_token();

        Some(Expression::InfixExpression(Box::new(InfixExpression {
            token,
            left,
            operator,
            right: Parser::parse_expression(parser, precedence)?,
        })))
    }

    pub fn parse_grouped_expression(parser: &mut Parser) -> Option<Expression> {
        parser.next_token();
        let expression = Parser::parse_expression(parser, Precedence::Lowest);
        if !parser.expect_peek(&Token::RPAREN).unwrap() {
            return None;
        }

        expression
    }

    pub fn parse_integer_literal(parser: &mut Parser) -> Option<Expression> {
        if let Ok(value) = parser.current_token.to_string().parse::<i64>() {
            return Some(Expression::IntegerLiteral(IntegerLiteral(value)));
        }
        None
    }

    pub fn parse_boolean(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Boolean(Boolean(
            parser.current_token_is(&Token::TRUE),
        )))
    }

    pub fn parse_if_expression(parser: &mut Parser) -> Option<Expression> {
        let token = parser.current_token.clone();

        if !parser.expect_peek(&Token::LPAREN).unwrap() {
            return None;
        }

        parser.next_token();
        let condition = Parser::parse_expression(parser, Precedence::Lowest)?;

        if !parser.expect_peek(&Token::RPAREN).unwrap()
            || !parser.expect_peek(&Token::LBRACE).unwrap()
        {
            return None;
        }

        let consequence = Parser::parse_block_statement(parser);
        let mut alternative = None;
        if parser.peek_token_is(&Token::ELSE) {
            parser.next_token();
            if !parser.expect_peek(&Token::LBRACE).unwrap() {
                return None;
            }
            alternative = Some(Parser::parse_block_statement(parser));
        };

        Some(Expression::IfExpression(Box::new(IfExpression {
            token,
            condition,
            consequence,
            alternative,
        })))
    }

    pub fn parse_function_literal(parser: &mut Parser) -> Option<Expression> {
        let token = parser.current_token.clone();

        if !parser.expect_peek(&Token::LPAREN).unwrap() {
            return None;
        }

        let parameters = Parser::parse_function_parameters(parser)?;

        if !parser.expect_peek(&Token::LBRACE).unwrap() {
            return None;
        }

        let body = Parser::parse_block_statement(parser);

        return Some(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body,
        }));
    }

    pub fn parse_function_parameters(parser: &mut Parser) -> Option<Vec<IdentifierToken>> {
        let mut identifiers = vec![];

        if parser.peek_token_is(&Token::RPAREN) {
            parser.next_token();
            return Some(identifiers);
        }

        parser.next_token();

        let identifier = IdentifierToken(parser.current_token.to_string());
        identifiers.push(identifier);
        while parser.peek_token_is(&Token::COMMA) {
            parser.next_token();
            parser.next_token();
            let identifier = IdentifierToken(parser.current_token.to_string());
            identifiers.push(identifier);
        }

        if !parser.expect_peek(&Token::RPAREN).unwrap() {
            return None;
        }

        return Some(identifiers);
    }

    pub fn parse_call_expression(parser: &mut Parser, function: Expression) -> Option<Expression> {
        Some(Expression::CallExpression(Box::new(CallExpression {
            token: parser.current_token.clone(),
            function,
            arguments: Parser::parse_call_arguments(parser)?,
        })))
    }

    fn parse_call_arguments(parser: &mut Parser) -> Option<Vec<Expression>> {
        let mut args = vec![];

        if parser.peek_token_is(&Token::RPAREN) {
            parser.next_token();
            return Some(args);
        }

        parser.next_token();
        args.push(Parser::parse_expression(parser, Precedence::Lowest)?);
        while parser.peek_token_is(&Token::COMMA) {
            parser.next_token();
            parser.next_token();
            args.push(Parser::parse_expression(parser, Precedence::Lowest)?);
        }

        if !parser.expect_peek(&Token::RPAREN).unwrap() {
            return None;
        }

        Some(args)
    }

    pub fn current_token_is(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.current_token) == std::mem::discriminant(token)
    }

    pub fn peek_token_is(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(token)
    }

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::try_from(self.peek_token.clone()).unwrap_or(Precedence::Lowest)
    }
    pub fn current_precedence(&self) -> Precedence {
        Precedence::try_from(self.current_token.clone()).unwrap_or(Precedence::Lowest)
    }

    pub fn expect_peek(&mut self, token: &Token) -> Result<bool, String> {
        if self.peek_token_is(token) {
            self.next_token();
            return Ok(true);
        }
        Err(format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        ))
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // my_function
}

impl TryFrom<Token> for Precedence {
    type Error = ();
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::EQ => Ok(Precedence::Equals),
            Token::NOTEQ => Ok(Precedence::Equals),
            Token::LT => Ok(Precedence::LessGreater),
            Token::GT => Ok(Precedence::LessGreater),
            Token::PLUS => Ok(Precedence::Sum),
            Token::MINUS => Ok(Precedence::Sum),
            Token::SLASH => Ok(Precedence::Product),
            Token::ASTERISK => Ok(Precedence::Product),
            Token::LPAREN => Ok(Precedence::Call),
            _ => Err(()),
        }
    }
}
