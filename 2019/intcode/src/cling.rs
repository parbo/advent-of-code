use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    Int,
    If,
    Else,
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
    Colon,
    Minus,
    Bang,
    QuestionMark,
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
    Identifier(String),
    Keyword(Keyword),
    Integer(i128),
    Str(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Location {
    pub file: String,
    pub start: Position,
    pub end: Position,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TokenWithLocation(Token, Location);

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

fn colon_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == ':' {
        Some(TokenizeResult(Token::Colon, 1, true))
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

fn question_mark_tokenizer(a: &str) -> Option<TokenizeResult> {
    if a.chars().next()? == '?' {
        Some(TokenizeResult(Token::QuestionMark, 1, true))
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
        "else" => Some(Keyword::Else),
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

pub fn tokenize(text: &str) -> Vec<TokenWithLocation> {
    let tokenizers: [fn(&str) -> Option<TokenizeResult>; 29] = [
        whitespace_tokenizer,
        block_comment_tokenizer,
        comment_tokenizer,
        open_paren_tokenizer,
        close_paren_tokenizer,
        open_brace_tokenizer,
        close_brace_tokenizer,
        semi_colon_tokenizer,
        colon_tokenizer,
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
        question_mark_tokenizer,
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

    let mut tokenlist = Vec::<TokenWithLocation>::new();
    let mut pos: usize = 0;
    let mut row: usize = 0;
    let mut col: usize = 0;
    loop {
        let last_pos = pos;
        let last_row = row;
        let last_col = col;
        for &tokenizer in tokenizers.iter() {
            if let Some(TokenizeResult(token, consumed, emit)) = tokenizer(&text[pos..]) {
                let mut last_c = ' ';
                for c in text[pos..(pos + consumed)].chars() {
                    match c {
                        '\r' | '\n' => {
                            if last_c != '\r' && last_c != '\n' {
                                row += 1;
                                col = 0;
                            }
                        }
                        _ => col += 1,
                    }
                    last_c = c;
                }
                if emit {
                    let location = Location {
                        file: "<none>".to_string(),
                        start: Position {
                            line: last_row,
                            column: last_col,
                        },
                        end: Position {
                            line: row,
                            column: col,
                        },
                    };
                    tokenlist.push(TokenWithLocation(token, location));
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
pub enum Declaration {
    Variable(String, Option<Expression>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    Conditional(Expression, Box<Statement>, Option<Box<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BlockItem {
    Statement(Statement),
    Declaration(Declaration),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    Function(String, Vec<BlockItem>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Program {
    Program(Function),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    SyntaxError(String, Location),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedEOF => write!(f, "Unexpected EOF"),
            ParseError::SyntaxError(s, location) => write!(
                f,
                "SyntaxError: {} --> {}:{}:{}",
                s, location.file, location.start.line, location.start.column
            ),
        }
    }
}

fn parse_factor(tokens: &[TokenWithLocation]) -> Result<(Expression, usize), ParseError> {
    let mut it = tokens.iter();
    match it.next() {
        Some(TokenWithLocation(Token::OpenParen, loc)) => {
            if let Ok((exp, offset)) = parse_expression(&tokens[1..]) {
                if let Some(TokenWithLocation(Token::CloseParen, _)) =
                    tokens[(1 + offset)..].iter().next()
                {
                    Ok((exp, 1 + offset + 1))
                } else {
                    Err(ParseError::SyntaxError(
                        "Expected )".into(),
                        tokens[1 + offset].1.clone(),
                    ))
                }
            } else {
                Err(ParseError::SyntaxError(
                    "Expected expression after (".into(),
                    loc.clone(),
                ))
            }
        }
        Some(TokenWithLocation(Token::Minus, loc)) => {
            if let Ok((exp, offset)) = parse_factor(&tokens[1..]) {
                Ok((
                    Expression::UnaryOperator(UnaryOperator::Negation, Box::new(exp)),
                    1 + offset,
                ))
            } else {
                Err(ParseError::SyntaxError(
                    "Expected expression after -".into(),
                    loc.clone(),
                ))
            }
        }
        Some(TokenWithLocation(Token::Bang, loc)) => {
            if let Ok((exp, offset)) = parse_factor(&tokens[1..]) {
                Ok((
                    Expression::UnaryOperator(UnaryOperator::LogicalNegation, Box::new(exp)),
                    1 + offset,
                ))
            } else {
                Err(ParseError::SyntaxError(
                    "Expected expression after !".into(),
                    loc.clone(),
                ))
            }
        }
        Some(TokenWithLocation(Token::Integer(x), _)) => Ok((Expression::Constant(*x), 1)),
        Some(TokenWithLocation(Token::Identifier(name), _)) => {
            Ok((Expression::VariableReference(name.clone()), 1))
        }
        _ => Err(ParseError::SyntaxError(
            "Expected factor".into(),
            tokens[0].1.clone(),
        )),
    }
}

fn parse_term(tokens: &[TokenWithLocation]) -> Result<(Expression, usize), ParseError> {
    match parse_factor(tokens) {
        Err(e) => Err(e),
        Ok((f, o)) => {
            let mut factor = f;
            let mut offset = o;
            loop {
                let mut it = tokens[offset..].iter();
                match it.next() {
                    Some(TokenWithLocation(Token::Asterisk, _)) => {
                        if let Ok((next_factor, next_offset)) =
                            parse_factor(&tokens[(1 + offset)..])
                        {
                            factor = Expression::BinaryOperator(
                                BinaryOperator::Multiplication,
                                Box::new(factor),
                                Box::new(next_factor),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected factor after *".into(),
                                tokens[1 + offset].1.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::Slash, _)) => {
                        if let Ok((next_factor, next_offset)) =
                            parse_factor(&tokens[(1 + offset)..])
                        {
                            factor = Expression::BinaryOperator(
                                BinaryOperator::Division,
                                Box::new(factor),
                                Box::new(next_factor),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected factor after /".into(),
                                tokens[1 + offset].1.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::Percent, _)) => {
                        if let Ok((next_factor, next_offset)) =
                            parse_factor(&tokens[(1 + offset)..])
                        {
                            factor = Expression::BinaryOperator(
                                BinaryOperator::Modulo,
                                Box::new(factor),
                                Box::new(next_factor),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected factor after %".into(),
                                tokens[1 + offset].1.clone(),
                            ));
                        }
                    }
                    _ => break,
                }
            }
            Ok((factor, offset))
        }
    }
}

fn parse_additive_expression(
    tokens: &[TokenWithLocation],
) -> Result<(Expression, usize), ParseError> {
    match parse_term(tokens) {
        Err(e) => Err(e),
        Ok((t, o)) => {
            let mut term = t;
            let mut offset = o;
            loop {
                let mut it = tokens[offset..].iter();
                match it.next() {
                    Some(TokenWithLocation(Token::Plus, loc)) => {
                        if let Ok((next_term, next_offset)) = parse_term(&tokens[(1 + offset)..]) {
                            term = Expression::BinaryOperator(
                                BinaryOperator::Addition,
                                Box::new(term),
                                Box::new(next_term),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected term".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::Minus, loc)) => {
                        if let Ok((next_term, next_offset)) = parse_term(&tokens[(1 + offset)..]) {
                            term = Expression::BinaryOperator(
                                BinaryOperator::Subtraction,
                                Box::new(term),
                                Box::new(next_term),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected term".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    _ => break,
                }
            }
            Ok((term, offset))
        }
    }
}

fn parse_relational_expression(
    tokens: &[TokenWithLocation],
) -> Result<(Expression, usize), ParseError> {
    match parse_additive_expression(tokens) {
        Err(e) => Err(e),
        Ok((ae, o)) => {
            let mut additive_expression = ae;
            let mut offset = o;
            loop {
                let mut it = tokens[offset..].iter();
                match it.next() {
                    Some(TokenWithLocation(Token::LessThan, loc)) => {
                        if let Ok((next_additive_expression, next_offset)) =
                            parse_additive_expression(&tokens[(1 + offset)..])
                        {
                            additive_expression = Expression::BinaryOperator(
                                BinaryOperator::LessThan,
                                Box::new(additive_expression),
                                Box::new(next_additive_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after <".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::LessThanOrEqual, loc)) => {
                        if let Ok((next_additive_expression, next_offset)) =
                            parse_additive_expression(&tokens[(1 + offset)..])
                        {
                            additive_expression = Expression::BinaryOperator(
                                BinaryOperator::LessThanOrEqual,
                                Box::new(additive_expression),
                                Box::new(next_additive_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after <=".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::GreaterThan, loc)) => {
                        if let Ok((next_additive_expression, next_offset)) =
                            parse_additive_expression(&tokens[(1 + offset)..])
                        {
                            additive_expression = Expression::BinaryOperator(
                                BinaryOperator::GreaterThan,
                                Box::new(additive_expression),
                                Box::new(next_additive_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after >".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::GreaterThanOrEqual, loc)) => {
                        if let Ok((next_additive_expression, next_offset)) =
                            parse_additive_expression(&tokens[(1 + offset)..])
                        {
                            additive_expression = Expression::BinaryOperator(
                                BinaryOperator::GreaterThanOrEqual,
                                Box::new(additive_expression),
                                Box::new(next_additive_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after >=".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    _ => break,
                }
            }
            Ok((additive_expression, offset))
        }
    }
}

fn parse_equality_expression(
    tokens: &[TokenWithLocation],
) -> Result<(Expression, usize), ParseError> {
    match parse_relational_expression(tokens) {
        Err(e) => Err(e),
        Ok((re, o)) => {
            let mut relational_expression = re;
            let mut offset = o;
            loop {
                let mut it = tokens[offset..].iter();
                match it.next() {
                    Some(TokenWithLocation(Token::Equal, loc)) => {
                        if let Ok((next_relational_expression, next_offset)) =
                            parse_relational_expression(&tokens[(1 + offset)..])
                        {
                            relational_expression = Expression::BinaryOperator(
                                BinaryOperator::Equal,
                                Box::new(relational_expression),
                                Box::new(next_relational_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after ==".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    Some(TokenWithLocation(Token::NotEqual, loc)) => {
                        if let Ok((next_relational_expression, next_offset)) =
                            parse_relational_expression(&tokens[(1 + offset)..])
                        {
                            relational_expression = Expression::BinaryOperator(
                                BinaryOperator::NotEqual,
                                Box::new(relational_expression),
                                Box::new(next_relational_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after !=".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    _ => break,
                }
            }
            Ok((relational_expression, offset))
        }
    }
}

fn parse_logical_and_expression(
    tokens: &[TokenWithLocation],
) -> Result<(Expression, usize), ParseError> {
    match parse_equality_expression(tokens) {
        Err(e) => Err(e),
        Ok((ee, o)) => {
            let mut equality_expression = ee;
            let mut offset = o;
            loop {
                let mut it = tokens[offset..].iter();
                match it.next() {
                    Some(TokenWithLocation(Token::And, loc)) => {
                        if let Ok((next_equality_expression, next_offset)) =
                            parse_equality_expression(&tokens[(1 + offset)..])
                        {
                            equality_expression = Expression::BinaryOperator(
                                BinaryOperator::And,
                                Box::new(equality_expression),
                                Box::new(next_equality_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after &&".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    _ => break,
                }
            }
            Ok((equality_expression, offset))
        }
    }
}

fn parse_logical_or_expression(
    tokens: &[TokenWithLocation],
) -> Result<(Expression, usize), ParseError> {
    match parse_logical_and_expression(tokens) {
        Err(e) => Err(e),
        Ok((lae, o)) => {
            let mut logical_and_expression = lae;
            let mut offset = o;
            loop {
                let mut it = tokens[offset..].iter();
                match it.next() {
                    Some(TokenWithLocation(Token::Or, loc)) => {
                        if let Ok((next_logical_and_expression, next_offset)) =
                            parse_logical_and_expression(&tokens[(1 + offset)..])
                        {
                            logical_and_expression = Expression::BinaryOperator(
                                BinaryOperator::Or,
                                Box::new(logical_and_expression),
                                Box::new(next_logical_and_expression),
                            );
                            offset = offset + 1 + next_offset;
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected expression after ||".into(),
                                loc.clone(),
                            ));
                        }
                    }
                    _ => break,
                }
            }
            Ok((logical_and_expression, offset))
        }
    }
}

fn parse_expression(tokens: &[TokenWithLocation]) -> Result<(Expression, usize), ParseError> {
    let mut offset = 0;
    if let Some(TokenWithLocation(Token::Identifier(name), _)) = tokens[offset..].iter().next() {
        offset = offset + 1;
        if let Some(TokenWithLocation(Token::Assign, loc)) = tokens[offset..].iter().next() {
            offset = offset + 1;
            if let Ok((exp, new_offset)) = parse_expression(&tokens[offset..]) {
                return Ok((
                    Expression::Assignment(name.clone(), Box::new(exp)),
                    offset + new_offset,
                ));
            } else {
                return Err(ParseError::SyntaxError(
                    "Expected expression after =".into(),
                    loc.clone(),
                ));
            }
        }
    }
    parse_logical_or_expression(tokens)
}

fn parse_statement(tokens: &[TokenWithLocation]) -> Result<(Statement, usize), ParseError> {
    let mut it = tokens.iter();
    match it.next() {
        Some(TokenWithLocation(Token::Keyword(Keyword::Return), loc)) => {
            if let Ok((exp, offset)) = parse_expression(&tokens[1..]) {
                if let TokenWithLocation(Token::SemiColon, _) = tokens[(1 + offset)..]
                    .iter()
                    .next()
                    .ok_or_else(|| ParseError::UnexpectedEOF)?
                {
                    return Ok((Statement::Return(exp), 1 + offset + 1));
                } else {
                    return Err(ParseError::SyntaxError(
                        "Expected ; at end of return statement".into(),
                        tokens[offset].1.clone(),
                    ));
                }
            } else {
                return Err(ParseError::SyntaxError(
                    "Expected expression after return".into(),
                    loc.clone(),
                ));
            }
        }
        Some(TokenWithLocation(Token::Keyword(Keyword::If), loc)) => {
            let mut offset = 1;
            if let TokenWithLocation(Token::OpenParen, _) = tokens[offset..]
                .iter()
                .next()
                .ok_or_else(|| ParseError::UnexpectedEOF)?
            {
                offset += 1;
                if let Ok((exp, new_offset)) = parse_expression(&tokens[offset..]) {
                    offset += new_offset;
                    if let TokenWithLocation(Token::CloseParen, _) = tokens[offset..]
                        .iter()
                        .next()
                        .ok_or_else(|| ParseError::UnexpectedEOF)?
                    {
                        offset += 1;
                        if let Ok((statement, new_offset)) = parse_statement(&tokens[offset..]) {
                            offset += new_offset;
                            let mut opt_else_statement = None;
                            if let Some(TokenWithLocation(Token::Keyword(Keyword::Else), _)) =
                                tokens[offset..].iter().next()
                            {
                                offset += 1;
                                if let Ok((statement, new_offset)) =
                                    parse_statement(&tokens[offset..])
                                {
                                    offset += new_offset;
                                    opt_else_statement = Some(Box::new(statement));
                                } else {
                                    return Err(ParseError::SyntaxError(
                                        "Expected statement after 'else'".into(),
                                        tokens[offset].1.clone(),
                                    ));
                                }
                            }
                            return Ok((
                                Statement::Conditional(
                                    exp,
                                    Box::new(statement),
                                    opt_else_statement,
                                ),
                                offset,
                            ));
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Expected statement after 'if (expression)'".into(),
                                tokens[offset].1.clone(),
                            ));
                        }
                    } else {
                        return Err(ParseError::SyntaxError(
                            "Expected ) after 'if (expression'".into(),
                            tokens[offset].1.clone(),
                        ));
                    }
                } else {
                    return Err(ParseError::SyntaxError(
                        "Expected expression after 'if ('".into(),
                        tokens[offset].1.clone(),
                    ));
                }
            } else {
                return Err(ParseError::SyntaxError(
                    "Expected ( after if".into(),
                    loc.clone(),
                ));
            }
        }
        _ => {
            if let Ok((exp, new_offset)) = parse_expression(&tokens) {
                if let TokenWithLocation(Token::SemiColon, _) =
                    tokens[new_offset..]
                        .iter()
                        .next()
                        .ok_or_else(|| ParseError::UnexpectedEOF)?
                {
                    return Ok((Statement::Expression(exp), new_offset + 1));
                } else {
                    return Err(ParseError::SyntaxError(
                        "Expected ; at end of expression statement".into(),
                        tokens[new_offset].1.clone(),
                    ));
                }
            } else {
                return Err(ParseError::SyntaxError(
                    "Expected expression".into(),
                    tokens[0].1.clone(),
                ));
            }
        }
    }
}

fn parse_declaration(tokens: &[TokenWithLocation]) -> Result<(Declaration, usize), ParseError> {
    let mut it = tokens.iter();
    match it.next() {
        Some(TokenWithLocation(Token::Keyword(Keyword::Int), _)) => {
            let mut offset = 1;
            let mut opt_assign = None;
            if let TokenWithLocation(Token::Identifier(name), _) = tokens[offset..]
                .iter()
                .next()
                .ok_or_else(|| ParseError::UnexpectedEOF)?
            {
                offset = offset + 1;
                if let Some(TokenWithLocation(Token::Assign, loc)) = tokens[offset..].iter().next()
                {
                    offset = offset + 1;
                    if let Ok((exp, new_offset)) = parse_expression(&tokens[offset..]) {
                        offset = offset + new_offset;
                        opt_assign = Some(exp);
                    } else {
                        return Err(ParseError::SyntaxError(
                            "Expected expression after =".into(),
                            loc.clone(),
                        ));
                    }
                } else {
                    // assignment is optional.
                }
                if let TokenWithLocation(Token::SemiColon, _) = tokens[offset..]
                    .iter()
                    .next()
                    .ok_or_else(|| ParseError::UnexpectedEOF)?
                {
                    return Ok((Declaration::Variable(name.clone(), opt_assign), offset + 1));
                } else {
                    return Err(ParseError::SyntaxError(
                        "Expected ; after declaration".into(),
                        tokens[offset - 1].1.clone(),
                    ));
                }
            }
        }
        _ => {}
    }
    return Err(ParseError::SyntaxError(
        "Expected declaration".into(),
        tokens[0].1.clone(),
    ));
}

fn parse_blockitem(tokens: &[TokenWithLocation]) -> Result<(BlockItem, usize), ParseError> {
    let res = parse_statement(tokens);
    match res {
        Err(e) => {
            if let Ok((d, offset)) = parse_declaration(tokens) {
                Ok((BlockItem::Declaration(d), offset))
            } else {
                Err(e)
            }
        }
        Ok((s, o)) => Ok((BlockItem::Statement(s), o)),
    }
}

fn parse_function(tokens: &[TokenWithLocation]) -> Result<(Function, usize), ParseError> {
    let mut it = tokens.iter();
    if let TokenWithLocation(Token::Keyword(Keyword::Int), _) =
        it.next().ok_or_else(|| ParseError::UnexpectedEOF)?
    {
        if let TokenWithLocation(Token::Identifier(name), _) =
            it.next().ok_or_else(|| ParseError::UnexpectedEOF)?
        {
            if let TokenWithLocation(Token::OpenParen, loc) =
                it.next().ok_or_else(|| ParseError::UnexpectedEOF)?
            {
                if let TokenWithLocation(Token::CloseParen, loc) =
                    it.next().ok_or_else(|| ParseError::UnexpectedEOF)?
                {
                    if let TokenWithLocation(Token::OpenBrace, _) =
                        it.next().ok_or_else(|| ParseError::UnexpectedEOF)?
                    {
                        let mut blockitems = vec![];
                        let mut offset = 5;
                        let err = loop {
                            let res = parse_blockitem(&tokens[offset..]);
                            match res {
                                Ok((blockitem, new_offset)) => {
                                    blockitems.push(blockitem);
                                    offset = offset + new_offset;
                                }
                                Err(e) => {
                                    break e;
                                }
                            }
                        };
                        if let TokenWithLocation(Token::CloseBrace, _) = tokens[offset..]
                            .iter()
                            .next()
                            .ok_or_else(|| ParseError::UnexpectedEOF)?
                        {
                            return Ok((Function::Function(name.clone(), blockitems), offset + 1));
                        } else {
                            return Err(err);
                        }
                    } else {
                        return Err(ParseError::SyntaxError("Expected {".into(), loc.clone()));
                    }
                } else {
                    return Err(ParseError::SyntaxError("Expected )".into(), loc.clone()));
                }
            }
        }
    }
    Err(ParseError::SyntaxError(
        "Expected function".into(),
        tokens[0].1.clone(),
    ))
}

fn parse_program(tokens: &[TokenWithLocation]) -> Result<(Program, usize), ParseError> {
    match parse_function(tokens) {
        Ok((function, offset)) => Ok((Program::Program(function), offset)),
        Err(e) => Err(e),
    }
}

pub fn parse(a: &[TokenWithLocation]) -> Result<Program, ParseError> {
    match parse_program(a) {
        Ok((program, i)) => {
            if i != a.len() {
                return Err(ParseError::SyntaxError("Error".to_string(), a[i].1.clone()));
            }
            Ok(program)
        }
        Err(e) => Err(e),
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

#[cfg(test)]
fn to_tokens(text: &str) -> Vec<Token> {
    let tokens_with_location = tokenize(text);
    tokens_with_location.iter().map(|x| x.0.clone()).collect()
}

#[test]
fn test_tokenizer() {
    assert_eq!(to_tokens("1 // apa"), [Token::Integer(1)]);
    assert_eq!(
        to_tokens("1 // apa\n2"),
        [Token::Integer(1), Token::Integer(2)]
    );
    assert_eq!(to_tokens("1"), [Token::Integer(1)]);
    assert_eq!(to_tokens("-1"), [Token::Minus, Token::Integer(1)]);
    assert_eq!(to_tokens("!1"), [Token::Bang, Token::Integer(1)]);
    assert_eq!(
        to_tokens("1+2"),
        [Token::Integer(1), Token::Plus, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1*2"),
        [Token::Integer(1), Token::Asterisk, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1/2"),
        [Token::Integer(1), Token::Slash, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1%2"),
        [Token::Integer(1), Token::Percent, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1==2"),
        [Token::Integer(1), Token::Equal, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1!=2"),
        [Token::Integer(1), Token::NotEqual, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1<2"),
        [Token::Integer(1), Token::LessThan, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1<=2"),
        [Token::Integer(1), Token::LessThanOrEqual, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1>2"),
        [Token::Integer(1), Token::GreaterThan, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1>=2"),
        [
            Token::Integer(1),
            Token::GreaterThanOrEqual,
            Token::Integer(2)
        ]
    );
    assert_eq!(
        to_tokens("1&&2"),
        [Token::Integer(1), Token::And, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1||2"),
        [Token::Integer(1), Token::Or, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("1=2"),
        [Token::Integer(1), Token::Assign, Token::Integer(2)]
    );
    assert_eq!(
        to_tokens("3-1"),
        [Token::Integer(3), Token::Minus, Token::Integer(1)]
    );
    assert_eq!(to_tokens("123"), [Token::Integer(123)]);
    assert_eq!(to_tokens("1 2"), [Token::Integer(1), Token::Integer(2)]);
    assert_eq!(
        to_tokens("123 321"),
        [Token::Integer(123), Token::Integer(321)]
    );
    assert_eq!(to_tokens("\"test\""), [Token::Str(String::from("test"))]);
    assert_eq!(to_tokens("x"), [Token::Identifier(String::from("x"))]);
    assert_eq!(to_tokens("_x"), [Token::Identifier(String::from("_x"))]);
    assert_eq!(to_tokens("y_2"), [Token::Identifier(String::from("y_2"))]);
    assert_eq!(to_tokens("int"), [Token::Keyword(Keyword::Int)]);
    assert_eq!(
        to_tokens("-x"),
        [Token::Minus, Token::Identifier(String::from("x"))]
    );
    assert_eq!(
        to_tokens("!x"),
        [Token::Bang, Token::Identifier(String::from("x"))]
    );
    assert_eq!(
        to_tokens("intblaj"),
        [Token::Identifier(String::from("intblaj"))]
    );
    assert_eq!(
        to_tokens("(1 2)"),
        [
            Token::OpenParen,
            Token::Integer(1),
            Token::Integer(2),
            Token::CloseParen
        ]
    );
    assert_eq!(
        to_tokens("int main() {\n  return 2;\n}\n"),
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
    assert_eq!(
        to_tokens("1 ? 2 : 3"),
        [
            Token::Integer(1),
            Token::QuestionMark,
            Token::Integer(2),
            Token::Colon,
            Token::Integer(3)
        ]
    );
    assert_eq!(
        to_tokens("if (1) { return iff; } else { return elsevier; }"),
        [
            Token::Keyword(Keyword::If),
            Token::OpenParen,
            Token::Integer(1),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Identifier("iff".into()),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Keyword(Keyword::Else),
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Identifier("elsevier".into()),
            Token::SemiColon,
            Token::CloseBrace,
        ]
    );
}

#[test]
fn test_tokenizer_location() {
    let tokens = tokenize("int main() {\n  return 2;\n}\n");
    assert_eq!(
        tokens,
        [
            TokenWithLocation(
                Token::Keyword(Keyword::Int),
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 3 }
                }
            ),
            TokenWithLocation(
                Token::Identifier("main".to_string()),
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 0, column: 4 },
                    end: Position { line: 0, column: 8 }
                }
            ),
            TokenWithLocation(
                Token::OpenParen,
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 0, column: 8 },
                    end: Position { line: 0, column: 9 }
                }
            ),
            TokenWithLocation(
                Token::CloseParen,
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 0, column: 9 },
                    end: Position {
                        line: 0,
                        column: 10
                    }
                }
            ),
            TokenWithLocation(
                Token::OpenBrace,
                Location {
                    file: "<none>".to_string(),
                    start: Position {
                        line: 0,
                        column: 11
                    },
                    end: Position {
                        line: 0,
                        column: 12
                    }
                }
            ),
            TokenWithLocation(
                Token::Keyword(Keyword::Return),
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 1, column: 2 },
                    end: Position { line: 1, column: 8 }
                }
            ),
            TokenWithLocation(
                Token::Integer(2),
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 1, column: 9 },
                    end: Position {
                        line: 1,
                        column: 10
                    }
                }
            ),
            TokenWithLocation(
                Token::SemiColon,
                Location {
                    file: "<none>".to_string(),
                    start: Position {
                        line: 1,
                        column: 10
                    },
                    end: Position {
                        line: 1,
                        column: 11
                    }
                }
            ),
            TokenWithLocation(
                Token::CloseBrace,
                Location {
                    file: "<none>".to_string(),
                    start: Position { line: 2, column: 0 },
                    end: Position { line: 2, column: 1 }
                }
            )
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::Constant(2)
            ))]
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::BinaryOperator(
                    BinaryOperator::Subtraction,
                    Box::new(Expression::BinaryOperator(
                        BinaryOperator::Subtraction,
                        Box::new(Expression::Constant(1)),
                        Box::new(Expression::Constant(2))
                    )),
                    Box::new(Expression::Constant(3))
                )
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::BinaryOperator(
                    BinaryOperator::Division,
                    Box::new(Expression::BinaryOperator(
                        BinaryOperator::Division,
                        Box::new(Expression::Constant(6)),
                        Box::new(Expression::Constant(3))
                    )),
                    Box::new(Expression::Constant(2))
                )
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::BinaryOperator(
                    BinaryOperator::Addition,
                    Box::new(Expression::Constant(2)),
                    Box::new(Expression::BinaryOperator(
                        BinaryOperator::Multiplication,
                        Box::new(Expression::Constant(3)),
                        Box::new(Expression::Constant(4))
                    ))
                )
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::BinaryOperator(
                    BinaryOperator::Or,
                    Box::new(Expression::Constant(1)),
                    Box::new(Expression::BinaryOperator(
                        BinaryOperator::And,
                        Box::new(Expression::Constant(0)),
                        Box::new(Expression::Constant(2))
                    ))
                )
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::BinaryOperator(
                    BinaryOperator::And,
                    Box::new(Expression::BinaryOperator(
                        BinaryOperator::Or,
                        Box::new(Expression::Constant(1)),
                        Box::new(Expression::Constant(0))
                    )),
                    Box::new(Expression::Constant(0))
                )
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
            vec![BlockItem::Statement(Statement::Return(
                Expression::BinaryOperator(
                    BinaryOperator::Equal,
                    Box::new(Expression::Constant(2)),
                    Box::new(Expression::BinaryOperator(
                        BinaryOperator::GreaterThan,
                        Box::new(Expression::Constant(2)),
                        Box::new(Expression::Constant(0)),
                    ))
                )
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
                BlockItem::Declaration(Declaration::Variable("a".to_string(), None)),
                BlockItem::Statement(Statement::Expression(Expression::Assignment(
                    "a".to_string(),
                    Box::new(Expression::Constant(2))
                ))),
                BlockItem::Statement(Statement::Return(Expression::VariableReference(
                    "a".to_string()
                )))
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
                BlockItem::Declaration(Declaration::Variable("a".to_string(), None)),
                BlockItem::Declaration(Declaration::Variable(
                    "b".to_string(),
                    Some(Expression::Assignment(
                        "a".to_string(),
                        Box::new(Expression::Constant(0))
                    ))
                )),
                BlockItem::Statement(Statement::Return(Expression::VariableReference(
                    "b".to_string()
                )))
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
                BlockItem::Declaration(Declaration::Variable("a".to_string(), None)),
                BlockItem::Declaration(Declaration::Variable("b".to_string(), None)),
                BlockItem::Statement(Statement::Expression(Expression::Assignment(
                    "a".to_string(),
                    Box::new(Expression::Assignment(
                        "b".to_string(),
                        Box::new(Expression::Constant(4))
                    ))
                ))),
                BlockItem::Statement(Statement::Return(Expression::BinaryOperator(
                    BinaryOperator::Subtraction,
                    Box::new(Expression::VariableReference("a".to_string())),
                    Box::new(Expression::VariableReference("b".to_string()))
                ))),
            ]
        ))
    );
}

#[test]
fn test_parse_syntax_error() {
    let text = "int main() {\n  int a;\n  inte b;  a = b = 4;\n  return a - b;\n}\n";
    let tokens = tokenize(text);
    let res = parse(&tokens);
    if let Err(e) = res {
        let s = format!("{}", e);
        assert_eq!(
            s,
            "SyntaxError: Expected ; at end of expression statement --> <none>:2:7"
        );
    } else {
        panic!();
    }
}

#[test]
fn test_parse_syntax_error_2() {
    let text = "int main() {\n  int a;\n  a *= 4;\n  return a - b;\n}\n";
    let tokens = tokenize(text);
    let res = parse(&tokens);
    if let Err(e) = res {
        let s = format!("{}", e);
        assert_eq!(s, "SyntaxError: Expected expression --> <none>:2:2");
    } else {
        panic!();
    }
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
fn test_parse_multiple_ifs() {
    let tokens = tokenize(
        "\
int main() {
    int a = 0;
    int b = 0;

    if (a)
        a = 2;
    else
        a = 3;

    if (b)
        b = 4;
    else
        b = 5;

    return a + b;
}
",
    );
    assert_eq!(
        parse(&tokens).expect("error"),
        Program::Program(Function::Function(
            "main".into(),
            vec![
                BlockItem::Declaration(Declaration::Variable(
                    "a".into(),
                    Some(Expression::Constant(0))
                )),
                BlockItem::Declaration(Declaration::Variable(
                    "b".into(),
                    Some(Expression::Constant(0))
                )),
                BlockItem::Statement(Statement::Conditional(
                    Expression::VariableReference("a".into()),
                    Box::new(Statement::Expression(Expression::Assignment(
                        "a".into(),
                        Box::new(Expression::Constant(2))
                    ))),
                    Some(Box::new(Statement::Expression(Expression::Assignment(
                        "a".into(),
                        Box::new(Expression::Constant(3))
                    ))))
                )),
                BlockItem::Statement(Statement::Conditional(
                    Expression::VariableReference("b".into()),
                    Box::new(Statement::Expression(Expression::Assignment(
                        "b".into(),
                        Box::new(Expression::Constant(4))
                    ))),
                    Some(Box::new(Statement::Expression(Expression::Assignment(
                        "b".into(),
                        Box::new(Expression::Constant(5))
                    ))))
                )),
                BlockItem::Statement(Statement::Return(Expression::BinaryOperator(
                    BinaryOperator::Addition,
                    Box::new(Expression::VariableReference("a".into())),
                    Box::new(Expression::VariableReference("b".into()))
                )))
            ]
        ))
    )
}
