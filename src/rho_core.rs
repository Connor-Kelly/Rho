use crate::tokens::*;


enum CallResult {
    Ok(Vec<Types>), // <- returns the function's return types
    Err
}

// mod IO {
fn inspect(token: Token) -> Result<String, String> {
    match token.token_type {
        TokenType::Type(Types::Primitive(PrimitiveType::Int)) => todo!(),
        TokenType::Type(_) => todo!(),
        TokenType::Literal(_) => todo!(),
        TokenType::Keyword(_) => todo!(),
        TokenType::Operator(_) => todo!(),
        TokenType::Delimiter(_) => todo!(),
        TokenType::EoF => todo!(),
        TokenType::Identifier => todo!(),
        TokenType::Comment => todo!(),
        TokenType::NewLine => todo!(),
    }

    // Err("Not implemented".to_string())
}
// }
