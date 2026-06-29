mod tests {
    use compiler::{
        lexer::Lexer,
        token::{Keyword, Token},
    };
    use test_case::test_case;

    #[test_case("=+(){},;", vec![
        (Token::ASSIGN, "="),
        (Token::PLUS, "+"),
        (Token::LPAREN, "("),
        (Token::RPAREN, ")"),
        (Token::LBRACE, "{"),
        (Token::RBRACE, "}"),
        (Token::COMMA, ","),
        (Token::SEMICOLON, ";"),
        (Token::EOF, ""),
    ]; "simple one caracter tokens")]
    #[test_case("100", vec![
        (Token::INT("100".to_owned()), "100"),
        (Token::EOF, ""),
    ]; "long number")]
    #[test_case("let a = 1;", vec![
        (Token::KEYWORD(Keyword::LET), "LET"),
        (Token::IDENTIFIER("a".to_owned()), "a"),
        (Token::ASSIGN, "="),
        (Token::INT("1".to_owned()), "1"),
        (Token::SEMICOLON, ";"),
        (Token::EOF, ""),
    ]; "keywords, identifiers, numbers")]
    #[test_case("if 1 < 2 { return 3; }", vec![
        (Token::IF, "IF"),
        (Token::INT("1".to_owned()), "1"),
        (Token::LT, "<"),
        (Token::INT("2".to_owned()), "2"),
        (Token::LBRACE, "{"),
        (Token::KEYWORD(Keyword::RETURN), "RETURN"),
        (Token::INT("3".to_owned()), "3"),
        (Token::SEMICOLON, ";"),
        (Token::RBRACE, "}"),
        (Token::EOF, ""),
    ]; "more keywords")]
    #[test_case("let a = 1; !-/*5; 5 < 10 > 5;", vec![
        (Token::KEYWORD(Keyword::LET), "LET"),
        (Token::IDENTIFIER("a".to_owned()), "a"),
        (Token::ASSIGN, "="),
        (Token::INT("1".to_owned()), "1"),
        (Token::SEMICOLON, ";"),
        (Token::BANG, "!"),
        (Token::MINUS, "-"),
        (Token::SLASH, "/"),
        (Token::ASTERISK, "*"),
        (Token::INT("5".to_owned()), "5"),
        (Token::SEMICOLON, ";"),
        (Token::INT("5".to_owned()), "5"),
        (Token::LT, "<"),
        (Token::INT("10".to_owned()), "10"),
        (Token::GT, ">"),
        (Token::INT("5".to_owned()), "5"),
        (Token::SEMICOLON, ";"),
        (Token::EOF, ""),
    ]; "more tokens")]
    #[test_case("{
        let a = 1;
        }", vec![
        (Token::LBRACE, "{"),
        (Token::KEYWORD(Keyword::LET), "LET"),
        (Token::IDENTIFIER("a".to_owned()), "a"),
        (Token::ASSIGN, "="),
        (Token::INT("1".to_owned()), "1"),
        (Token::SEMICOLON, ";"),
        (Token::RBRACE, "}"),
        (Token::EOF, ""),
    ]; "multiline input")]
    #[test_case("10 == 10; 9 != 9;", vec![
        (Token::INT("10".to_owned()), "10"),
        (Token::EQ, "=="),
        (Token::INT("10".to_owned()), "10"),
        (Token::SEMICOLON, ";"),
        (Token::INT("9".to_owned()), "9"),
        (Token::NOTEQ, "!="),
        (Token::INT("9".to_owned()), "9"),
        (Token::SEMICOLON, ";"),
        (Token::EOF, ""),
    ]; "2-char tokens")]
    fn test_next_token_cases(input: &str, expected: Vec<(Token, &str)>) {
        let mut l = Lexer::new(input);
        let mut tokens = vec![];

        for (i, tt) in expected.iter().enumerate() {
            let token = l.next_token();
            tokens.push(token.clone());

            if token != tt.0 {
                panic!(
                    "tests[{}] - tokentype wrong. expected={:?}, got={:?}, {:?}",
                    i, tt.0, token, tokens
                );
            }

            if token.to_string() != tt.1 {
                panic!(
                    "tests[{}] - literal wrong. expected={}, got={}, {:?}",
                    i,
                    tt.1,
                    token.to_string(),
                    tokens
                )
            }
        }
    }
}
