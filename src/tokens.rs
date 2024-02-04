use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub string: String,
    pub token_type: TokenType,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ":[{:?}, {:?}]", self.token_type, self.string)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Type(Types),
    Literal(Literals),
    Keyword(Keywords),
    Operator(Operators),
    Delimiter(Delimiters),
    EoF,
    Identifier,
    Comment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Definition,   // <type> <ident> = <expression>
    Calculation,  // <ident> <operator> <expression>
    FunctionCall, // <functionName>(<parameterList>)
    Literal,      // <a literal>
}

// For parser outputs, a parser can determine if it cannot parse the given input, or if there is an error state in the context of the input.
pub enum ParseResult<T> {
    // The input has been successfully parsed, T is the resulting output
    Parsed(T),
    // The input it unparsable by the parser
    Unparsable,
    // The parser found a malformed string or other erroneous state
    Err,
}

pub fn match_expression(expression: Vec<Token>) -> ParseResult<Expression> {
    // let iter = expression.iter();
    // if matches!(expression.get(0).unwrap().token_type, TokenType::Type(_)) && matches!(expression.get(1).unwrap().token_type, TokenType::Identifier) && matches!(expression.get(2).unwrap().token_type, TokenType::Operator(Operators::Equal)) {
    //             match match_expression(expression.get(3..).unwrap().to_vec()) {
    //                 Err;
    //             }
    // }
    ParseResult::Err
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Primitive(PrimitiveType),
    BuiltIn(BuiltinType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    Primitive(PrimitiveType),
    BuiltIn(BuiltinType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
    Char,
    Atom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinType {
    // in order of importance
    String, // "\" my string of things {interpolated expression} "
    Enum,   // Enum <name> {Thing, AThing, TheThing}
    Tuple,  // {thing, thing, thing}
    List,   // [thing, another, thing ]
    Range,  // 1..5 || -1..-10..-1 <- not sure about the step...
    Map,    // {key: value, name: item}
    Stream, // not entirely sure how the stream works...
}

pub fn tokenize_type(string: &str) -> Result<(&str, TokenType), &str> {
    // Stream,
    if string.starts_with("stream") {
        return Ok((
            "stream",
            TokenType::Type(Types::BuiltIn(BuiltinType::Stream)),
        ));
    }
    // String,
    else if string.starts_with("str") {
        return Ok(("str", TokenType::Type(Types::BuiltIn(BuiltinType::String))));
    }
    // Enum,
    else if string.starts_with("enum") {
        return Ok(("enum", TokenType::Type(Types::BuiltIn(BuiltinType::Enum))));
    }
    // Tuple,
    else if string.starts_with("tuple") {
        return Ok(("tuple", TokenType::Type(Types::BuiltIn(BuiltinType::Tuple))));
    }
    // List,
    else if string.starts_with("list") {
        return Ok(("list", TokenType::Type(Types::BuiltIn(BuiltinType::List))));
    }
    // Range,
    else if string.starts_with("range") {
        return Ok(("range", TokenType::Type(Types::BuiltIn(BuiltinType::Range))));
    }
    // Map,
    else if string.starts_with("map") {
        return Ok(("map", TokenType::Type(Types::BuiltIn(BuiltinType::Map))));
    }
    // Int,
    else if string.starts_with("int") {
        return Ok(("int", TokenType::Type(Types::Primitive(PrimitiveType::Int))));
    }
    // Float,
    else if string.starts_with("float") {
        return Ok((
            "float",
            TokenType::Type(Types::Primitive(PrimitiveType::Float)),
        ));
    }
    // Bool,
    else if string.starts_with("bool") {
        return Ok((
            "bool",
            TokenType::Type(Types::Primitive(PrimitiveType::Bool)),
        ));
    }
    // Char,
    else if string.starts_with("char") {
        return Ok((
            "char",
            TokenType::Type(Types::Primitive(PrimitiveType::Char)),
        ));
    }
    // Atom,
    else if string.starts_with("atom") {
        return Ok((
            "atom",
            TokenType::Type(Types::Primitive(PrimitiveType::Atom)),
        ));
    }
    Err("Err: no matching type keyword")
}

#[test]
fn test_tokenize_type() {
    assert_eq!(
        tokenize_type("str"),
        Ok(("str", TokenType::Type(Types::BuiltIn(BuiltinType::String))))
    );
    assert_eq!(
        tokenize_type("enum"),
        Ok(("enum", TokenType::Type(Types::BuiltIn(BuiltinType::Enum))))
    );
    assert_eq!(
        tokenize_type("tuple"),
        Ok(("tuple", TokenType::Type(Types::BuiltIn(BuiltinType::Tuple))))
    );
    assert_eq!(
        tokenize_type("list"),
        Ok(("list", TokenType::Type(Types::BuiltIn(BuiltinType::List))))
    );
    assert_eq!(
        tokenize_type("range"),
        Ok(("range", TokenType::Type(Types::BuiltIn(BuiltinType::Range))))
    );
    assert_eq!(
        tokenize_type("map"),
        Ok(("map", TokenType::Type(Types::BuiltIn(BuiltinType::Map))))
    );
    assert_eq!(
        tokenize_type("stream"),
        Ok((
            "stream",
            TokenType::Type(Types::BuiltIn(BuiltinType::Stream))
        ))
    );
    assert_eq!(
        tokenize_type("int"),
        Ok(("int", TokenType::Type(Types::Primitive(PrimitiveType::Int))))
    );
    assert_eq!(
        tokenize_type("float"),
        Ok((
            "float",
            TokenType::Type(Types::Primitive(PrimitiveType::Float))
        ))
    );
    assert_eq!(
        tokenize_type("bool"),
        Ok((
            "bool",
            TokenType::Type(Types::Primitive(PrimitiveType::Bool))
        ))
    );
    assert_eq!(
        tokenize_type("char"),
        Ok((
            "char",
            TokenType::Type(Types::Primitive(PrimitiveType::Char))
        ))
    );
    assert_eq!(
        tokenize_type("atom"),
        Ok((
            "atom",
            TokenType::Type(Types::Primitive(PrimitiveType::Atom))
        ))
    );
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Add,         // "+",
    EnumConcat,  // "++",
    Subtract,    // "-",
    Div,         // "/",
    Mult,        // "*",
    Modulo,      // "%",
    Exp,         // "^",
    Concat,      // "<>", //(for strings),
    LessThan,    // "<",
    GreaterThan, // ">",
    LEq,         // "<=",
    GEq,         // ">=",
    BEq,         // "==",
    BNEq,        // "!=",
    Lshift,      // "<<",
    Rshift,      // ">>",
    Into,        // "->"
    Equal,       // "="
    DoubleDot,   // ".." (for ranges)
    Pipe,        // "|>"
}

// pub fn operator_to_string(op: Operators) -> &'static str {
//     match op {
//         Operators::Add => "+",
//         Operators::EnumConcat => "++",
//         Operators::Subtract => "-",
//         Operators::Div => "/",
//         Operators::Mult => "*",
//         Operators::Modulo => "%",
//         Operators::Exp => "^",
//         Operators::Concat => "<>",
//         Operators::LessThan => "<",
//         Operators::GreaterThan => ">",
//         Operators::LEq => "<=",
//         Operators::GEq => ">=",
//         Operators::BEq => "==",
//         Operators::BNEq => "!=",
//         Operators::Lshift => "<<",
//         Operators::Rshift => ">>",
//         Operators::Into => "->",
//         Operators::Equal => "=",
//         Operators::DoubleDot => "..",
//         Operators::Pipe => "|>"
//     }
// }

pub fn tokenize_operator(string: &str) -> Result<(&str, TokenType), &str> {
    match string.chars().take(2).collect::<String>().as_str() {
        "++" => return Ok(("++", TokenType::Operator(Operators::EnumConcat))),
        "<>" => return Ok(("<>", TokenType::Operator(Operators::Concat))),
        "<=" => return Ok(("<=", TokenType::Operator(Operators::LEq))),
        ">=" => return Ok((">=", TokenType::Operator(Operators::GEq))),
        "==" => return Ok(("==", TokenType::Operator(Operators::BEq))),
        "!=" => return Ok(("!=", TokenType::Operator(Operators::BNEq))),
        "<<" => return Ok(("<<", TokenType::Operator(Operators::Lshift))),
        ">>" => return Ok((">>", TokenType::Operator(Operators::Rshift))),
        "->" => return Ok(("->", TokenType::Operator(Operators::Into))),
        ".." => return Ok(("..", TokenType::Operator(Operators::DoubleDot))),
        "|>" => return Ok(("|>", TokenType::Operator(Operators::Pipe))),
        _ => {}
    }
    match string.chars().take(1).collect::<String>().as_str() {
        "+" => return Ok(("+", TokenType::Operator(Operators::Add))),
        "-" => return Ok(("-", TokenType::Operator(Operators::Subtract))),
        "/" => return Ok(("/", TokenType::Operator(Operators::Div))),
        "*" => return Ok(("*", TokenType::Operator(Operators::Mult))),
        "%" => return Ok(("%", TokenType::Operator(Operators::Modulo))),
        "^" => return Ok(("^", TokenType::Operator(Operators::Exp))),
        "<" => return Ok(("<", TokenType::Operator(Operators::LessThan))),
        ">" => return Ok((">", TokenType::Operator(Operators::GreaterThan))),
        "=" => return Ok(("=", TokenType::Operator(Operators::Equal))),
        _ => {}
    };

    Err("Could not match string to an operator")
}

#[test]
fn test_tokenize_operator() {
    assert_eq!(
        tokenize_operator("+"),
        Ok(("+", TokenType::Operator(Operators::Add)))
    );
    assert_eq!(
        tokenize_operator("-"),
        Ok(("-", TokenType::Operator(Operators::Subtract)))
    );
    assert_eq!(
        tokenize_operator("/"),
        Ok(("/", TokenType::Operator(Operators::Div)))
    );
    assert_eq!(
        tokenize_operator("*"),
        Ok(("*", TokenType::Operator(Operators::Mult)))
    );
    assert_eq!(
        tokenize_operator("%"),
        Ok(("%", TokenType::Operator(Operators::Modulo)))
    );
    assert_eq!(
        tokenize_operator("^"),
        Ok(("^", TokenType::Operator(Operators::Exp)))
    );
    assert_eq!(
        tokenize_operator("<"),
        Ok(("<", TokenType::Operator(Operators::LessThan)))
    );
    assert_eq!(
        tokenize_operator(">"),
        Ok((">", TokenType::Operator(Operators::GreaterThan)))
    );
    assert_eq!(
        tokenize_operator("="),
        Ok(("=", TokenType::Operator(Operators::Equal)))
    );
    assert_eq!(
        tokenize_operator("++"),
        Ok(("++", TokenType::Operator(Operators::EnumConcat)))
    );
    assert_eq!(
        tokenize_operator("<>"),
        Ok(("<>", TokenType::Operator(Operators::Concat)))
    );
    assert_eq!(
        tokenize_operator("<="),
        Ok(("<=", TokenType::Operator(Operators::LEq)))
    );
    assert_eq!(
        tokenize_operator(">="),
        Ok((">=", TokenType::Operator(Operators::GEq)))
    );
    assert_eq!(
        tokenize_operator("=="),
        Ok(("==", TokenType::Operator(Operators::BEq)))
    );
    assert_eq!(
        tokenize_operator("!="),
        Ok(("!=", TokenType::Operator(Operators::BNEq)))
    );
    assert_eq!(
        tokenize_operator("<<"),
        Ok(("<<", TokenType::Operator(Operators::Lshift)))
    );
    assert_eq!(
        tokenize_operator(">>"),
        Ok((">>", TokenType::Operator(Operators::Rshift)))
    );
    assert_eq!(
        tokenize_operator("->"),
        Ok(("->", TokenType::Operator(Operators::Into)))
    );
    assert_eq!(
        tokenize_operator(".."),
        Ok(("..", TokenType::Operator(Operators::DoubleDot)))
    );
    assert_eq!(
        tokenize_operator("|>"),
        Ok(("|>", TokenType::Operator(Operators::Pipe)))
    );
}

/*
f/ https://go.dev/ref/spec#Keywords
break        default      func         interface    select
case         defer        go           map          struct
chan         else         goto         package      switch
const        fallthrough  if           range        type
continue     for          import       return       var
*/
#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    Import,    // for importing other rho files and modules
    Include,   // for including C header files
    Struct,    // Defines a struct
    For,       // A for loop initiator
    Continue,  // a loop continue ie br : loop_head
    Break,     // a loop break ie br : loop_end
    Func,      // for defining a function or method
    Fn,        // for defining a lambda / closure
    Match, // for elixir style match cases | https://hexdocs.pm/elixir/1.16.1/case-cond-and-if.html
    Cond,  // for elixir style cond cases
    When, // for defining elixir style function guards | https://hexdocs.pm/elixir/1.16.1/Kernel.html#module-guards
    If,   // a basic c style if
    Elif, // a basic c style else if
    Else, // a basic c style else
    Return, // return
    Interface, // defines an interface / abstract struct with definable methods
    Assert, // allows for runtime assertions / testing
    Panic, // nukes everything and panics!
    Test, // allows for testing in modules, which auto tests on `rho test || rho t`
    EoF,
}

pub fn tokenize_keyword(string: &str) -> Result<(&str, TokenType), &str> {
    if string.starts_with("import") {
        return Ok(("import", TokenType::Keyword(Keywords::Import)));
    } else if string.starts_with("include") {
        return Ok(("include", TokenType::Keyword(Keywords::Include)));
    } else if string.starts_with("struct") {
        return Ok(("struct", TokenType::Keyword(Keywords::Struct)));
    } else if string.starts_with("for") {
        return Ok(("for", TokenType::Keyword(Keywords::For)));
    } else if string.starts_with("continue") {
        return Ok(("continue", TokenType::Keyword(Keywords::Continue)));
    } else if string.starts_with("break") {
        return Ok(("break", TokenType::Keyword(Keywords::Break)));
    } else if string.starts_with("func") {
        return Ok(("func", TokenType::Keyword(Keywords::Func)));
    } else if string.starts_with("fn") {
        return Ok(("fn", TokenType::Keyword(Keywords::Fn)));
    } else if string.starts_with("match") {
        return Ok(("match", TokenType::Keyword(Keywords::Match)));
    } else if string.starts_with("cond") {
        return Ok(("cond", TokenType::Keyword(Keywords::Cond)));
    } else if string.starts_with("when") {
        return Ok(("when", TokenType::Keyword(Keywords::When)));
    } else if string.starts_with("if") {
        return Ok(("if", TokenType::Keyword(Keywords::If)));
    } else if string.starts_with("elif") {
        return Ok(("elif", TokenType::Keyword(Keywords::Elif)));
    } else if string.starts_with("else") {
        return Ok(("else", TokenType::Keyword(Keywords::Else)));
    } else if string.starts_with("return") {
        return Ok(("return", TokenType::Keyword(Keywords::Return)));
    } else if string.starts_with("interface") {
        return Ok(("interface", TokenType::Keyword(Keywords::Interface)));
    } else if string.starts_with("assert") {
        return Ok(("assert", TokenType::Keyword(Keywords::Assert)));
    } else if string.starts_with("panic") {
        return Ok(("panic", TokenType::Keyword(Keywords::Panic)));
    } else if string.starts_with("test") {
        return Ok(("test", TokenType::Keyword(Keywords::Test)));
    } else if string.starts_with("<EOF>") {
        return Ok(("<EOF>", TokenType::Keyword(Keywords::EoF)));
    }
    Err("Err: could not match keyword.")
}

#[test]
fn test_tokenize_keyword() {
    assert_eq!(
        tokenize_keyword("import"),
        Ok(("import", TokenType::Keyword(Keywords::Import)))
    );
    assert_eq!(
        tokenize_keyword("include"),
        Ok(("include", TokenType::Keyword(Keywords::Include)))
    );
    assert_eq!(
        tokenize_keyword("struct"),
        Ok(("struct", TokenType::Keyword(Keywords::Struct)))
    );
    assert_eq!(
        tokenize_keyword("for"),
        Ok(("for", TokenType::Keyword(Keywords::For)))
    );
    assert_eq!(
        tokenize_keyword("continue"),
        Ok(("continue", TokenType::Keyword(Keywords::Continue)))
    );
    assert_eq!(
        tokenize_keyword("break"),
        Ok(("break", TokenType::Keyword(Keywords::Break)))
    );
    assert_eq!(
        tokenize_keyword("func"),
        Ok(("func", TokenType::Keyword(Keywords::Func)))
    );
    assert_eq!(
        tokenize_keyword("fn"),
        Ok(("fn", TokenType::Keyword(Keywords::Fn)))
    );
    assert_eq!(
        tokenize_keyword("match"),
        Ok(("match", TokenType::Keyword(Keywords::Match)))
    );
    assert_eq!(
        tokenize_keyword("cond"),
        Ok(("cond", TokenType::Keyword(Keywords::Cond)))
    );
    assert_eq!(
        tokenize_keyword("when"),
        Ok(("when", TokenType::Keyword(Keywords::When)))
    );
    assert_eq!(
        tokenize_keyword("if"),
        Ok(("if", TokenType::Keyword(Keywords::If)))
    );
    assert_eq!(
        tokenize_keyword("elif"),
        Ok(("elif", TokenType::Keyword(Keywords::Elif)))
    );
    assert_eq!(
        tokenize_keyword("else"),
        Ok(("else", TokenType::Keyword(Keywords::Else)))
    );
    assert_eq!(
        tokenize_keyword("return"),
        Ok(("return", TokenType::Keyword(Keywords::Return)))
    );
    assert_eq!(
        tokenize_keyword("interface"),
        Ok(("interface", TokenType::Keyword(Keywords::Interface)))
    );
    assert_eq!(
        tokenize_keyword("assert"),
        Ok(("assert", TokenType::Keyword(Keywords::Assert)))
    );
    assert_eq!(
        tokenize_keyword("panic"),
        Ok(("panic", TokenType::Keyword(Keywords::Panic)))
    );
    assert_eq!(
        tokenize_keyword("test"),
        Ok(("test", TokenType::Keyword(Keywords::Test)))
    );
    assert_eq!(
        tokenize_keyword("<EOF>"),
        Ok(("<EOF>", TokenType::Keyword(Keywords::EoF)))
    );
}

pub fn tokenize_identifier(string: &str) -> Result<(&str, TokenType), &str> {
    if !string.chars().next().unwrap_or(' ').is_alphabetic() {
        return Err("Err: could not parse identifer | Ident must start with alphabetic");
    }
    let ident = string
        .chars()
        .take_while(|c| c.is_alphanumeric() || c == &'_')
        .collect::<String>()
        .to_owned();
    if !ident.is_empty() {
        let l = ident.len();
        return Ok((string.get(0..l).unwrap(), TokenType::Identifier));
    }
    Err("Err: Could not parse identifier.")
}

pub fn tokenize_string_literal(string: &str) -> Result<(&str, TokenType), &str> {
    if !string.starts_with('\"') || string.len() < 2 {
        return Err("Not a string literal");
    }
    for i in 1..(string.len()) {
        if string.as_bytes()[i] == b'\"' && string.as_bytes()[i - 1] != b'\\' {
            return Ok((
                string.get(0..i + 1).unwrap(),
                TokenType::Literal(Literals::BuiltIn(BuiltinType::String)),
            ));
        }
    }

    // TODO: should panic
    Err("Err: String literal never closed.")
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiters {
    ParOpen,
    ParClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,
    DQuote,
    Quote,
    Comma,
    Semicolon,
}

pub fn tokenize_delimiter(string: &str) -> Result<(&str, TokenType), &str> {
    match string.chars().take(1).collect::<String>().as_str() {
        "(" => return Ok(("(", TokenType::Delimiter(Delimiters::ParOpen))),
        ")" => return Ok((")", TokenType::Delimiter(Delimiters::ParClose))),
        "[" => return Ok(("[", TokenType::Delimiter(Delimiters::BracketOpen))),
        "]" => return Ok(("]", TokenType::Delimiter(Delimiters::BracketClose))),
        "{" => return Ok(("{", TokenType::Delimiter(Delimiters::BraceOpen))),
        "}" => return Ok(("}", TokenType::Delimiter(Delimiters::BraceClose))),
        "\"" => return Ok(("\"", TokenType::Delimiter(Delimiters::DQuote))),
        "'" => return Ok(("'", TokenType::Delimiter(Delimiters::Quote))),
        "," => return Ok((",", TokenType::Delimiter(Delimiters::Comma))),
        ";" => return Ok((";", TokenType::Delimiter(Delimiters::Semicolon))),
        _ => {}
    };
    Err("Err: Could not parse delimiter")
}

#[test]
fn test_tokenize_delim() {
    assert_eq!(
        tokenize_delimiter("("),
        Ok(("(", TokenType::Delimiter(Delimiters::ParOpen)))
    );
    assert_eq!(
        tokenize_delimiter(")"),
        Ok((")", TokenType::Delimiter(Delimiters::ParClose)))
    );
    assert_eq!(
        tokenize_delimiter("["),
        Ok(("[", TokenType::Delimiter(Delimiters::BracketOpen)))
    );
    assert_eq!(
        tokenize_delimiter("]"),
        Ok(("]", TokenType::Delimiter(Delimiters::BracketClose)))
    );
    assert_eq!(
        tokenize_delimiter("{"),
        Ok(("{", TokenType::Delimiter(Delimiters::BraceOpen)))
    );
    assert_eq!(
        tokenize_delimiter("}"),
        Ok(("}", TokenType::Delimiter(Delimiters::BraceClose)))
    );
    assert_eq!(
        tokenize_delimiter("\""),
        Ok(("\"", TokenType::Delimiter(Delimiters::DQuote)))
    );
    assert_eq!(
        tokenize_delimiter("'"),
        Ok(("'", TokenType::Delimiter(Delimiters::Quote)))
    );
    assert_eq!(
        tokenize_delimiter(","),
        Ok((",", TokenType::Delimiter(Delimiters::Comma)))
    );
    assert_eq!(
        tokenize_delimiter(";"),
        Ok((";", TokenType::Delimiter(Delimiters::Semicolon)))
    );
}

fn tokenize_comment(string: &str) -> Result<(&str, TokenType), &str> {
    if string.starts_with("//") {
        let len = string.chars().take_while(|c| c != &'\n').count();
        return Ok((string.get(0..len).unwrap_or(""), TokenType::Comment));
    }
    if string.starts_with("/*") {
        let len = string
            .match_indices("*/")
            .next()
            .unwrap_or((string.len() - 2, ""))
            .0;
        return Ok((string.get(0..len + 2).unwrap_or(""), TokenType::Comment));
    }
    Err("Err not implemented")
}

#[test]
fn test_tokenize_comment() {
    assert_eq!(
        tokenize_comment("// asdflkj\n"),
        Ok(("// asdflkj", TokenType::Comment))
    );
    assert_eq!(tokenize_comment("//\n"), Ok(("//", TokenType::Comment)));
    assert_eq!(tokenize_comment("// \n"), Ok(("// ", TokenType::Comment)));
    assert_eq!(tokenize_comment("//aa\n"), Ok(("//aa", TokenType::Comment)));
    assert_eq!(tokenize_comment("//a"), Ok(("//a", TokenType::Comment)));
    assert_eq!(
        tokenize_comment("/* an inline multiline comment */"),
        Ok(("/* an inline multiline comment */", TokenType::Comment))
    );
    assert_eq!(
        tokenize_comment(
            "/* a multiline 
        * multiline 
        * comment 
        */"
        ),
        Ok((
            "/* a multiline 
        * multiline 
        * comment 
        */",
            TokenType::Comment
        ))
    );
    assert_eq!(
        tokenize_comment("/* an unclosed multiline"),
        Ok(("/* an unclosed multiline", TokenType::Comment))
    );
}

pub fn tokenize_numeric_literal(string: &str) -> Result<(&str, TokenType), &str> {
    let init_string = string.to_owned();
    let is_negative = string.starts_with('-');
    // if is_negative {
    //     // string = string.get(1..).unwrap_or("")
    // }
    let mut n = "".to_owned();
    if is_negative {
        n.insert(0, '-');
    }
    let l = n.len();
    let n = n + string
        .get(l..)
        .unwrap_or("")
        .chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .as_str();
    // string = string.get(n.len()..).unwrap_or("");

    // if is_negative {
    //     n.insert(0, '-');
    // }
    let ros = string.get(n.len()..).unwrap_or("");

    if !n.is_empty() {
        if !ros.is_empty() && ros.as_bytes()[0] == b'.' {
            // either a float literal or an error
            if ros.len() >= 2 {
                if ros.as_bytes()[1] != b'.' {
                    let dec = ros
                        .get(1..)
                        .unwrap_or("")
                        .chars()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>();
                    let token_string = n + "." + &dec;
                    return Ok((
                        string.get(0..token_string.len()).unwrap_or(""),
                        TokenType::Literal(Literals::Primitive(PrimitiveType::Int)),
                    ));
                } else {
                    return Ok((
                        string.get(0..n.len()).unwrap_or(""),
                        TokenType::Literal(Literals::Primitive(PrimitiveType::Int)),
                    ));
                }
            }
        } else {
            return Ok((
                string.get(0..n.len()).unwrap_or(""),
                TokenType::Literal(Literals::Primitive(PrimitiveType::Int)),
            ));
        }
    }
    Err("Err: could not parse numeric")
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Int(i32),
    String(String),
    Expression(Expression),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Expression {
    tokens: Vec<Token>,
}

pub fn tokenize(mut input_string: &str) -> Vec<Token> {
    let v = &mut vec![];
    let mut max_run = 3;
    // if string.len() > 1 {
    //     let mut ros = tokenize(string.get(1..).unwrap());
    //     v.append(&mut ros);
    // }
    // let mut input: &str = input_string;
    #[allow(clippy::type_complexity)]
    let tokenizers: Vec<fn(&str) -> Result<(&str, TokenType), &str>> = vec![
        // parse_keyword,
        tokenize_comment,
        tokenize_string_literal,
        tokenize_numeric_literal,
        tokenize_keyword,
        tokenize_type,
        tokenize_operator,
        tokenize_delimiter,
        tokenize_identifier,
    ];

    println!("tokenize instr: {:?}", input_string);
    'tokenLoop: while !input_string.is_empty() && max_run > 0 {
        input_string = input_string.trim_start();
        for tokenizer in tokenizers.iter() {
            match tokenizer(input_string) {
                Ok((s, t_type)) => {
                    v.push(Token {
                        string: s.to_string(),
                        token_type: t_type,
                    });
                    input_string = input_string.get(s.len()..).unwrap_or("");
                    continue 'tokenLoop;
                }
                Err(s) => {}
            }
        }

        println!("instr: {:?} | max_run: {}", input_string, max_run);
        max_run -= 1;
    }

    v.to_vec()
}
