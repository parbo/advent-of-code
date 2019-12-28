use std::error::Error as StdError;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    Int,
    If,
    Return,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Whitespace,
    BlockComment,
    Comment,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SemiColon,
    IntegerType,
    Identifier(String),
    Keyword(Keyword),
    Integer(i128),
    Str(String),
}

struct TokenizeResult(Token, usize, bool);

fn whitespace_tokenizer(a: &str) -> Option<TokenizeResult> {
    let mut consumed = 0;
    for (offset, char) in a.char_indices() {
        if char.is_whitespace() {
            consumed = offset + 1;
        } else {
            break;
        }
    }
    match consumed {
        0 => None,
        _ => Some(TokenizeResult(Token::Whitespace, consumed, false)),
    }
}

fn comment_tokenizer(a: &str) -> Option<TokenizeResult> {
    let mut consumed = 0;
    if a.starts_with("//") {
        consumed += 2;
        for (offset, char) in a[2..].char_indices() {
            if char == '\n' {
                break;
            } else {
                consumed = offset + 3;
            }
        }
    }
    match consumed {
        0 => None,
        _ => Some(TokenizeResult(Token::Comment, consumed, false)),
    }
}

fn block_comment_tokenizer(a: &str) -> Option<TokenizeResult> {
    let mut consumed = 0;
    if a.starts_with("/*") {
        consumed += 2;
        while !a[consumed..].starts_with("*/") {
            consumed += 1;
        }
        consumed += 2;
    }
    match consumed {
        0 => None,
        _ => Some(TokenizeResult(Token::BlockComment, consumed, false)),
    }
}

fn open_paren_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '(' {
        Some(TokenizeResult(Token::OpenParen, 1, true))
    } else {
        None
    }
}

fn close_paren_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == ')' {
        Some(TokenizeResult(Token::CloseParen, 1, true))
    } else {
        None
    }
}

fn open_brace_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '{' {
        Some(TokenizeResult(Token::OpenBrace, 1, true))
    } else {
        None
    }
}

fn close_brace_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '}' {
        Some(TokenizeResult(Token::CloseBrace, 1, true))
    } else {
        None
    }
}

fn semi_colon_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == ';' {
        Some(TokenizeResult(Token::SemiColon, 1, true))
    } else {
        None
    }
}

fn is_identifier_start(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '_' => true,
        _ => false,
    }
}

fn is_identifier_rest(c: char) -> bool {
    match c {
        '0'..='9' => true,
        'a'..='z' => true,
        'A'..='Z' => true,
        '_' => true,
        _ => false,
    }
}

fn is_decimal_number(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn match_keyword(a: &str) -> Option<Keyword> {
    match a {
        "int" => Some(Keyword::Int),
        "if" => Some(Keyword::If),
        "return" => Some(Keyword::Return),
        _ => None,
    }
}

fn is_keyword(a: &str) -> bool {
    match match_keyword(a) {
        Some(_) => true,
        _ => false,
    }
}

fn match_identifier(a: &str) -> Option<&str> {
    if is_identifier_start(a.chars().next()?) {
        let mut consumed = 0;
        for (offset, char) in a.char_indices() {
            if is_identifier_rest(char) {
                consumed = offset + 1;
            } else {
                break;
            }
        }
        match consumed {
            0 => None,
            _ => Some(&a[0..consumed]),
        }
    } else {
        None
    }
}

fn identifier_tokenizer(a: &str) -> Option<TokenizeResult> {
    match match_identifier(a) {
        Some(id) if !is_keyword(id) => Some(TokenizeResult(
            Token::Identifier(id.to_string()),
            id.len(),
            true,
        )),
        _ => None,
    }
}

fn keyword_tokenizer(a: &str) -> Option<TokenizeResult> {
    // Same as identifier, but with a reversed check for keyword-ness
    match match_identifier(a) {
        Some(id) => match match_keyword(id) {
            Some(op) => Some(TokenizeResult(Token::Keyword(op), id.len(), true)),
            _ => None,
        },
        _ => None,
    }
}

fn integer_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.is_empty() {
        return None;
    }
    let mut consumed = 0;
    for (offset, char) in a.char_indices() {
        if is_decimal_number(char) {
            consumed = offset + 1;
        } else {
            break;
        }
    }
    match consumed {
        0 => None,
        _ => Some(TokenizeResult(
            Token::Integer(a[0..consumed].parse().expect("error parsing integer")),
            consumed,
            true,
        )),
    }
}

fn string_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.len() <= 1 {
        return None;
    }
    if a.chars().next()? == '"' && a.len() > 1 {
        let mut consumed = 0;
        for (offset, char) in a[1..].char_indices() {
            if char == '"' {
                break;
            } else {
                consumed = offset + 1;
            }
        }
        match consumed {
            0 => None,
            _ => Some(TokenizeResult(
                Token::Str(String::from(&a[1..=consumed])),
                consumed + 2,
                true,
            )), // Add 2 for the "'s
        }
    } else {
        None
    }
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let tokenizers: [fn(&str) -> Option<TokenizeResult>; 12] = [
        whitespace_tokenizer,
        block_comment_tokenizer,
        comment_tokenizer,
        open_paren_tokenizer,
        close_paren_tokenizer,
        open_brace_tokenizer,
        close_brace_tokenizer,
        semi_colon_tokenizer,
        identifier_tokenizer,
        keyword_tokenizer,
        integer_tokenizer,
        string_tokenizer,
    ];

    let mut tokenlist = Vec::<Token>::new();
    let mut pos: usize = 0;
    loop {
        let last_pos = pos;
        for &tokenizer in tokenizers.iter() {
            if let Some(TokenizeResult(token, consumed, emit)) = tokenizer(&text[pos..]) {
                if emit {
                    tokenlist.push(token)
                }
                pos += consumed;
                break;
            }
        }
        if pos == last_pos || pos == text.len() {
            break;
        }
    }

    tokenlist
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Constant(i128),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Return(Expression),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    Function(String, Statement),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Program {
    Program(Function),
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::SyntaxError => "SyntaxError",
        }
    }
}

fn expression_parser(tokens: &[Token]) -> Option<(Expression, usize)> {
    let mut it = tokens.iter();
    if let Token::Integer(x) = it.next()? {
        Some((Expression::Constant(*x), 1))
    } else {
        None
    }
}

fn statement_parser(tokens: &[Token]) -> Option<(Statement, usize)> {
    let mut it = tokens.iter();
    if let Token::Keyword(Keyword::Return) = it.next()? {
        if let Some((exp, offset)) = expression_parser(&tokens[1..]) {
            if let Token::SemiColon = tokens[(1 + offset)..].iter().next()? {
                return Some((Statement::Return(exp), 1 + offset + 1));
            }
        }
    }
    None
}

fn function_parser(tokens: &[Token]) -> Option<(Function, usize)> {
    let mut it = tokens.iter();
    if let Token::Keyword(Keyword::Int) = it.next()? {
        if let Token::Identifier(name) = it.next()? {
            if let Token::OpenParen = it.next()? {
                if let Token::CloseParen = it.next()? {
                    if let Token::OpenBrace = it.next()? {
                        if let Some((statement, offset)) = statement_parser(&tokens[5..]) {
                            if let Token::CloseBrace = tokens[(5 + offset)..].iter().next()? {
                                return Some((
                                    Function::Function(name.clone(), statement),
                                    5 + offset + 1,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn program_parser(tokens: &[Token]) -> Option<(Program, usize)> {
    if let Some((function, offset)) = function_parser(tokens) {
        return Some((Program::Program(function), offset));
    }
    None
}

pub fn parse(a: &[Token]) -> Result<Program, ParseError> {
    if let Some((program, i)) = program_parser(a) {
        if i != a.len() {
            return Err(ParseError::SyntaxError);
        }
        Ok(program)
    } else {
        Err(ParseError::SyntaxError)
    }
}

// fn generate_expression(e: &Expression) -> Vec<String> {
//     let mut r = vec![];
//     match e {
// 	Expression::Constant(c) => {
// 	    let s = format!("ADD 0 {} blah", c);
// 	    r.push(s);
// 	}
//     }
//     r
// }

fn generate_statement(s: &Statement) -> Vec<String> {
    let mut r = vec![];
    match s {
	Statement::Return(exp) => {
	    match exp {
		Expression::Constant(x) => {
		    r.push(format!("ADD 0 {} [SP+1]", x));
		}
	    }
	    r.push("JIT 1 [SP+0]".to_string());
	}
    }
    r
}

fn generate_program(p: &Program) -> Vec<String> {
    let mut r = vec![];
    match p {
	Program::Program(Function::Function(_name, statement)) => {
	    r.push("ADD 1 [SP+0] [SP+0]".to_string());
	    let v = generate_statement(&statement);
	    r.extend(v);
	}
    }
    r
}

pub fn generate(p: &Program) -> Vec<String> {
    generate_program(p)
}

#[test]
fn test_comment_tokenizer_comment() {
    let result = comment_tokenizer("// blah");
    match result {
        None => assert!(false),
        Some(TokenizeResult(token, consumed, emit)) => {
            assert_eq!(token, Token::Comment);
            assert_eq!(consumed, 7);
            assert_eq!(emit, false);
        }
    }
}

#[test]
fn test_comment_tokenizer_non_comment() {
    let result = comment_tokenizer("blah");
    match result {
        None => {}
        Some(_) => assert!(false),
    }
}

#[test]
fn test_tokenizer() {
    assert_eq!(tokenize("1 // apa"), [Token::Integer(1)]);
    assert_eq!(
        tokenize("1 // apa\n2"),
        [Token::Integer(1), Token::Integer(2)]
    );
    assert_eq!(tokenize("1"), [Token::Integer(1)]);
    assert_eq!(tokenize("123"), [Token::Integer(123)]);
    assert_eq!(tokenize("1 2"), [Token::Integer(1), Token::Integer(2)]);
    assert_eq!(
        tokenize("123 321"),
        [Token::Integer(123), Token::Integer(321)]
    );
    assert_eq!(tokenize("\"test\""), [Token::Str(String::from("test"))]);
    assert_eq!(tokenize("x"), [Token::Identifier(String::from("x"))]);
    assert_eq!(tokenize("_x"), [Token::Identifier(String::from("_x"))]);
    assert_eq!(tokenize("y_2"), [Token::Identifier(String::from("y_2"))]);
    assert_eq!(tokenize("int"), [Token::Keyword(Keyword::Int)]);
    assert_eq!(
        tokenize("intblaj"),
        [Token::Identifier(String::from("intblaj"))]
    );
    assert_eq!(
        tokenize("(1 2)"),
        [
            Token::OpenParen,
            Token::Integer(1),
            Token::Integer(2),
            Token::CloseParen
        ]
    );
    assert_eq!(
        tokenize("int main() {\n  return 2;\n}\n"),
        [
            Token::Keyword(Keyword::Int),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Integer(2),
            Token::SemiColon,
            Token::CloseBrace
        ]
    );
}

#[test]
fn test_parser() {
    let tokens = tokenize("int main() {\n  return 2;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            Statement::Return(Expression::Constant(2))
        ))
    );
}

#[test]
#[should_panic]
fn test_syntax_error_1() {
    parse(&tokenize("main() {\n  return 2;\n}\n")).expect("error");
}

#[test]
#[should_panic]
fn test_syntax_error_2() {
    parse(&tokenize("int main) {\n  return 2;\n}\n")).expect("error");
}

#[test]
#[should_panic]
fn test_syntax_error_3() {
    parse(&tokenize("int main() {\n  return;\n}\n")).expect("error");
}

#[test]
#[should_panic]
fn test_syntax_error_4() {
    parse(&tokenize("int main() {\n  return 2;\n")).expect("error");
}

#[test]
fn test_generate() {
    let tokens = tokenize("int main() {\n  return 2;\n}\n");
    let program = parse(&tokens).expect("error");
    let asm = generate(&program);
    for a in asm {
	println!("{}", a);
    }
    assert_eq!(5, 4);
}
