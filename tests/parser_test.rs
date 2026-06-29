mod tests {
    use compiler::{
        ast::{ExpressionStatement, FunctionLiteral, IfExpression, InfixExpression, prelude::*},
        lexer::Lexer,
        parser::Parser,
    };
    use test_case::test_case;

    fn init_program_parsing(input: &str) -> Program {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        Parser::parse_program(&mut parser)
    }

    #[test_case("let x = 5;
        let y = 5;", vec!["x", "y"])]
    fn test_let_statements(input: &str, expected: Vec<&str>) {
        let program = init_program_parsing(input);

        if program.statements.len() != expected.len() {
            panic!(
                "program statements does not contain {} statements. got={}",
                expected.len(),
                program.statements.len()
            )
        }

        for (i, tt) in expected.iter().enumerate() {
            let statement = &program.statements[i];
            if !test_let_statement(statement, tt) {
                return;
            }
        }

        // println!("{:#?}", program.statements);
    }

    #[test]
    #[should_panic]
    fn test_let_statement_invalid() {
        let input = "let x 5;";
        init_program_parsing(input);
    }

    fn test_let_statement(statement: &Statement, name: &str) -> bool {
        if statement.token_literal() != "LET" {
            panic!(
                "s.TokenLiteral not 'let'. got={}",
                statement.token_literal()
            );
        }

        let let_statement = match statement {
            Statement::Let(let_statement) => let_statement,
            _ => {
                panic!("statement is not a LetStatement. got={:?}", statement);
            }
        };

        if let_statement.name.0 != name {
            panic!("let_statement not {}. got={}", name, let_statement.name.0);
        }

        if let_statement.name.0 != name {
            panic!("statement name not {}. got={:?}", name, let_statement.name);
        }

        true
    }

    #[test_case("return 1;")]
    fn test_return_statements(input: &str) {
        init_program_parsing(input);
    }

    #[test_case("foobar;")]
    fn test_identifier_expression(input: &str) {
        let program = init_program_parsing(input);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);
        test_identifier(&expression_statement.expression, "foobar");
        assert_eq!(expression_statement.token.to_string(), "foobar");

        // println!("{:#?}", program.statements);
    }

    #[test_case("5;")]
    pub fn test_integer_expression(input: &str) {
        let program = init_program_parsing(input);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);
        test_integer_literal(&expression_statement.expression, 5);
        assert_eq!(expression_statement.token.to_string(), "5");

        // println!("{:#?}", program.statements);
    }

    #[test_case("!5;", "!", 5)]
    #[test_case("-15;", "-", 15)]
    pub fn test_parsing_prefix(input: &str, operator: &str, integer_value: i64) {
        let program = init_program_parsing(input);

        assert_eq!(program.statements.len(), 1);

        // println!("{:#?}", program.statements);
    }

    #[test_case("5 + 5;", 5, "+", 5 ; "add")]
    #[test_case("5 - 5;", 5, "-", 5 ; "subtract")]
    #[test_case("5 * 5;", 5, "*", 5 ; "multiply")]
    #[test_case("5 / 5;", 5, "/", 5 ; "divide")]
    #[test_case("5 > 5;", 5, ">", 5 ; "greater than")]
    #[test_case("5 < 5;", 5, "<", 5 ; "less than")]
    #[test_case("5 == 5;", 5, "==", 5 ; "equal")]
    #[test_case("5 != 5;", 5, "!=", 5 ; "not equal")]
    pub fn test_parsing_infix_expression(
        input: &str,
        left_value: i64,
        operator: &str,
        right_value: i64,
    ) {
        let program = init_program_parsing(input);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);

        test_infix_expression(
            &expression_statement.expression,
            left_value.into(),
            operator,
            right_value.into(),
        );

        // println!("{:#?}", program.statements);
    }

    #[test_case("-a * b",                             "((-a) * b)"                          ; "neg a times b")]
    #[test_case("!-a",                                "(!(-a))"                             ; "bang neg a")]
    #[test_case("a + b + c",                          "((a + b) + c)"                       ; "a plus b plus c")]
    #[test_case("a + b - c",                          "((a + b) - c)"                       ; "a plus b minus c")]
    #[test_case("a * b * c",                          "((a * b) * c)"                       ; "a times b times c")]
    #[test_case("a * b / c",                          "((a * b) / c)"                       ; "a times b div c")]
    #[test_case("a + b / c",                          "(a + (b / c))"                       ; "a plus b div c")]
    #[test_case("a + b * c + d / e - f",              "(((a + (b * c)) + (d / e)) - f)"     ; "complex")]
    #[test_case("3 + 4; -5 * 5",                      "(3 + 4)((-5) * 5)"                   ; "two statements")]
    #[test_case("5 > 4 == 3 < 4",                     "((5 > 4) == (3 < 4))"                ; "gt eq lt")]
    #[test_case("5 < 4 != 3 > 4",                     "((5 < 4) != (3 > 4))"                ; "lt neq gt")]
    #[test_case("3 + 4 * 5 == 3 * 1 + 4 * 5",        "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))" ; "complex eq")]
    #[test_case("1 + (2 + 3) + 4",                    "((1 + (2 + 3)) + 4)"                     ; "grouped add")]
    #[test_case("(5 + 5) * 2",                        "((5 + 5) * 2)"                           ; "grouped mul")]
    #[test_case("2 / (5 + 5)",                        "(2 / (5 + 5))"                           ; "grouped div")]
    #[test_case("-(5 + 5)",                           "(-(5 + 5))"                              ; "grouped neg")]
    #[test_case("!(true == true)", "(!(true == true))" ; "grouped bool")]
    #[test_case("a + add(b * c) + d",                                    "((a + add((b * c))) + d)"                        ; "call in add")]
    #[test_case("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",           "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"; "call multi args")]
    #[test_case("add(a + b + c * d / f + g)",                           "add((((a + b) + ((c * d) / f)) + g))"           ; "call complex expr")]
    fn test_operator_precedence_parsing(input: &str, expected: &str) {
        let program = init_program_parsing(input);

        assert_eq!(program.to_string(), expected, "input: {}", input);

        // println!("{:#?}", program.statements);
    }

    #[test_case("if (x < y) { x }")]
    #[test_case("if (x < y) { x } else { y }")]
    fn test_if(input: &str) {
        let program = init_program_parsing(input);
        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);
        let if_expression = test_if_expression(&expression_statement.expression);

        test_infix_expression(&if_expression.condition, "x".into(), "<", "y".into());
        assert_eq!(if_expression.consequence.statements.len(), 1);

        let statement = &if_expression.consequence.statements[0];
        let statement_expression = test_statement_expresssion(statement);

        test_identifier(&statement_expression.expression, "x");

        // if let Some(alternative) = &if_expression.alternative {
        //     println!(
        //         "if_expression.alternative.statements was not None. got={:?}",
        //         alternative
        //     )
        // }

        // println!("{:#?}", program.statements);
    }

    #[test_case("fn(x, y) { x + y; }")]
    fn test_function_literal_parsing(input: &str) {
        let program = init_program_parsing(input);
        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);

        let Expression::FunctionLiteral(function_literal) = &expression_statement.expression else {
            panic!(
                "expression is not Expression::IntegerLiteral. got={:?}",
                expression_statement.expression
            );
        };

        assert_eq!(function_literal.parameters.len(), 2);

        assert_eq!(function_literal.parameters[0].0.to_string(), "x");
        assert_eq!(function_literal.parameters[1].0.to_string(), "y");

        let statement = &function_literal.body.statements[0];
        let body_statement = test_statement_expresssion(statement);
        test_infix_expression(&body_statement.expression, "x".into(), "+", "y".into());
    }

    #[test_case("fn() {};", vec![])]
    #[test_case("fn(x) {};", vec!["x"])]
    #[test_case("fn(x, y, z) {};", vec!["x", "y", "z"])]
    fn test_function_paramater_parsing(input: &str, expected: Vec<&str>) {
        let program = init_program_parsing(input);

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);
        let function = test_function_literal(&expression_statement.expression);

        assert_eq!(function.parameters.len(), expected.len());

        for (i, ident) in expected.iter().enumerate() {
            assert_eq!(function.parameters[i].0.to_string(), *ident);
        }
    }

    #[test_case("add(1, 2 * 3, 4 + 5);")]
    fn test_call_expression(input: &str) {
        let program = init_program_parsing(input);
        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        let expression_statement = test_statement_expresssion(statement);

        let Expression::CallExpression(call_expression) = &expression_statement.expression else {
            panic!(
                "expression is not Expression::CallExpression. got={:?}",
                expression_statement.expression
            );
        };

        test_identifier(&call_expression.function, "add");
        assert_eq!(call_expression.arguments.len(), 3);

        test_literal_expression(&call_expression.arguments[0], 1.into());
        test_infix_expression(&call_expression.arguments[1], 2.into(), "*", 3.into());
        test_infix_expression(&call_expression.arguments[2], 4.into(), "+", 5.into());
    }

    fn test_integer_literal(expression: &Expression, value: i64) -> &IntegerLiteral {
        let Expression::IntegerLiteral(integer_literal) = expression else {
            panic!(
                "expression is not Expression::IntegerLiteral. got={:?}",
                expression
            );
        };
        assert_eq!(integer_literal.0, value);

        integer_literal
    }

    fn test_boolean_literal(expression: &Expression, value: bool) -> &Boolean {
        let Expression::Boolean(boolean) = expression else {
            panic!(
                "expression is not Expression::Boolean. got={:?}",
                expression
            );
        };
        assert_eq!(boolean.0, value);

        boolean
    }

    fn test_if_expression(expression: &Expression) -> &IfExpression {
        let Expression::IfExpression(if_expression) = expression else {
            panic!(
                "expression is not Expression::IfExpression. got={:?}",
                expression
            );
        };

        if_expression
    }

    fn test_identifier<'a>(expression: &'a Expression, value: &str) -> &'a IdentifierToken {
        let Expression::Identifier(identifier) = expression else {
            panic!(
                "expression is not Expression::Identifier. got={:?}",
                expression
            );
        };
        assert_eq!(identifier.0, value);

        identifier
    }

    fn test_function_literal(expression: &Expression) -> &FunctionLiteral {
        let Expression::FunctionLiteral(function_literal) = expression else {
            panic!(
                "expression is not Expression::FunctionLiteral. got={:?}",
                expression
            );
        };

        function_literal
    }

    fn test_infix_expression<'a>(
        expression: &'a Expression,
        left: LiteralExpression,
        operator: &str,
        right: LiteralExpression,
    ) -> &'a InfixExpression {
        let Expression::InfixExpression(infix_expression) = expression else {
            panic!(
                "expression not Expression::InfixExpression. got={:?}",
                expression
            );
        };
        test_literal_expression(&infix_expression.left, left);
        assert_eq!(
            infix_expression.operator, operator,
            "infix_expression.operator is not {}. got={}",
            operator, infix_expression.operator
        );
        test_literal_expression(&infix_expression.right, right);

        infix_expression
    }

    fn test_statement_expresssion(statement: &Statement) -> &ExpressionStatement {
        let Statement::Expression(expression_statement) = statement else {
            panic!(
                "statement is not Statement::Expression. got={:?}",
                statement
            )
        };

        expression_statement
    }

    enum LiteralExpression {
        Integer(i64),
        Str(&'static str),
        Bool(bool),
    }

    impl From<i64> for LiteralExpression {
        fn from(v: i64) -> Self {
            LiteralExpression::Integer(v)
        }
    }
    impl From<&'static str> for LiteralExpression {
        fn from(v: &'static str) -> Self {
            LiteralExpression::Str(v)
        }
    }
    impl From<bool> for LiteralExpression {
        fn from(v: bool) -> Self {
            LiteralExpression::Bool(v)
        }
    }

    fn test_literal_expression(expression: &Expression, expected: LiteralExpression) {
        match expected {
            LiteralExpression::Integer(i) => _ = test_integer_literal(expression, i),
            LiteralExpression::Str(s) => _ = test_identifier(expression, s),
            LiteralExpression::Bool(b) => _ = test_boolean_literal(expression, b),
        }
    }
}
