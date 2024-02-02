use crate::tokens::*;

pub fn parse_keyword(string: &str) -> Result<(Token, &str), &str> {
    Err("Err: parse_keyword not implemented")
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
pub fn parse_type(string: &str) -> Result<(Token, &str), &str> {
    if string.len() < 3 {
        return Err("not long enough for a type");
    }
    if let ("str", s) = string.split_at("str".len()) {
        let t = Token {
            string: "str".to_string(),
            token_type: TokenType::Type(Types::BuiltIn(BuiltinType::String)),
            value: TokenValue::String("str".to_string()),
        };
        Ok((t, s))
    } else if let ("int", s) = string.split_at("int".len()) {
        let t = Token {
            string: "int".to_string(),
            token_type: TokenType::Type(Types::Primitive(PrimitiveType::Int)),
            value: TokenValue::String("int".to_string()),
        };
        Ok((t, s))
    } else {
        Err("not a parsable type")
    }
}

// pub fn parse_keyword(string: &str) -> Result<(Token, &str), &str> {
//     Err("Err: parse_keyword not implemented")
// }

pub fn parse_identifier(string: &str) -> Result<(Token, &str), &str> {
    match string.split_once(|c: char| c.is_whitespace()) {
        Some((ident, ros)) => {
            let t = (
                Token {
                    string: ident.to_string(),
                    token_type: TokenType::Identifier,
                    value: TokenValue::String(ident.to_string()),
                },
                ros,
            );
            Ok(t)
        }
        None => Err("Err: Could not parse identifer"),
    }
}

pub fn parse_operator(string: &str) -> Result<(Token, &str), &str> {
    match string_to_operator(string) {
        Ok((s, op)) => Ok((
            Token {
                string: s.to_string(),
                token_type: TokenType::Operator(op),
                value: TokenValue::String(s.to_string()),
            },
            string.get(s.len() + 1..).unwrap_or(""),
        )),
        Err(err) => Err("Could not parse operator"),
    }
}

pub fn parse_literal(string: &str) -> Result<(Token, &str), &str> {
    if string.is_empty() {
        return Err("not long enough for a type");
    }

    // parse a numeric
    match parse_numeric_literal(string) {
        Ok(ret) => {
            return Ok(ret);
        }
        Err(_) => {}
    }

    // parse bool
    match string
        .chars()
        .take_while(|c| c.is_alphabetic())
        .collect::<String>()
        .as_str()
    {
        "true" => {
            return Ok((
                Token {
                    string: "true".to_string(),
                    token_type: TokenType::Type(Types::Primitive(PrimitiveType::Bool)),
                    value: TokenValue::Bool(true),
                },
                string.get("true".len()..).unwrap_or(""),
            ));
        }
        "false" => {
            return Ok((
                Token {
                    string: "false".to_string(),
                    token_type: TokenType::Type(Types::Primitive(PrimitiveType::Bool)),
                    value: TokenValue::Bool(false),
                },
                string.get("false".len()..).unwrap_or(""),
            ));
        }
        _ => {}
    }

    // parse an atom
    if string.starts_with(':') {
        let atom = string
            .get(1..)
            .unwrap_or("")
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect::<String>();
        if !atom.is_empty() {
            return Ok((
                Token {
                    string: ":".to_owned() + &atom,
                    token_type: TokenType::Type(Types::Primitive(PrimitiveType::Atom)),
                    value: TokenValue::String(":".to_owned() + &atom),
                },
                string.get(atom.len() + 1..).unwrap_or(""),
            ));
        }
    }

    if string.starts_with('\"') {
        return parse_string_literal(string);
    } else {
        Err("not a parsable type")
    }
}

pub fn parse_numeric_literal(mut string: &str) -> Result<(Token, &str), &str> {
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
                    match token_string.parse::<f32>() {
                        Ok(parsed_number) => {
                            return Ok((
                                Token {
                                    string: token_string,
                                    token_type: TokenType::Type(Types::Primitive(
                                        PrimitiveType::Float,
                                    )),
                                    value: TokenValue::Float(parsed_number),
                                },
                                string.get(dec.len() + 1..).unwrap_or(""),
                            ));
                        }
                        Err(err) => return Err("Err: could not parse float"),
                    }
                } else {
                    match n.parse() {
                        Ok(parsed_number) => {
                            return Ok((
                                Token {
                                    string: n,
                                    token_type: TokenType::Type(Types::Primitive(
                                        PrimitiveType::Int,
                                    )),
                                    value: TokenValue::Int(parsed_number),
                                },
                                string,
                            ));
                        }
                        Err(err) => return Err("Err: could not parse int"),
                    }
                }
            }
        } else {
            match n.parse() {
                Ok(parsed_number) => {
                    return Ok((
                        Token {
                            string: n,
                            token_type: TokenType::Type(Types::Primitive(PrimitiveType::Int)),
                            value: TokenValue::Int(parsed_number),
                        },
                        string,
                    ));
                }
                Err(err) => return Err("Err: could not parse int"),
            }
        }
    }
    Err("Err: could not parse numeric")
}

pub fn parse_string_literal(string: &str) -> Result<(Token, &str), &str> {
    if !string.starts_with('\"') || string.len() < 2 {
        return Err("Not a string literal");
    }
    for i in 1..(string.len()) {
        if string.as_bytes()[i] == b'\"' && string.as_bytes()[i - 1] != b'\\' {
            let t = Token {
                string: string.get(1..i - 1).unwrap().to_string(),
                token_type: TokenType::Type(Types::BuiltIn(BuiltinType::String)),
                value: TokenValue::String(string.get(1..i - 1).unwrap().to_string()),
            };
            let s = string.get(i + 1..).unwrap_or("");
            return Ok((t, s));
        }
    }

    // TODO: should panic
    Err("Err: String literal never closed.")
}

// static PARSERS: Vec<pub fn(&str) -> Result<(Token, &str), &str>> = [parse_keyword, parse_type, parse_identifier].to_vec();

pub fn tokenize(input_string: &str) -> Vec<Token> {
    let v = &mut vec![];
    // if string.len() > 1 {
    //     let mut ros = tokenize(string.get(1..).unwrap());
    //     v.append(&mut ros);
    // }
    let mut input: &str = input_string;
    let parsers = vec![
        // parse_keyword,
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
