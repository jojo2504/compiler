mod tests {
    use compiler::{
        ast::{Expression, IdentifierToken, LetStatement, Node, Program, Statement},
        token::{Keyword, Token},
    };
    use test_case::test_case;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                token: Token::KEYWORD(Keyword::LET),
                name: IdentifierToken("myVar".to_owned()),
                value: Expression::Identifier(IdentifierToken("anotherVar".to_owned())),
            })],
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
