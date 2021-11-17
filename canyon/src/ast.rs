use crate::token::{Token};

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
	ReturnStatement(ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(l) => l.token_literal(),
			Statement::ReturnStatement(r) => r.token_literal(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
	Null // Used for testing only
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token_literal(),
			Expression::Null => "Null".to_string()
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String{
        let mut output = String::from("");
        for statement in &self.statements {
            output.push_str(&statement.token_literal())
        }
        output
    }
}

//
// Various tokens
//

#[derive(Clone, Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.which()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.which()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
	pub token: Token,
	pub return_value: Expression,
}

impl Node for ReturnStatement {
	fn token_literal(&self) -> String {
		self.token.which()
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
	pub token: Token,
	pub expression: Expression,
}

impl Node for ExpressionStatement {
	fn token_literal(&self) -> String {
		self.token.which()
	}
}