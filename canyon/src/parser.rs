use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType, Literal};
use std::vec::IntoIter;
use std::iter::Peekable;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParserErrors {
	errors: Vec<ParsingError>
}
impl ParserErrors {
	pub fn is_errors(&self) -> bool {
		self.errors.len() != 0
	}

	pub fn new() -> Self {
		ParserErrors {
			errors: vec![]
		}
	}

	pub fn push(&mut self, err: ParsingError) {
		self.errors.push(err);
	}

	pub fn print_error(&self) -> String {
		use std::io::{BufReader, BufRead};
		use std::fs::File;
		let mut output = String::from("");
		
		// Get all the filenames
		let mut file_names: Vec<String> = self.errors.clone().into_iter().map(|p| p.file).collect();
		file_names.dedup();

		// Sort into vectors of errors in the same file
		let mut sorted_errors: Vec<Vec<ParsingError>> = vec![];
		for file in file_names {
			sorted_errors.push(self.errors.clone().into_iter().filter(|p| p.file == file ).collect())
		}

		for errors_in_file in sorted_errors {
			// Grab the file
			let mut file = BufReader::new(File::open(&errors_in_file.first().unwrap().file).unwrap()).lines();

			for mut error in errors_in_file {
				// Grab the code from the line the error references
				error.insert_code(file.nth(error.line.0 - 1).unwrap().unwrap());

				// Apply cool formating
				output.push_str(&error.print_error())
			}
		}

		output
	}
}

impl Display for ParserErrors {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
		write!(f, "{}", self.print_error())
	}
}

impl Error for ParserErrors {}

#[derive(Debug, Clone)]
pub struct ParsingError {
	file: String,
	line: (usize, String),
	details: String,
}

impl ParsingError {
	fn new(details: String, token: Token) -> ParsingError {
		let line = token.line;
		let file = token.filename;

		ParsingError {
			details,
			file,
			line: (line, "".to_string())
		}
	}

	fn line_number_spacer(line_n: &str) -> String {
		let greatest_length = line_n.len();
		let mut spacer = String::from("");
		spacer.push_str(line_n);
		for _ in line_n.len()..greatest_length {
			spacer.push(' ');
		}
		spacer
	}
	pub fn print_error(&self) -> String {
		let mut output = String::from("");

		output.push_str(&format!("{:-<10}\n", "Error"));
		output.push_str(&format!("{}|\n", Self::line_number_spacer("")));
		output.push_str(&format!("{}| {}\n", Self::line_number_spacer(&format!("{}", self.line.0)), self.line.1));
		output.push_str(&format!("{}|\n", Self::line_number_spacer("")));
		output.push_str(&format!("--> {}", self.details));
		output.push_str(&format!("--> {}:{}", self.file, self.line.0));
		output
	}

	pub fn insert_code(&mut self, code: String) {
		self.line.1 = code;
	}
}

impl Display for ParsingError {
	fn fmt(&self, f: &mut std::fmt:: Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.print_error())
	}
}

impl Error for ParsingError {}

pub struct Parser {
	token_stream: Peekable<IntoIter<Token>>,
}

impl Parser {
	pub fn new(lexer: Lexer) -> Result<Self, String> {
		let mut token_stream: Vec<Token> = vec![];

		for token in lexer {
			match token {
				Ok(t) => token_stream.push(t),
				Err(s) => return Err(s),
			}
		}

		Ok(Parser {
			token_stream: token_stream.into_iter().peekable()
		})
	}

	pub fn parse_program(&mut self) -> Result<Program, ParserErrors> {
		// Create a program and errors
		let program = Program{ statements: vec![] };
		let mut errors = ParserErrors::new();

		loop {
			if let Some(r) = self.parse_statement() {
				if let Err(e) = r {
					// If we get an error add it to the list and continue
					errors.push(e);
				}
			} else {
				// No more statements to parse so we leave
				break;
			}
		}

		if errors.is_errors() != true {
			Ok(program)
		} else {
			Err(errors)
		}
	}

	pub fn parse_statement(&mut self) -> Option<Result<Statement, ParsingError>> {
		let token = self.token_stream.next()?;
		match token.token.clone() {
			TokenType::Keyword(l) => { 
				match l.as_str() {
					"let" => Some(self.parse_let_statement(token)),
					_ => Some(Err(ParsingError::new("Expected statement".to_string(), token)))
				}
			},
			_ => None
		}
	}

	pub fn is_literal(to_check: Option<&Token>, details: String, root: &Token) -> Result<Literal, ParsingError> {
		if let Some(Token{ token: TokenType::Literal(l), ..}) = to_check {
			Ok(l.clone())
		} else {
			Err(ParsingError::new(details, root.clone()))
		}
	}

	pub fn is_identifier(to_check: Option<&Token>, details: String, root: &Token) -> Result<String, ParsingError> {
		if let Some(Token{ token: TokenType::Identifier(i), ..}) = to_check {
			Ok(i.clone())
		} else {
			Err(ParsingError::new(details, root.clone()))
		}
	}

	pub fn is_symbol(to_check: Option<&Token>, allowed: &[&str], details: String, root: &Token) -> Result<String, ParsingError> {
		if let Some(Token{ token: TokenType::Symbol(s), ..}) = to_check {
			for sym in allowed {
				if s == *sym {
					return Ok(sym.to_string())
				}
			}
			Err(ParsingError::new(details, root.clone()))
		} else {
			Err(ParsingError::new(details, root.clone()))
		}
	}

	pub fn is_keyword(to_check: Option<&Token>, allowed: &[&str], details: String, root: &Token) -> Result<String, ParsingError> {
		if let Some(Token{ token: TokenType::Keyword(k), ..}) = to_check {
			for key in allowed {
				if k == *key {
					return Ok(key.to_string())
				}
			}
			Err(ParsingError::new(details, root.clone()))
		} else {
			Err(ParsingError::new(details, root.clone()))
		}
	}

	pub fn parse_let_statement(&mut self, first_token: Token) -> Result<Statement, ParsingError> {
		// Check if the next token is a literal
		let value = Parser::is_identifier(self.token_stream.peek(), "Expected variable name".to_string(), &first_token)?;

		// Create identifier token and advance (unwrap is safe as it would have panicked above otherwise)
		let name = Identifier{ token: self.token_stream.next().unwrap(), value };

		// Check if the next token is a symbol
		Parser::is_symbol(self.token_stream.peek(), &["="], "Expected =".to_string(), &first_token)?;

		// Advance the token stream
		self.token_stream.next();

		// look until semicolon for now
		while Parser::is_symbol(self.token_stream.peek(), &[";"], "Expected ;".to_string(), &first_token).is_err() {
			if let None = self.token_stream.next() {
				return Err(ParsingError::new("Expected ;".to_string(), first_token.clone()))
			};
		}

		let value = Expression::Null;

		Ok( Statement::LetStatement(LetStatement{token: first_token, name, value}) )
	}
}