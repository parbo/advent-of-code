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
    Minus,
    Bang,
    Plus,
    Asterisk,
    Slash,
    Percent,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Assign,
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

fn minus_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '-' {
        Some(TokenizeResult(Token::Minus, 1, true))
    } else {
        None
    }
}

fn bang_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '!' {
        Some(TokenizeResult(Token::Bang, 1, true))
    } else {
        None
    }
}

fn plus_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '+' {
        Some(TokenizeResult(Token::Plus, 1, true))
    } else {
        None
    }
}

fn asterisk_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '*' {
        Some(TokenizeResult(Token::Asterisk, 1, true))
    } else {
        None
    }
}

fn slash_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '/' {
        Some(TokenizeResult(Token::Slash, 1, true))
    } else {
        None
    }
}

fn percent_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '%' {
        Some(TokenizeResult(Token::Percent, 1, true))
    } else {
        None
    }
}

fn and_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("&&") {
        Some(TokenizeResult(Token::And, 2, true))
    } else {
        None
    }
}

fn or_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("||") {
        Some(TokenizeResult(Token::Or, 2, true))
    } else {
        None
    }
}

fn equal_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("==") {
        Some(TokenizeResult(Token::Equal, 2, true))
    } else {
        None
    }
}

fn not_equal_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("!=") {
        Some(TokenizeResult(Token::NotEqual, 2, true))
    } else {
        None
    }
}

fn less_than_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("<") {
        Some(TokenizeResult(Token::LessThan, 1, true))
    } else {
        None
    }
}

fn less_than_or_equal_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("<=") {
        Some(TokenizeResult(Token::LessThanOrEqual, 2, true))
    } else {
        None
    }
}

fn greater_than_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with(">") {
        Some(TokenizeResult(Token::GreaterThan, 1, true))
    } else {
        None
    }
}

fn greater_than_or_equal_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with(">=") {
        Some(TokenizeResult(Token::GreaterThanOrEqual, 2, true))
    } else {
        None
    }
}

fn assign_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.starts_with("=") {
        Some(TokenizeResult(Token::Assign, 1, true))
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
    let tokenizers: [fn(&str) -> Option<TokenizeResult>; 27] = [
        whitespace_tokenizer,
        block_comment_tokenizer,
        comment_tokenizer,
        open_paren_tokenizer,
        close_paren_tokenizer,
        open_brace_tokenizer,
        close_brace_tokenizer,
        semi_colon_tokenizer,
        and_tokenizer,
        or_tokenizer,
        equal_tokenizer,
        not_equal_tokenizer,
        less_than_or_equal_tokenizer,
        less_than_tokenizer,
        greater_than_or_equal_tokenizer,
        greater_than_tokenizer,
        minus_tokenizer,
        bang_tokenizer,
        plus_tokenizer,
        asterisk_tokenizer,
        slash_tokenizer,
        percent_tokenizer,
        assign_tokenizer,
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
pub enum UnaryOperator {
    Negation,
    LogicalNegation,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Constant(i128),
    UnaryOperator(UnaryOperator, Box<Expression>),
    BinaryOperator(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(String, Box<Expression>),
    VariableReference(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Return(Expression),
    Declaration(String, Option<Expression>),
    Expression(Expression),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    Function(String, Vec<Statement>),
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

fn parse_factor(tokens: &[Token]) -> Option<(Expression, usize)> {
    let mut it = tokens.iter();
    match it.next()? {
        Token::OpenParen => {
            let (exp, offset) = parse_expression(&tokens[1..])?;
            if Token::CloseParen == *tokens[(1 + offset)..].iter().next()? {
                Some((exp, 1 + offset + 1))
            } else {
                None
            }
        }
        Token::Minus => {
            let (exp, offset) = parse_factor(&tokens[1..])?;
            Some((
                Expression::UnaryOperator(UnaryOperator::Negation, Box::new(exp)),
                1 + offset,
            ))
        }
        Token::Bang => {
            let (exp, offset) = parse_factor(&tokens[1..])?;
            Some((
                Expression::UnaryOperator(UnaryOperator::LogicalNegation, Box::new(exp)),
                1 + offset,
            ))
        }
        Token::Integer(x) => Some((Expression::Constant(*x), 1)),
        Token::Identifier(name) => Some((Expression::VariableReference(name.clone()), 1)),
        _ => None,
    }
}

fn parse_term(tokens: &[Token]) -> Option<(Expression, usize)> {
    let (f, o) = parse_factor(tokens)?;
    let mut factor = f;
    let mut offset = o;
    loop {
        let mut it = tokens[offset..].iter();
        match it.next() {
            Some(Token::Asterisk) => {
                let (next_factor, next_offset) = parse_factor(&tokens[(1 + offset)..])?;
                factor = Expression::BinaryOperator(
                    BinaryOperator::Multiplication,
                    Box::new(factor),
                    Box::new(next_factor),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::Slash) => {
                let (next_factor, next_offset) = parse_factor(&tokens[(1 + offset)..])?;
                factor = Expression::BinaryOperator(
                    BinaryOperator::Division,
                    Box::new(factor),
                    Box::new(next_factor),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::Percent) => {
                let (next_factor, next_offset) = parse_factor(&tokens[(1 + offset)..])?;
                factor = Expression::BinaryOperator(
                    BinaryOperator::Modulo,
                    Box::new(factor),
                    Box::new(next_factor),
                );
                offset = offset + 1 + next_offset;
            }
            _ => break,
        }
    }
    Some((factor, offset))
}

fn parse_additive_expression(tokens: &[Token]) -> Option<(Expression, usize)> {
    let (t, o) = parse_term(tokens)?;
    let mut term = t;
    let mut offset = o;
    loop {
        let mut it = tokens[offset..].iter();
        match it.next() {
            Some(Token::Plus) => {
                let (next_term, next_offset) = parse_term(&tokens[(1 + offset)..])?;
                term = Expression::BinaryOperator(
                    BinaryOperator::Addition,
                    Box::new(term),
                    Box::new(next_term),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::Minus) => {
                let (next_term, next_offset) = parse_term(&tokens[(1 + offset)..])?;
                term = Expression::BinaryOperator(
                    BinaryOperator::Subtraction,
                    Box::new(term),
                    Box::new(next_term),
                );
                offset = offset + 1 + next_offset;
            }
            _ => break,
        }
    }
    Some((term, offset))
}

fn parse_relational_expression(tokens: &[Token]) -> Option<(Expression, usize)> {
    let (ae, o) = parse_additive_expression(tokens)?;
    let mut additive_expression = ae;
    let mut offset = o;
    loop {
        let mut it = tokens[offset..].iter();
        match it.next() {
            Some(Token::LessThan) => {
                let (next_additive_expression, next_offset) =
                    parse_additive_expression(&tokens[(1 + offset)..])?;
                additive_expression = Expression::BinaryOperator(
                    BinaryOperator::LessThan,
                    Box::new(additive_expression),
                    Box::new(next_additive_expression),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::LessThanOrEqual) => {
                let (next_additive_expression, next_offset) =
                    parse_additive_expression(&tokens[(1 + offset)..])?;
                additive_expression = Expression::BinaryOperator(
                    BinaryOperator::LessThanOrEqual,
                    Box::new(additive_expression),
                    Box::new(next_additive_expression),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::GreaterThan) => {
                let (next_additive_expression, next_offset) =
                    parse_additive_expression(&tokens[(1 + offset)..])?;
                additive_expression = Expression::BinaryOperator(
                    BinaryOperator::GreaterThan,
                    Box::new(additive_expression),
                    Box::new(next_additive_expression),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::GreaterThanOrEqual) => {
                let (next_additive_expression, next_offset) =
                    parse_additive_expression(&tokens[(1 + offset)..])?;
                additive_expression = Expression::BinaryOperator(
                    BinaryOperator::GreaterThanOrEqual,
                    Box::new(additive_expression),
                    Box::new(next_additive_expression),
                );
                offset = offset + 1 + next_offset;
            }
            _ => break,
        }
    }
    Some((additive_expression, offset))
}

fn parse_equality_expression(tokens: &[Token]) -> Option<(Expression, usize)> {
    let (re, o) = parse_relational_expression(tokens)?;
    let mut relational_expression = re;
    let mut offset = o;
    loop {
        let mut it = tokens[offset..].iter();
        match it.next() {
            Some(Token::Equal) => {
                let (next_relational_expression, next_offset) =
                    parse_relational_expression(&tokens[(1 + offset)..])?;
                relational_expression = Expression::BinaryOperator(
                    BinaryOperator::Equal,
                    Box::new(relational_expression),
                    Box::new(next_relational_expression),
                );
                offset = offset + 1 + next_offset;
            }
            Some(Token::NotEqual) => {
                let (next_relational_expression, next_offset) =
                    parse_relational_expression(&tokens[(1 + offset)..])?;
                relational_expression = Expression::BinaryOperator(
                    BinaryOperator::NotEqual,
                    Box::new(relational_expression),
                    Box::new(next_relational_expression),
                );
                offset = offset + 1 + next_offset;
            }
            _ => break,
        }
    }
    Some((relational_expression, offset))
}

fn parse_logical_and_expression(tokens: &[Token]) -> Option<(Expression, usize)> {
    let (ee, o) = parse_equality_expression(tokens)?;
    let mut equality_expression = ee;
    let mut offset = o;
    loop {
        let mut it = tokens[offset..].iter();
        match it.next() {
            Some(Token::And) => {
                let (next_equality_expression, next_offset) =
                    parse_equality_expression(&tokens[(1 + offset)..])?;
                equality_expression = Expression::BinaryOperator(
                    BinaryOperator::And,
                    Box::new(equality_expression),
                    Box::new(next_equality_expression),
                );
                offset = offset + 1 + next_offset;
            }
            _ => break,
        }
    }
    Some((equality_expression, offset))
}

fn parse_logical_or_expression(tokens: &[Token]) -> Option<(Expression, usize)> {
    let (lae, o) = parse_logical_and_expression(tokens)?;
    let mut logical_and_expression = lae;
    let mut offset = o;
    loop {
        let mut it = tokens[offset..].iter();
        match it.next() {
            Some(Token::Or) => {
                let (next_logical_and_expression, next_offset) =
                    parse_logical_and_expression(&tokens[(1 + offset)..])?;
                logical_and_expression = Expression::BinaryOperator(
                    BinaryOperator::Or,
                    Box::new(logical_and_expression),
                    Box::new(next_logical_and_expression),
                );
                offset = offset + 1 + next_offset;
            }
            _ => break,
        }
    }
    Some((logical_and_expression, offset))
}

fn parse_expression(tokens: &[Token]) -> Option<(Expression, usize)> {
    let mut offset = 0;
    if let Some(Token::Identifier(name)) = tokens[offset..].iter().next() {
        offset = offset + 1;
        if let Some(Token::Assign) = tokens[offset..].iter().next() {
            offset = offset + 1;
            if let Some((exp, new_offset)) = parse_expression(&tokens[offset..]) {
                return Some((
                    Expression::Assignment(name.clone(), Box::new(exp)),
                    offset + new_offset,
                ));
            }
        }
    }
    parse_logical_or_expression(tokens)
}

fn parse_statement(tokens: &[Token]) -> Option<(Statement, usize)> {
    let mut it = tokens.iter();
    match it.next() {
        Some(Token::Keyword(Keyword::Return)) => {
            if let Some((exp, offset)) = parse_expression(&tokens[1..]) {
                if let Token::SemiColon = tokens[(1 + offset)..].iter().next()? {
                    return Some((Statement::Return(exp), 1 + offset + 1));
                }
            }
        }
        Some(Token::Keyword(Keyword::Int)) => {
            let mut offset = 1;
            let mut opt_assign = None;
            if let Token::Identifier(name) = tokens[offset..].iter().next()? {
                offset = offset + 1;
                if let Some(Token::Assign) = tokens[offset..].iter().next() {
                    offset = offset + 1;
                    if let Some((exp, new_offset)) = parse_expression(&tokens[offset..]) {
                        offset = offset + new_offset;
                        opt_assign = Some(exp);
                    } else {
                        return None;
                    }
                }
                if let Token::SemiColon = tokens[offset..].iter().next()? {
                    return Some((Statement::Declaration(name.clone(), opt_assign), offset + 1));
                }
            }
        }
        _ => {
            if let Some((exp, new_offset)) = parse_expression(&tokens) {
                if let Token::SemiColon = tokens[new_offset..].iter().next()? {
                    return Some((Statement::Expression(exp), new_offset + 1));
                }
            }
        }
    }
    None
}

fn parse_function(tokens: &[Token]) -> Option<(Function, usize)> {
    let mut it = tokens.iter();
    if let Token::Keyword(Keyword::Int) = it.next()? {
        if let Token::Identifier(name) = it.next()? {
            if let Token::OpenParen = it.next()? {
                if let Token::CloseParen = it.next()? {
                    if let Token::OpenBrace = it.next()? {
                        let mut statements = vec![];
                        let mut offset = 5;
                        loop {
                            if let Some((statement, new_offset)) =
                                parse_statement(&tokens[offset..])
                            {
                                statements.push(statement);
                                offset = offset + new_offset;
                            } else {
                                break;
                            }
                        }
                        if let Token::CloseBrace = tokens[offset..].iter().next()? {
                            return Some((
                                Function::Function(name.clone(), statements),
                                offset + 1,
                            ));
                        }
                    }
                }
            }
        }
    }
    None
}

fn parse_program(tokens: &[Token]) -> Option<(Program, usize)> {
    if let Some((function, offset)) = parse_function(tokens) {
        return Some((Program::Program(function), offset));
    }
    None
}

pub fn parse(a: &[Token]) -> Result<Program, ParseError> {
    if let Some((program, i)) = parse_program(a) {
        if i != a.len() {
            return Err(ParseError::SyntaxError);
        }
        Ok(program)
    } else {
        Err(ParseError::SyntaxError)
    }
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
    assert_eq!(tokenize("-1"), [Token::Minus, Token::Integer(1)]);
    assert_eq!(tokenize("!1"), [Token::Bang, Token::Integer(1)]);
    assert_eq!(
        tokenize("1+2"),
        [Token::Integer(1), Token::Plus, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1*2"),
        [Token::Integer(1), Token::Asterisk, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1/2"),
        [Token::Integer(1), Token::Slash, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1%2"),
        [Token::Integer(1), Token::Percent, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1==2"),
        [Token::Integer(1), Token::Equal, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1!=2"),
        [Token::Integer(1), Token::NotEqual, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1<2"),
        [Token::Integer(1), Token::LessThan, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1<=2"),
        [Token::Integer(1), Token::LessThanOrEqual, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1>2"),
        [Token::Integer(1), Token::GreaterThan, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1>=2"),
        [
            Token::Integer(1),
            Token::GreaterThanOrEqual,
            Token::Integer(2)
        ]
    );
    assert_eq!(
        tokenize("1&&2"),
        [Token::Integer(1), Token::And, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1||2"),
        [Token::Integer(1), Token::Or, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("1=2"),
        [Token::Integer(1), Token::Assign, Token::Integer(2)]
    );
    assert_eq!(
        tokenize("3-1"),
        [Token::Integer(3), Token::Minus, Token::Integer(1)]
    );
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
        tokenize("-x"),
        [Token::Minus, Token::Identifier(String::from("x"))]
    );
    assert_eq!(
        tokenize("!x"),
        [Token::Bang, Token::Identifier(String::from("x"))]
    );
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
            vec![Statement::Return(Expression::Constant(2))]
        ))
    );
}

#[test]
fn test_parse_unary_operator() {
    assert_eq!(
        parse_expression(&tokenize("-7")).expect("error"),
        (
            Expression::UnaryOperator(UnaryOperator::Negation, Box::new(Expression::Constant(7))),
            2
        )
    );
    assert_eq!(
        parse_expression(&tokenize("!1")).expect("error"),
        (
            Expression::UnaryOperator(
                UnaryOperator::LogicalNegation,
                Box::new(Expression::Constant(1))
            ),
            2
        )
    );
}

#[test]
fn test_parse_associativity() {
    let tokens = tokenize("int main() {\n  return 1 - 2 - 3;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![Statement::Return(Expression::BinaryOperator(
                BinaryOperator::Subtraction,
                Box::new(Expression::BinaryOperator(
                    BinaryOperator::Subtraction,
                    Box::new(Expression::Constant(1)),
                    Box::new(Expression::Constant(2))
                )),
                Box::new(Expression::Constant(3))
            ))]
        ))
    );
}

#[test]
fn test_parse_associativity_2() {
    let tokens = tokenize("int main() {\n  return 6 / 3 / 2;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![Statement::Return(Expression::BinaryOperator(
                BinaryOperator::Division,
                Box::new(Expression::BinaryOperator(
                    BinaryOperator::Division,
                    Box::new(Expression::Constant(6)),
                    Box::new(Expression::Constant(3))
                )),
                Box::new(Expression::Constant(2))
            ))]
        ))
    );
}

#[test]
fn test_parse_precedence() {
    let tokens = tokenize("int main() {\n  return 2 + 3 * 4;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![Statement::Return(Expression::BinaryOperator(
                BinaryOperator::Addition,
                Box::new(Expression::Constant(2)),
                Box::new(Expression::BinaryOperator(
                    BinaryOperator::Multiplication,
                    Box::new(Expression::Constant(3)),
                    Box::new(Expression::Constant(4))
                ))
            ))]
        ))
    );
}

#[test]
fn test_parse_and_or_precedence() {
    let tokens = tokenize("int main() {\n  return 1 || 0 && 2;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![Statement::Return(Expression::BinaryOperator(
                BinaryOperator::Or,
                Box::new(Expression::Constant(1)),
                Box::new(Expression::BinaryOperator(
                    BinaryOperator::And,
                    Box::new(Expression::Constant(0)),
                    Box::new(Expression::Constant(2))
                ))
            ))]
        ))
    );
}

#[test]
fn test_parse_and_or_precedence_2() {
    let tokens = tokenize("int main() {\n  return (1 || 0) && 0;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![Statement::Return(Expression::BinaryOperator(
                BinaryOperator::And,
                Box::new(Expression::BinaryOperator(
                    BinaryOperator::Or,
                    Box::new(Expression::Constant(1)),
                    Box::new(Expression::Constant(0))
                )),
                Box::new(Expression::Constant(0))
            ))]
        ))
    );
}

#[test]
fn test_parse_and_or_precedence_3() {
    let tokens = tokenize("int main() {\n  return 2 == 2 > 0;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![Statement::Return(Expression::BinaryOperator(
                BinaryOperator::Equal,
                Box::new(Expression::Constant(2)),
                Box::new(Expression::BinaryOperator(
                    BinaryOperator::GreaterThan,
                    Box::new(Expression::Constant(2)),
                    Box::new(Expression::Constant(0)),
                ))
            ))]
        ))
    );
}

#[test]
fn test_parse_assign() {
    let tokens = tokenize("int main() {\n  int a;\n  a = 2;\n  return a;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![
                Statement::Declaration("a".to_string(), None),
                Statement::Expression(Expression::Assignment(
                    "a".to_string(),
                    Box::new(Expression::Constant(2))
                )),
                Statement::Return(Expression::VariableReference("a".to_string()))
            ]
        ))
    );
}

#[test]
fn test_parse_assign_val() {
    let tokens = tokenize("int main() {\n  int a;\n  int b = a = 0;\n  return b;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![
                Statement::Declaration("a".to_string(), None),
                Statement::Declaration(
                    "b".to_string(),
                    Some(Expression::Assignment(
                        "a".to_string(),
                        Box::new(Expression::Constant(0))
                    ))
                ),
                Statement::Return(Expression::VariableReference("b".to_string()))
            ]
        ))
    );
}

#[test]
fn test_parse_exp_return_val() {
    let tokens = tokenize("int main() {\n  int a;\n  int b;  a = b = 4;\n  return a - b;\n}\n");
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".to_string(),
            vec![
                Statement::Declaration("a".to_string(), None),
                Statement::Declaration("b".to_string(), None),
                Statement::Expression(Expression::Assignment(
                    "a".to_string(),
                    Box::new(Expression::Assignment(
                        "b".to_string(),
                        Box::new(Expression::Constant(4))
                    ))
                )),
                Statement::Return(Expression::BinaryOperator(
                    BinaryOperator::Subtraction,
                    Box::new(Expression::VariableReference("a".to_string())),
                    Box::new(Expression::VariableReference("b".to_string()))
                )),
            ]
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
