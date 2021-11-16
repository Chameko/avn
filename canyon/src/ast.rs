use crate::token::{Token};

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum Statement {
    LetStatement(LetStatement)
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(l) => l.token_literal(),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.which()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.which()
    }
}