#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    LT,
    GT,
    SLASH,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    EOF,
    ILLEGAL(String),
    INT(String),
    KEYWORD(Keyword),
    IDENTIFIER(String),
    EQ,
    NOTEQ,
    TRUE,
    FALSE,
    IF,
    ELSE,
    FUNCTION,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    LET,
    RETURN,
}

impl Keyword {
    pub fn read_keyword(word: &str) -> Token {
        match word {
            "if" => Token::IF,
            "else" => Token::ELSE,
            "fn" => Token::FUNCTION,
            "let" => Token::KEYWORD(Keyword::LET),
            "return" => Token::KEYWORD(Keyword::RETURN),
            _ => Token::IDENTIFIER(word.to_owned()),
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let keyword_str: String;
        String::from(match self {
            Token::ILLEGAL(value) => value,
            Token::EOF => "",
            Token::ASSIGN => "=",
            Token::PLUS => "+",
            Token::MINUS => "-",
            Token::BANG => "!",
            Token::ASTERISK => "*",
            Token::LT => "<",
            Token::GT => ">",
            Token::SLASH => "/",
            Token::COMMA => ",",
            Token::SEMICOLON => ";",
            Token::LPAREN => "(",
            Token::RPAREN => ")",
            Token::LBRACE => "{",
            Token::RBRACE => "}",
            Token::INT(value) => value,
            Token::IDENTIFIER(name) => name,
            Token::KEYWORD(keyword) => {
                keyword_str = keyword.to_string();
                &keyword_str
            }
            Token::EQ => "==",
            Token::NOTEQ => "!=",
            Token::TRUE => "TRUE",
            Token::FALSE => "FALSE",
            Token::IF => "IF",
            Token::ELSE => "ELSE",
            Token::FUNCTION => "FUNCTION",
        })
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        String::from(match self {
            Keyword::LET => "LET",
            Keyword::RETURN => "RETURN",
        })
    }
}
