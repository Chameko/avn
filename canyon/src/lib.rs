mod token;
mod lexer;
mod ast;

use lexer::Lexer;

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_file_1() {
        let result = super::tokens_from_file("examples/test.cyon");
		let join = |num: &str, out: &str| format!("examples/test.cyon: {} -> {}", num, out);
        assert_eq!(result, vec![join("1", "Key let"),join("1", "Ident test"), join("1", "Sym ="), join("1", "Ident thingy"), join("1", "Sym ;"), join("2", "Int 1234"), join("3", "Key for"), join("4", "Key if"), join("5", "Str soijfseosjk"), join("6", "Sym +"), join("6", "Sym -"), join("6", "Sym !"), join("9", "Key if"), join("9", "Ident name"), join("9", "Sym =="), join("9", "Ident you"), join("9", "Sym {"), join("10", "Ident dab"), join("10", "Sym ("), join("10", "Sym )"), join("10", "Sym ;"), join("11", "Sym }")]);
    }
}
