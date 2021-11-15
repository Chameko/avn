pub const SYMBOLS: &[&str] = &[
	"+", "-", "-", "/", // Mathmatical operators
	"==", "!=", "<=", ">=", "<", ">", "=<", "=>", "!", "||", // Logical operators
	"(", ")", // Normal braces
	"[", "]", // Square braces
	"{", "}", // Curly braces
	",", ";", ".", "@", "=", "//" // Other symbols
];

pub const KEYWORDS: &[&str] = &[
	"let",
	"return",
	"fn",
	"true",
	"false",
	"struct",
	"for",
	"while",
	"if"
];

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
	Literal(Literal),
	Symbol(String),
	Keyword(String),
	Identifier(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
	Int(String),
	String(String)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
	pub filename: String,
	pub line: usize,
	pub token: TokenType
}

impl Token {
	pub fn which(&self) -> String {
		match &self.token {
			TokenType::Literal(Literal::Int(n)) => format!("{}: {} -> Int {}", self.filename, self.line, n),
			TokenType::Literal(Literal::String(s)) => format!("{}: {} -> Str {}", self.filename, self.line, s),
			TokenType::Keyword(s) => format!("{}: {} -> Key {}", self.filename, self.line, s),
			TokenType::Symbol(s) => format!("{}: {} -> Sym {}", self.filename, self.line, s),
			TokenType::Identifier(s) => format!("{}: {} -> Ident {}", self.filename, self.line, s)
		}
	}
}