use core::fmt;
use std::fmt::write;

use crate::tokens::{self, *};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Expression {
//     tokens: Vec<Token>,
// }
trait HasLen {
    fn len(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Definition {
        definition_type: &'a Token<'a>,
        identifier: &'a Token<'a>,
        value: Box<Expression<'a>>,
    }, // <type> <ident> = <expression>
    Calculation, // <expression> <operator> <expression>
    FunctionCall {
        function_name: FunctionName<'a>,
        parameters: ParameterList<'a>,
    }, // <functionName>(<parameterList>) && functionName : <ident>.<ident>.<ident>...
    // TODO: Change literalType to take literals
    Literal {
        literal_type: TokenType,
        token: &'a Token<'a>,
    }, // <a literal>
    Identifier, // <ident>
    Empty {
        tokens: Vec<&'a Token<'a>>,
    },
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Definition {
                definition_type,
                identifier,
                value,
            } => write!(
                f,
                "Def[t: {:?} | ident: {:?} | val: {:?}]",
                definition_type.token_type, identifier.string, value
            ),
            // Expression::Calculation => todo!(),
            // Expression::FunctionCall => todo!(),
            // Expression::Literal { literal_type, token } => todo!(),
            // Expression::Identifier => todo!(),
            // Expression::Empty { tokens } => todo!(),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl HasLen for Expression<'_> {
    fn len(&self) -> usize {
        match &self {
            Expression::Calculation => todo!(),
            Expression::FunctionCall {
                function_name,
                parameters,
            } => function_name.idents.len() + parameters.params.len(),
            // Expression::Literal(Literal) => 1,
            Expression::Identifier => todo!(),
            Expression::Definition {
                definition_type,
                identifier,
                value,
            } => 3 + value.len(),
            Expression::Literal {
                literal_type,
                token,
            } => 1,
            Expression::Empty { tokens } => tokens.len(),
            // Expression::Empty(se) => se.tokens.len(),
        }
    }
}

// pub struct Expression {
//     expression_type: ExpressionType,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Empty {
//     tokens: Vec<Token>,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Definition {
//     definition_type: Token,
//     identifier: Token,
//     value: Expression,
// }

// impl fmt::Display for Definition<'static> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self.definition_type.token_type {
//             TokenType::Type(Types::Primitive(PrimitiveType::Int)) => {
//                 write!(
//                     f,
//                     "
//     %{1} = alloca i32
//     %computed_value_{1} = {0:?}

//     store i32 %computed_value_{1}, i32* %{1}
//                 ",
//                     self.value,
//                     // self.identifier.string,
//                     self.identifier.string
//                 )
//             }
//             TokenType::Type(_) => todo!(),
//             _ => todo!(),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Calculation<'a> {
    left: Token<'a>,
    operator: Token<'a>,
    right: Token<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall<'a> {
    function_name: Token<'a>,
    parameter_list: Vec<Token<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionName<'a> {
    idents: Vec<Token<'a>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList<'a> {
    params: Vec<Token<'a>>,
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Literal {
//     token: Token,
// }

// fn create_expression_literal<'a>(token: &'a Token) -> Expression<'a> {
//     // let tok = token.to_owned();
//     // let expression =
//     Expression::Literal {
//         literal_type: token.token_type,
//         token: token,
//     }
// }

fn match_empty<'a>(tokens: Vec<&'a Token<'a>>) -> Result<Expression<'a>, &'a str> {
    let empty_len: usize = tokens
        .iter()
        .take_while(|t| {
            matches!(t.token_type, TokenType::NewLine) || matches!(t.token_type, TokenType::Comment)
        })
        .count();
    if empty_len > 0 {
        return Ok(Expression::Empty {
            tokens: tokens.get(0..empty_len).unwrap().to_vec(),
        });
    }
    Err("Failed to match empty")
}

fn match_definition<'a>(tokens: Vec<&'a Token<'a>>) -> Result<Expression<'a>, &'a str> {
    if matches!(tokens.get(0).unwrap().token_type, TokenType::Type(_))
        && matches!(tokens.get(1).unwrap().token_type, TokenType::Identifier)
        && matches!(
            tokens.get(2).unwrap().token_type,
            TokenType::Operator(Operators::Equal)
        )
        && matches!(tokens.get(3).unwrap().token_type, TokenType::Literal(_))
    {
        let dt = tokens.get(0).unwrap();
        let literal_token = tokens.get(3).unwrap().to_owned();
        let v: Expression<'a> = Expression::Literal {
            literal_type: tokens.get(3).unwrap().token_type,
            token: tokens.get(3).unwrap(),
        };
        let expression: Result<Expression<'a>, &str> = Ok(Expression::Definition {
            definition_type: dt,
            identifier: tokens.get(1).unwrap(),
            value: Box::new(v),
        });
        return expression;
    }
    Err("Could not match defintion")
}

fn match_function_name<'a>(tokens: Vec<&'a Token<'a>>) -> Result<FunctionName<'a>, &'a str> {
    let iter = tokens.iter();
    let mut fn_name: Vec<Token> = vec![];
    // let x = iter.clone().next().unwrap();
    'keep_matching: for t in iter {
        if matches!(t.token_type, TokenType::Identifier)
            && matches!(
                fn_name
                    .last()
                    .unwrap_or(&Token {
                        string: "",
                        token_type: TokenType::Delimiter(Delimiters::Period)
                    })
                    .token_type,
                TokenType::Delimiter(Delimiters::Period)
            )
        {
            fn_name.push(**t);
            continue 'keep_matching;
        }
        if matches!(t.token_type, TokenType::Delimiter(Delimiters::Period))
            && matches!(
                fn_name
                    .last()
                    .unwrap_or(&Token {
                        string: "",
                        token_type: TokenType::EoF
                    })
                    .token_type,
                TokenType::Identifier
            )
        {
            fn_name.push(**t);
            continue 'keep_matching;
        }
        break;
    }
    if matches!(
        fn_name
            .last()
            .unwrap_or(&Token {
                string: "",
                token_type: TokenType::EoF
            })
            .token_type,
        TokenType::Identifier
    ) {
        return Ok(FunctionName { idents: fn_name });
    }
    println!("fn name: {:?}", fn_name);

    Err("")
}

fn match_parameter_list<'a>(tokens: Vec<&'a Token<'a>>) -> Result<ParameterList<'a>, &'a str> {
    if matches!(
        tokens.first().unwrap().token_type,
        TokenType::Delimiter(Delimiters::ParOpen)
    ) {
        let mut params: Vec<Token> = tokens
            .clone()
            .into_iter()
            .take_while(|t| !matches!(t.token_type, TokenType::Delimiter(Delimiters::ParClose)))
            .collect();
        let t = tokens.get(params.len()).unwrap();
        if !(params.is_empty()) {
            if matches!(t.token_type, TokenType::Delimiter(Delimiters::ParClose)) {
                params.push(**t)
            }
            return Ok(ParameterList { params });
        }
        // if params.collect()
    }
    Err("Couldn't match param list")
}

fn match_function_call<'a>(tokens: Vec<&'a Token<'a>>) -> Result<Expression<'a>, &'a str> {
    let iter = tokens.clone().into_iter();
    let fn_name: FunctionName<'_>;
    match match_function_name(tokens.clone()) {
        Ok(e) => fn_name = e,
        Err(_) => {
            return Err("Could not match function name");
        }
    };
    println!("{:?}", fn_name);
    let params: ParameterList<'_>;
    let toks: Vec<&Token> = iter.skip(fn_name.idents.len()).collect();
    match match_parameter_list(toks) {
        Ok(e) => params = e,
        Err(_) => {
            return Err("Could not match function name");
        }
    }
    println!("params {:?}", params);
    let e = Expression::FunctionCall {
        function_name: fn_name,
        parameters: params,
    };
    return Ok(e);
    // need to match against a parameter list now.
}

pub fn match_expression<'a>(tokens: Vec<&'a Token<'a>>) -> Result<Expression<'a>, &'a str> {
    match match_empty(tokens.clone()) {
        Ok(e) => {
            return Ok(e);
        }
        Err(_) => {}
    }
    match match_definition(tokens.clone()) {
        Ok(e) => {
            return Ok(e);
        }
        Err(_) => {}
    }

    match match_function_call(tokens.clone()) {
        Ok(e) => {
            return Ok(e);
        }
        Err(_) => {}
    }
    // match def

    Err("Not implemented")
}

pub fn expressionize<'a>(mut tokens: Vec<&'a Token<'a>>) -> Vec<Expression<'a>> {
    let mut v: Vec<Expression> = vec![];

    'keep_matching: while !tokens.is_empty() {
        match match_expression(tokens.clone()) {
            Ok(e) => {
                let l = e.len();
                v.push(e.to_owned());
                tokens = tokens.drain(l..).collect();
                // v.iter().skip(e.len());
                continue 'keep_matching;
            }
            Err(e) => {}
        }
        println!("end_tokens: {:?}", tokens);
        break;
    }

    // v.reverse();
    v
}
