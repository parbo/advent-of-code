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

struct Result(Token, usize, bool);

fn whitespace_tokenizer(a: &str) -> Option<Result> {
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
        _ => Some(Result(Token::Whitespace, consumed, false)),
    }
}

fn comment_tokenizer(a: &str) -> Option<Result> {
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
        _ => Some(Result(Token::Comment, consumed, false)),
    }
}

fn block_comment_tokenizer(a: &str) -> Option<Result> {
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
        _ => Some(Result(Token::BlockComment, consumed, false)),
    }
}

fn open_paren_tokenizer(a: &str) -> Option<Result> {
    if a.chars().next()? == '(' {
        Some(Result(Token::OpenParen, 1, true))
    } else {
        None
    }
}

fn close_paren_tokenizer(a: &str) -> Option<Result> {
    if a.chars().next()? == ')' {
        Some(Result(Token::CloseParen, 1, true))
    } else {
        None
    }
}

fn open_brace_tokenizer(a: &str) -> Option<Result> {
    if a.chars().next()? == '{' {
        Some(Result(Token::OpenBrace, 1, true))
    } else {
        None
    }
}

fn close_brace_tokenizer(a: &str) -> Option<Result> {
    if a.chars().next()? == '}' {
        Some(Result(Token::CloseBrace, 1, true))
    } else {
        None
    }
}

fn semi_colon_tokenizer(a: &str) -> Option<Result> {
    if a.chars().next()? == ';' {
        Some(Result(Token::SemiColon, 1, true))
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

fn identifier_tokenizer(a: &str) -> Option<Result> {
    match match_identifier(a) {
        Some(id) if !is_keyword(id) => {
            Some(Result(Token::Identifier(id.to_string()), id.len(), true))
        }
        _ => None,
    }
}

fn keyword_tokenizer(a: &str) -> Option<Result> {
    // Same as identifier, but with a reversed check for keyword-ness
    match match_identifier(a) {
        Some(id) => match match_keyword(id) {
            Some(op) => Some(Result(Token::Keyword(op), id.len(), true)),
            _ => None,
        },
        _ => None,
    }
}

fn integer_tokenizer(a: &str) -> Option<Result> {
    if a.is_empty() {
        return None;
    }
    let mut pos = 0;
    // Skip minus sign if any
    if a.chars().next()? == '-' {
        pos += 1;
    }
    let mut consumed = 0;
    for (offset, char) in a[pos..].char_indices() {
        if is_decimal_number(char) {
            consumed = offset + 1;
        } else {
            break;
        }
    }
    match consumed {
        0 => None,
        _ => Some(Result(
            Token::Integer(
                a[0..(pos + consumed)]
                    .parse()
                    .expect("error parsing integer"),
            ),
            pos + consumed,
            true,
        )),
    }
}

fn string_tokenizer(a: &str) -> Option<Result> {
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
            _ => Some(Result(
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
    let tokenizers: [fn(&str) -> Option<Result>; 12] = [
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
            if let Some(Result(token, consumed, emit)) = tokenizer(&text[pos..]) {
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

#[test]
fn test_comment_tokenizer_comment() {
    let result = comment_tokenizer("// blah");
    match result {
        None => assert!(false),
        Some(Result(token, consumed, emit)) => {
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
    assert_eq!(tokenize("-1"), [Token::Integer(-1)]);
    assert_eq!(tokenize("-123"), [Token::Integer(-123)]);
    assert_eq!(tokenize("1 2"), [Token::Integer(1), Token::Integer(2)]);
    assert_eq!(
        tokenize("123 321"),
        [Token::Integer(123), Token::Integer(321)]
    );
    assert_eq!(tokenize("-1-1"), [Token::Integer(-1), Token::Integer(-1)]);
    assert_eq!(tokenize("\"test\""), [Token::Str(String::from("test"))]);
    assert_eq!(tokenize("x"), [Token::Identifier(String::from("x"))]);
    assert_eq!(tokenize("_x"), [Token::Identifier(String::from("_x"))]);
    assert_eq!(
        tokenize("y_2"),
        [Token::Identifier(String::from("y_2"))]
    );
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
