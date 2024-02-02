#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

extern crate regex;
use regex::Regex;
use std::string;

mod parsers;
mod tokens;
use crate::parsers::*;
use crate::tokens::*;

// use std::env;

// use std::str::pattern::Pattern;


fn main() {
    // let args: Vec<String> = env::args().collect();

    // let file_path = &args[1];

    // println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let file_path = "../hello.rho";
    // let contents: &str = &fs::read_to_string(file_path).expect("Should have been able to read the file");fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{:?}", "abc".split_at(10));

    let contents = "str s = \"rho_is_cool\"\n";

    // // println!("{:?}", contents.split_once(|c: char| c.is_whitespace()));
    // println!("{:?}", contents);
    println!("tokenize output: {:?}", tokenize(contents));

    // let s = "\"asdfklsdjfa\\\"\" asdf";
    // println!("{:?}", parse_literal(s))
    // while true {
    //     s.find("\n")
    // }

    // println!("{:?}: {:?}\n", "1..1", parse_literal("1..1"));
    // println!("{:?}: {:?}\n", "10..1.", parse_literal("10..1."));
    // println!("{:?}: {:?}\n", "-1.0 1.0", parse_literal("-1.0 1.0"));
    // println!("{:?}: {:?}\n", "1 ", parse_literal("1 "));
    // println!("{:?}: {:?}\n", "1", parse_literal("-1 "));
    // println!("{:?}: {:?}\n", "-1", parse_literal("-1 "));
    // println!("{:?}: {:?}\n", "-2.14", parse_literal("-2.14"));

    // println!("{:?}: {:?}\n", ":my0Atom:another ", parse_literal(":my0Atom:another "));

    // println!("{:?}: {:?}\n", "true false", tokenize("true asdf"));
    // println!("{:?}: {:?}\n", "false", parse_literal("false"));
    // println!("{:?}: {:?}\n", "falsetrue", parse_literal("falsetrue"));
    // let tokens = tokenize("true false");
    // println!("{:?}\n", tokens);

    // println!("{:?}", parsers::parse_keyword("func"))
    // println!(
    //     "{:?}",
    //     matches!(
    //         tokens.first().unwrap().token_type,
    //         TokenType::Type(Types::Primitive(_))
    //     )
    // )

    // let (mut t, mut ros) = tokenize(contents);
    // while !ros.is_empty() {
    //     println!("{t}, {ros}");
    //     (t, ros) = tokenize(ros);
    // }
    // println!("{:?}", tokenize(contents));

    // println!("{:?}", format!("@{0} = constant [{1} x i8] c\"Hellow, World!\0A\"", "name", 16))
}
