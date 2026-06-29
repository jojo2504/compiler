use crate::token::Token;

pub mod prelude {
    pub(crate) use crate::ast;
    pub use ast::Boolean;
    pub use ast::Expression;
    pub use ast::IdentifierToken;
    pub use ast::IntegerLiteral;
    pub use ast::Node;
    pub use ast::Program;
    pub use ast::Statement;
}

pub trait Node: ToString {
    fn token_literal(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(let_statement) => let_statement.token.to_string(),
            Statement::Return(return_statement) => return_statement.token.to_string(),
            Statement::Expression(expression_statement) => expression_statement.token.to_string(),
        }
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(s) => s.to_string(),
            Statement::Return(s) => s.to_string(),
            Statement::Expression(s) => s.to_string(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_owned()
        }
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        let mut buffer = String::new();

        for statement in &self.statements {
            buffer.push_str(&statement.to_string());
        }

        buffer
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(IdentifierToken),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(Box<PrefixExpression>), // Box because `PrefixExpression` contains an `Expression` itself
    InfixExpression(Box<InfixExpression>),   // Same
    Boolean(Boolean),
    IfExpression(Box<IfExpression>), // Same
    FunctionLiteral(FunctionLiteral),
    CallExpression(Box<CallExpression>),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.0.to_string(),
            Expression::IntegerLiteral(ilit) => ilit.0.to_string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.to_string(),
            Expression::InfixExpression(infix_expression) => infix_expression.to_string(),
            Expression::Boolean(boolean) => boolean.to_string(),
            Expression::IfExpression(if_expression) => if_expression.to_string(),
            Expression::FunctionLiteral(function_literal) => function_literal.to_string(),
            Expression::CallExpression(call_expression) => call_expression.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierToken(pub String);

#[derive(Debug, Clone)]
pub struct IntegerLiteral(pub i64);

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.0.to_string()
    }
}

impl ToString for Boolean {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierToken,
    pub value: Expression,
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.token.to_string().to_lowercase());
        buffer.push(' ');
        buffer.push_str(&self.name.0);
        buffer.push_str(" = ");
        buffer.push_str(&self.value.to_string());
        buffer.push(';');

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.token.to_string().to_lowercase());
        buffer.push(' ');
        buffer.push_str(&self.return_value.clone().to_string());
        buffer.push(';');

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Expression,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push('(');
        buffer.push_str(&self.operator);
        buffer.push_str(&self.right.to_string());
        buffer.push(')');

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Expression,
    pub operator: String,
    pub right: Expression,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push('(');
        buffer.push_str(&self.left.to_string());
        buffer.push_str(&format!(" {} ", self.operator.to_string()));
        buffer.push_str(&self.right.to_string());
        buffer.push(')');

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl ToString for IfExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("if");
        buffer.push_str(&self.condition.to_string());
        buffer.push(' ');
        buffer.push_str(&self.consequence.to_string());

        if let Some(alternative) = &self.alternative {
            buffer.push_str("else ");
            buffer.push_str(&alternative.to_string());
        }

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl ToString for BlockStatement {
    fn to_string(&self) -> String {
        let mut buffer = String::new();

        for s in &self.statements {
            buffer.push_str(&s.to_string());
        }

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<IdentifierToken>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl ToString for FunctionLiteral {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        let params: Vec<String> = self.parameters.iter().map(|x| x.0.to_string()).collect();
        buffer.push_str(&self.token_literal());
        buffer.push('(');
        buffer.push_str(&params.join(", "));
        buffer.push(')');
        buffer.push_str(&self.body.to_string());

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Expression,
    pub arguments: Vec<Expression>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl ToString for CallExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        let args: Vec<String> = self.arguments.iter().map(|a| a.to_string()).collect();
        buffer.push_str(&self.function.to_string());
        buffer.push('(');
        buffer.push_str(&args.join(", "));
        buffer.push(')');

        buffer
    }
}
