#[derive(Debug, Clone)]
pub struct Token {
    pub string: String,
    pub token_type: TokenType,
    pub value: TokenValue,
}

impl Token {
    // fn is_primitive(&self) -> bool {
    //     matches!(self.token_type, TokenType::Primitive(_))
    // }
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
    Err("Could not match string to an operator")
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
    Panic, // nukes everything and panics
    // Test,       // allows for testing in modules, which auto tests on `rho test || rho t`
    EoF,
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
