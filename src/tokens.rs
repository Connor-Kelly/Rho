use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub string: String,
    pub token_type: TokenType,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T[{} {:?}]", self.string, self.token_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Type(Types),
    Keyword(Keywords),
    Operator(Operators),
    EoF,
    Identifier,
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

pub fn string_to_type(string: &str) -> Result<(&str, Types), &str> {
    // Stream,
    if string.starts_with("stream") {
        return Ok(("stream", Types::BuiltIn(BuiltinType::Stream)));
    }
    // String,
    else if string.starts_with("str") {
        return Ok(("str", Types::BuiltIn(BuiltinType::String)));
    }
    // Enum,
    else if string.starts_with("enum") {
        return Ok(("enum", Types::BuiltIn(BuiltinType::Enum)));
    }
    // Tuple,
    else if string.starts_with("tuple") {
        return Ok(("tuple", Types::BuiltIn(BuiltinType::Tuple)));
    }
    // List,
    else if string.starts_with("list") {
        return Ok(("list", Types::BuiltIn(BuiltinType::List)));
    }
    // Range,
    else if string.starts_with("range") {
        return Ok(("range", Types::BuiltIn(BuiltinType::Range)));
    }
    // Map,
    else if string.starts_with("map") {
        return Ok(("map", Types::BuiltIn(BuiltinType::Map)));
    }
    // Int,
    else if string.starts_with("int") {
        return Ok(("int", Types::Primitive(PrimitiveType::Int)));
    }
    // Float,
    else if string.starts_with("float") {
        return Ok(("float", Types::Primitive(PrimitiveType::Float)));
    }
    // Bool,
    else if string.starts_with("bool") {
        return Ok(("bool", Types::Primitive(PrimitiveType::Bool)));
    }
    // Char,
    else if string.starts_with("char") {
        return Ok(("char", Types::Primitive(PrimitiveType::Char)));
    }
    // Atom,
    else if string.starts_with("atom") {
        return Ok(("atom", Types::Primitive(PrimitiveType::Atom)));
    }
    Err("Err: no matching type keyword")
}

#[test]
fn test_s_to_type() {
    assert_eq!(
        string_to_type("str"),
        Ok(("str", Types::BuiltIn(BuiltinType::String)))
    );
    assert_eq!(
        string_to_type("enum"),
        Ok(("enum", Types::BuiltIn(BuiltinType::Enum)))
    );
    assert_eq!(
        string_to_type("tuple"),
        Ok(("tuple", Types::BuiltIn(BuiltinType::Tuple)))
    );
    assert_eq!(
        string_to_type("list"),
        Ok(("list", Types::BuiltIn(BuiltinType::List)))
    );
    assert_eq!(
        string_to_type("range"),
        Ok(("range", Types::BuiltIn(BuiltinType::Range)))
    );
    assert_eq!(
        string_to_type("map"),
        Ok(("map", Types::BuiltIn(BuiltinType::Map)))
    );
    assert_eq!(
        string_to_type("stream"),
        Ok(("stream", Types::BuiltIn(BuiltinType::Stream)))
    );
    assert_eq!(
        string_to_type("int"),
        Ok(("int", Types::Primitive(PrimitiveType::Int)))
    );
    assert_eq!(
        string_to_type("float"),
        Ok(("float", Types::Primitive(PrimitiveType::Float)))
    );
    assert_eq!(
        string_to_type("bool"),
        Ok(("bool", Types::Primitive(PrimitiveType::Bool)))
    );
    assert_eq!(
        string_to_type("char"),
        Ok(("char", Types::Primitive(PrimitiveType::Char)))
    );
    assert_eq!(
        string_to_type("atom"),
        Ok(("atom", Types::Primitive(PrimitiveType::Atom)))
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
}

pub fn operator_to_string(op: Operators) -> &'static str {
    match op {
        Operators::Add => "+",
        Operators::EnumConcat => "++",
        Operators::Subtract => "-",
        Operators::Div => "/",
        Operators::Mult => "*",
        Operators::Modulo => "%",
        Operators::Exp => "^",
        Operators::Concat => "<>",
        Operators::LessThan => "<",
        Operators::GreaterThan => ">",
        Operators::LEq => "<=",
        Operators::GEq => ">=",
        Operators::BEq => "==",
        Operators::BNEq => "!=",
        Operators::Lshift => "<<",
        Operators::Rshift => ">>",
        Operators::Into => "->",
        Operators::Equal => "=",
        Operators::DoubleDot => "..",
    }
}
pub fn string_to_operator(string: &str) -> Result<(&str, Operators), &str> {
    match string.chars().take(2).collect::<String>().as_str() {
        "++" => return Ok(("++", Operators::EnumConcat)),
        "<>" => return Ok(("<>", Operators::Concat)),
        "<=" => return Ok(("<=", Operators::LEq)),
        ">=" => return Ok((">=", Operators::GEq)),
        "==" => return Ok(("==", Operators::BEq)),
        "!=" => return Ok(("!=", Operators::BNEq)),
        "<<" => return Ok(("<<", Operators::Lshift)),
        ">>" => return Ok((">>", Operators::Rshift)),
        "->" => return Ok(("->", Operators::Into)),
        ".." => return Ok(("..", Operators::DoubleDot)),
        _ => {}
    }
    match string.chars().take(1).collect::<String>().as_str() {
        "+" => return Ok(("+", Operators::Add)),
        "-" => return Ok(("-", Operators::Subtract)),
        "/" => return Ok(("/", Operators::Div)),
        "*" => return Ok(("*", Operators::Mult)),
        "%" => return Ok(("%", Operators::Modulo)),
        "^" => return Ok(("^", Operators::Exp)),
        "<" => return Ok(("<", Operators::LessThan)),
        ">" => return Ok((">", Operators::GreaterThan)),
        "=" => return Ok(("=", Operators::Equal)),
        _ => {}
    };

    Err("Could not match string to an operator")
}

#[test]
fn test_s_to_op() {
    assert_eq!(string_to_operator("+"), Ok(("+", Operators::Add)));
    assert_eq!(string_to_operator("-"), Ok(("-", Operators::Subtract)));
    assert_eq!(string_to_operator("/"), Ok(("/", Operators::Div)));
    assert_eq!(string_to_operator("*"), Ok(("*", Operators::Mult)));
    assert_eq!(string_to_operator("%"), Ok(("%", Operators::Modulo)));
    assert_eq!(string_to_operator("^"), Ok(("^", Operators::Exp)));
    assert_eq!(string_to_operator("<"), Ok(("<", Operators::LessThan)));
    assert_eq!(string_to_operator(">"), Ok((">", Operators::GreaterThan)));
    assert_eq!(string_to_operator("="), Ok(("=", Operators::Equal)));
    assert_eq!(string_to_operator("++"), Ok(("++", Operators::EnumConcat)));
    assert_eq!(string_to_operator("<>"), Ok(("<>", Operators::Concat)));
    assert_eq!(string_to_operator("<="), Ok(("<=", Operators::LEq)));
    assert_eq!(string_to_operator(">="), Ok((">=", Operators::GEq)));
    assert_eq!(string_to_operator("=="), Ok(("==", Operators::BEq)));
    assert_eq!(string_to_operator("!="), Ok(("!=", Operators::BNEq)));
    assert_eq!(string_to_operator("<<"), Ok(("<<", Operators::Lshift)));
    assert_eq!(string_to_operator(">>"), Ok((">>", Operators::Rshift)));
    assert_eq!(string_to_operator("->"), Ok(("->", Operators::Into)));
    assert_eq!(string_to_operator(".."), Ok(("..", Operators::DoubleDot)));
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
    Continue,  // a loop continue ie br :loop_head
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

pub fn string_to_keyword(string: &str) -> Result<(&str, Keywords), &str> {
    if string.starts_with("import") {
        return Ok(("import", Keywords::Import));
    } else if string.starts_with("include") {
        return Ok(("include", Keywords::Include));
    } else if string.starts_with("struct") {
        return Ok(("struct", Keywords::Struct));
    } else if string.starts_with("for") {
        return Ok(("for", Keywords::For));
    } else if string.starts_with("continue") {
        return Ok(("continue", Keywords::Continue));
    } else if string.starts_with("break") {
        return Ok(("break", Keywords::Break));
    } else if string.starts_with("func") {
        return Ok(("func", Keywords::Func));
    } else if string.starts_with("fn") {
        return Ok(("fn", Keywords::Fn));
    } else if string.starts_with("match") {
        return Ok(("match", Keywords::Match));
    } else if string.starts_with("cond") {
        return Ok(("cond", Keywords::Cond));
    } else if string.starts_with("when") {
        return Ok(("when", Keywords::When));
    } else if string.starts_with("if") {
        return Ok(("if", Keywords::If));
    } else if string.starts_with("elif") {
        return Ok(("elif", Keywords::Elif));
    } else if string.starts_with("else") {
        return Ok(("else", Keywords::Else));
    } else if string.starts_with("return") {
        return Ok(("return", Keywords::Return));
    } else if string.starts_with("interface") {
        return Ok(("interface", Keywords::Interface));
    } else if string.starts_with("assert") {
        return Ok(("assert", Keywords::Assert));
    } else if string.starts_with("panic") {
        return Ok(("panic", Keywords::Panic));
    } else if string.starts_with("test") {
        return Ok(("test", Keywords::Test));
    } else if string.starts_with("<EOF>") {
        return Ok(("<EOF>", Keywords::EoF));
    }
    Err("Err: could not match keyword.")
}

#[test]
fn test_s_to_keyword() {
    assert_eq!(
        string_to_keyword("import"),
        Ok(("import", Keywords::Import))
    );
    assert_eq!(
        string_to_keyword("include"),
        Ok(("include", Keywords::Include))
    );
    assert_eq!(
        string_to_keyword("struct"),
        Ok(("struct", Keywords::Struct))
    );
    assert_eq!(string_to_keyword("for"), Ok(("for", Keywords::For)));
    assert_eq!(
        string_to_keyword("continue"),
        Ok(("continue", Keywords::Continue))
    );
    assert_eq!(string_to_keyword("break"), Ok(("break", Keywords::Break)));
    assert_eq!(string_to_keyword("func"), Ok(("func", Keywords::Func)));
    assert_eq!(string_to_keyword("fn"), Ok(("fn", Keywords::Fn)));
    assert_eq!(string_to_keyword("match"), Ok(("match", Keywords::Match)));
    assert_eq!(string_to_keyword("cond"), Ok(("cond", Keywords::Cond)));
    assert_eq!(string_to_keyword("when"), Ok(("when", Keywords::When)));
    assert_eq!(string_to_keyword("if"), Ok(("if", Keywords::If)));
    assert_eq!(string_to_keyword("elif"), Ok(("elif", Keywords::Elif)));
    assert_eq!(string_to_keyword("else"), Ok(("else", Keywords::Else)));
    assert_eq!(
        string_to_keyword("return"),
        Ok(("return", Keywords::Return))
    );
    assert_eq!(
        string_to_keyword("interface"),
        Ok(("interface", Keywords::Interface))
    );
    assert_eq!(
        string_to_keyword("assert"),
        Ok(("assert", Keywords::Assert))
    );
    assert_eq!(string_to_keyword("panic"), Ok(("panic", Keywords::Panic)));
    assert_eq!(string_to_keyword("test"), Ok(("test", Keywords::Test)));
    assert_eq!(string_to_keyword("<EOF>"), Ok(("<EOF>", Keywords::EoF)));
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

enum Delimiters {
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
    let mut max_run = 10;
    // if string.len() > 1 {
    //     let mut ros = tokenize(string.get(1..).unwrap());
    //     v.append(&mut ros);
    // }
    // let mut input: &str = input_string;
    // let tokenizers: Vec<fn &str -> Result<(&str, _), &str>> = vec![
    //     // parse_keyword,
    //     string_to_keyword,
    //     string_to_operator,
    //     string_to_type,
    // ];
    println!("tokenize instr: {:?}", input_string);
    'tokenLoop: while !input_string.is_empty() && max_run > 0 {
        input_string = input_string.trim();
        match string_to_keyword(input_string) {
            Ok((s, kw)) => {
                v.push(Token {
                    string: s.to_string(),
                    token_type: TokenType::Keyword(kw),
                });
                input_string = input_string.get(s.len() + 1..).unwrap_or("");
                continue 'tokenLoop;
            }
            Err(s) => {}
        }

        match string_to_operator(input_string) {
            Ok((s, op)) => {
                v.push(Token {
                    string: s.to_string(),
                    token_type: TokenType::Operator(op),
                });
                input_string = input_string.get(s.len() + 1..).unwrap_or("");
                continue 'tokenLoop;
            }
            Err(s) => {}
        }

        match string_to_type(input_string) {
            Ok((s, typ)) => {
                v.push(Token {
                    string: s.to_string(),
                    token_type: TokenType::Type(typ),
                });
                input_string = input_string.get(s.len() + 1..).unwrap_or("");
                continue 'tokenLoop;
            }
            Err(s) => {}
        }

        match tokenize_identifier(input_string) {
            Ok((s, typ)) => {
                v.push(Token {
                    string: s.to_string(),
                    token_type: typ,
                });
                input_string = input_string.get(s.len() + 1..).unwrap_or("");
                continue 'tokenLoop;
            }
            Err(s) => {}
        }

        println!("instr: {:?} | max_run: {}", input_string, max_run);
        max_run -= 1;
    }

    v.to_vec()
}
