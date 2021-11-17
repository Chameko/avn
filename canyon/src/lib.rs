mod token;
mod lexer;
mod ast;
mod parser;

use lexer::Lexer;
use parser::{Parser, ParserErrors};

pub fn tokens_from_text(program: &str) -> Vec<String> {
	let lex = Lexer::from_text(program);
	let mut vec: Vec<String> = vec![];
	for token in lex {
		match token {
			Ok(t) => vec.push(t.which()),
			Err(s) => vec.push(s.to_string()),
		}
	}
	vec
}

pub fn tokens_from_file(path: &str) -> Vec<String> {
	let lex = Lexer::from_file(path).unwrap();
	let mut vec: Vec<String> = vec![];
	for token in lex {
		match token {
			Ok(t) => vec.push(t.which()),
			Err(s) => vec.push(s.to_string()),
		}
	}
	vec
}

pub fn count_statements(path: &str) -> Result<usize, ParserErrors> {
	let lex = Lexer::from_file(path).unwrap();
	let mut par = Parser::new(lex).unwrap();
	Ok(par.parse_program()?.statements.len())
}

pub fn print_statements(path: &str) -> String {
	let lex = Lexer::from_file(path).unwrap();
	let mut par = Parser::new(lex).unwrap();
	let statements = par.parse_program().unwrap().statements;
	let mut output = "".to_string();
	for statement in statements {
		output.push_str(&format!("{:?}", statement));
	}
	output
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lexer_1() {
        let result = super::tokens_from_file("examples/test.cyon");
		let join = |num: &str, out: &str| format!("examples/test.cyon: {} -> {}", num, out);
        assert_eq!(result, vec![join("1", "Key let"),join("1", "Ident test"), join("1", "Sym ="), join("1", "Ident thingy"), join("1", "Sym ;"), join("2", "Int 1234"), join("3", "Key for"), join("4", "Key if"), join("5", "Str soijfseosjk"), join("6", "Sym +"), join("6", "Sym -"), join("6", "Sym !"), join("9", "Key if"), join("9", "Ident name"), join("9", "Sym =="), join("9", "Ident you"), join("9", "Sym {"), join("10", "Ident dab"), join("10", "Sym ("), join("10", "Sym )"), join("10", "Sym ;"), join("11", "Sym }")]);
    }

	#[test]
	fn test_parser_1() {
		let result = super::count_statements("examples/test2.cyon").unwrap();
		assert_eq!(result, 3)
	}

	#[test]
	fn test_parser_2() {
		use crate::ast::Statement;
		use crate::token::{TokenType};
		let lex = super::Lexer::from_file("examples/test2.cyon").unwrap();
		let mut par = super::Parser::new(lex).unwrap();
		let statement = par.parse_program().unwrap().statements[0].clone();
		let mut tktype: String = "".to_string();
		let mut ident_name: String = "".to_string();
		let value: String;
		if let Statement::LetStatement(l) = statement {
			if let TokenType::Identifier(name) = l.name.token.token {
				ident_name = name;
			}
			if let TokenType::Keyword(name) = l.token.token {
				tktype = name;
			}
		}
		assert_eq!((tktype, ident_name), ("let".to_string(), "x".to_string()))
	}
}
