#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

extern crate regex;
use std::string;

use regex::Regex;

// use std::env;

// use std::str::pattern::Pattern;

#[derive(Debug, Clone)]
enum TokenType {
    Primitive(PrimitiveType),
    BuiltIn(BuiltinType),
    Keyword(Keywords),
    Operator(Operators),
    EoF,
    Identifier,
}

#[derive(Debug, Clone)]
enum ExpressionType {
    Definition,   // <type> <ident> = <expression>
    Calculation,  // <ident> <operator> <expression>
    FunctionCall, // <functionName>(<parameterList>)
    Literal,      // <a literal>
}

#[derive(Debug, Clone)]
enum PrimitiveType {
    Int,
    Float,
    Bool,
    Char,
    Atom,
}
#[derive(Debug, Clone)]
enum BuiltinType {
    // in order of importance
    String,
    Enum,
    Tuple,
    List,
    Range,
    Map,
    Stream,
}

#[derive(Debug, Clone)]
enum Operators {
    Add,         // "+",
    EnumConcat,  // "++",
    Subtract,    // "-",
    Div,         // "/",
    Mult,        // "*",
    Modulo,      // "%",
    Exp,         // "^",
    Concat,      //(for strings), // "<>",
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

/*
f/ https://go.dev/ref/spec#Keywords
break        default      func         interface    select
case         defer        go           map          struct
chan         else         goto         package      switch
const        fallthrough  if           range        type
continue     for          import       return       var
*/
#[derive(Debug, Clone)]
enum Keywords {
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
    Assert, // allows for runtime assertions
    Panic, // nukes everything and panics
    // Test,       // allows for testing in modules, which auto tests on `rho test || rho t`
    EoF,
}

#[derive(Debug, Clone)]
struct Token {
    string: String,
    token_type: TokenType,
}

/*
Int,
Float,
Bool,
Char,
Atom,
..
String,
Enum,
Tuple,
List,
Range,
Map,
Stream,
*/
fn parse_type(string: &str) -> Result<(Token, &str), &str> {
    if string.len() < 3 {
        return Err("not long enough for a type");
    }
    if let ("str", s) = string.split_at("str".len()) {
        let t = Token {
            string: "str".to_string(),
            token_type: TokenType::BuiltIn(BuiltinType::String),
        };
        Ok((t, s))
    } else if let ("int", s) = string.split_at("int".len()) {
        let t = Token {
            string: "int".to_string(),
            token_type: TokenType::Primitive(PrimitiveType::Int),
        };
        Ok((t, s))
    } else {
        Err("not a parsable type")
    }
}

fn parse_keyword(string: &str) -> Result<(Token, &str), &str> {
    Err("Err: parse_keyword not implemented")
}

fn parse_identifier(string: &str) -> Result<(Token, &str), &str> {
    match string.split_once(|c: char| c.is_whitespace()) {
        Some((ident, ros)) => {
            let t = (
                Token {
                    string: ident.to_string(),
                    token_type: TokenType::Identifier,
                },
                ros,
            );
            Ok(t)
        }
        None => Err("Err: Could not parse identifer"),
    }
}

fn parse_operator(string: &str) -> Result<(Token, &str), &str> {
    if string.is_empty() {
        return Err("not long enough for a type");
    }
    if let ("=", s) = string.split_at("=".len()) {
        let t = Token {
            string: "=".to_string(),
            token_type: TokenType::Operator(Operators::Equal),
        };
        Ok((t, s))
    } else if let ("+", s) = string.split_at("+".len()) {
        let t = Token {
            string: "+".to_string(),
            token_type: TokenType::Operator(Operators::Add),
        };
        Ok((t, s))
    } else {
        Err("not a parsable type")
    }
}

fn parse_literal(string: &str) -> Result<(Token, &str), &str> {
    if string.is_empty() {
        return Err("not long enough for a type");
    }

    match parse_numeric_literal(string) {
        Ok(ret) => {
            return Ok(ret);
        }
        Err(_) => {}
    }

    if string.starts_with('\"') {
        return parse_string_literal(string);
    } else {
        Err("not a parsable type")
    }
}

fn parse_numeric_literal(mut string: &str) -> Result<(Token, &str), &str> {
    // TODO: need to check if the preciding character is a negative..
    let is_negative = string.starts_with('-');
    if is_negative {
        string = string.get(1..).unwrap_or("")
    }
    let mut n = string
        .chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>();

    string = string.get(n.len()..).unwrap_or("");

    if is_negative {
        n.insert(0, '-');
    }

    println!("n {:?} | isNeg {:?} | string {:?}", n, is_negative, string);

    if !n.is_empty() {
        if !string.is_empty() && string.as_bytes()[0] == b'.' {
            // either a float literal or an error
            if string.len() >= 2 {
                if string.as_bytes()[1] != b'.' {
                    let dec = string
                        .get(1..)
                        .unwrap_or("")
                        .chars()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>();
                    let token_string = n + "." + &dec;
                    let lts = token_string.len();
                    return Ok((
                        Token {
                            string: token_string,
                            token_type: TokenType::Primitive(PrimitiveType::Float),
                        },
                        string.get(dec.len() + 1..).unwrap_or(""),
                    ));
                } else {
                    return Ok((
                        Token {
                            string: n,
                            token_type: TokenType::Primitive(PrimitiveType::Int),
                        },
                        string,
                    ));
                }
            }
        } else {
            return Ok((
                Token {
                    string: n,
                    token_type: TokenType::Primitive(PrimitiveType::Int),
                },
                string,
            ));
        }
    }
    Err("Err: could not parse numeric")
}

fn parse_string_literal(string: &str) -> Result<(Token, &str), &str> {
    if !string.starts_with('\"') || string.len() < 2 {
        return Err("Not a string literal");
    }
    for i in 1..(string.len()) {
        if string.as_bytes()[i] == b'\"' && string.as_bytes()[i - 1] != b'\\' {
            let t = Token {
                string: string.get(1..i - 1).unwrap().to_string(),
                token_type: TokenType::BuiltIn(BuiltinType::String),
            };
            let s = string.get(i + 1..).unwrap_or("");
            return Ok((t, s));
        }
    }

    // TODO: should panic
    Err("Err: String literal never closed.")
}

// static PARSERS: Vec<fn(&str) -> Result<(Token, &str), &str>> = [parse_keyword, parse_type, parse_identifier].to_vec();

fn tokenize(input_string: &str) -> Vec<Token> {
    let v = &mut vec![];
    // if string.len() > 1 {
    //     let mut ros = tokenize(string.get(1..).unwrap());
    //     v.append(&mut ros);
    // }
    let mut input: &str = input_string;
    let parsers = vec![
        parse_keyword,
        parse_type,
        parse_operator,
        parse_literal,
        parse_identifier,
    ];

    'drain_input: while !input.is_empty() {
        for parser in &parsers {
            match parser(input) {
                Ok((t, s)) => {
                    v.push(t);
                    input = s.trim_start();
                    continue 'drain_input;
                }
                Err(err) => {
                    // input = match input.get(1..) {
                    //     Some(s) => s,
                    //     None => "",
                    // };

                    // println!("{:?}", err)
                }
            }
        }
        input = match input.get(1..) {
            Some(s) => s,
            None => "",
        };
    }
    // v.push((parse_type(input_string).unwrap()).0);

    v.to_vec()
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let file_path = &args[1];

    // println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let file_path = "../hello.rho";
    // let contents: &str = &fs::read_to_string(file_path).expect("Should have been able to read the file");fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{:?}", "abc".split_at(10));

    let contents = "str s = \"rho_is_cool\"\nint a = 1 + 2";

    // println!("{:?}", contents.split_once(|c: char| c.is_whitespace()));
    println!("{:?}", contents);
    println!("tokenize output: {:?}", tokenize(contents));

    // let s = "\"asdfklsdjfa\\\"\" asdf";
    // println!("{:?}", parse_literal(s))
    // while true {
    //     s.find("\n")
    // }

    println!("{:?}: {:?}\n", "1..1", parse_literal("1..1"));
    println!("{:?}: {:?}\n", "10..1.", parse_literal("10..1."));
    println!("{:?}: {:?}\n", "-1.0 1.0", parse_literal("-1.0 1.0"));
    println!("{:?}: {:?}\n", "1 ", parse_literal("1 "));
    println!("{:?}: {:?}\n", "1", parse_literal("-1 "));
    println!("{:?}: {:?}\n", "-1", parse_literal("-1 "));
    println!("{:?}: {:?}\n", "-2.14", parse_literal("-2.14"));

    // println!("{:?}");

    // let (mut t, mut ros) = tokenize(contents);
    // while !ros.is_empty() {
    //     println!("{t}, {ros}");
    //     (t, ros) = tokenize(ros);
    // }
    // println!("{:?}", tokenize(contents));

    // println!("{:?}", format!("@{0} = constant [{1} x i8] c\"Hellow, World!\0A\"", "name", 16))
}
