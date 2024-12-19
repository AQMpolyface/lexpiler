mod lexer;
mod parser;

use parser::Token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};

//using external crate cuz the std::exit_with_code msde some weird mess with the stdout
#[quit::main]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        quit::with_code(64);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                quit::with_code(64);
            });

            let mut lexer = Lexer::new();
            let exit_code = lexer.tokenize(&file_contents);

            quit::with_code(exit_code);
        }
        "parse" => {
            writeln!(io::stderr(), "Parsing file...").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                quit::with_code(64);
            });

            let mut parser = Parser::new();
            let (exit_code, tokens) = parser.parse(&file_contents);
            //println!("tokens: {:?}", tokens);
            let result = parse_more(tokens);

            /*
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            }*/
            //let result2 = parse_signs(result);

            for uwu in result {
                println!("{}", uwu);
            }
            quit::with_code(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            quit::with_code(64);
        }
    }
}

fn parse_signs(tokens: Vec<String>) -> Vec<String> {
    let mut result = tokens.clone();

    // First pass: handle unary minus
    let mut i = 0;
    while i < result.len() {
        if result[i] == "-" {
            // Check if it's a unary minus (at start or after another operator)
            if i == 0
                || (i > 0
                    && (result[i - 1] == "+"
                        || result[i - 1] == "-"
                        || result[i - 1] == "*"
                        || result[i - 1] == "/"))
            {
                if i + 1 < result.len() {
                    let rhs = result[i + 1].clone();
                    let new_expr = format!("(- {})", rhs);
                    result.splice(i..=i + 1, vec![new_expr]);
                }
            }
        }
        i += 1;
    }

    // Second pass: handle multiplication and division
    let mut i = 1;
    while i < result.len() {
        match result[i].as_str() {
            "*" | "/" => {
                if i > 0 && i < result.len() - 1 {
                    let operator = result[i].clone();
                    let lhs = result[i - 1].clone();
                    let rhs = result[i + 1].clone();
                    let new_expr = format!("({} {} {})", operator, lhs, rhs);
                    result.splice(i - 1..=i + 1, vec![new_expr]);
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Third pass: handle addition and subtraction
    let mut i = 1;
    while i < result.len() {
        match result[i].as_str() {
            "+" | "-" => {
                if i > 0 && i < result.len() - 1 {
                    let operator = result[i].clone();
                    let lhs = result[i - 1].clone();
                    let rhs = result[i + 1].clone();
                    let new_expr = format!("({} {} {})", operator, lhs, rhs);
                    result.splice(i - 1..=i + 1, vec![new_expr]);
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }

    result
}

fn parse_more(tokens: Vec<Token>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut send_sign = false;
    let mut has_shitty_sign = false;
    let mut debug = false;
    while i < tokens.len() {
        let token = &tokens[i];
        let token_type = token.token_type.as_str();

        match token_type {
            "NUMBER" | "STRING" => {
                result.push(token.literal.clone());
            }
            "LEFT_PAREN" => {
                // Handle grouped expressions
                let mut inner_tokens = Vec::new();
                let mut paren_count = 1;
                i += 1;

                while i < tokens.len() && paren_count > 0 {
                    let inner_token = &tokens[i];
                    let inner_type = inner_token.token_type.as_str();

                    if inner_type == "LEFT_PAREN" {
                        paren_count += 1;
                    } else if inner_type == "RIGHT_PAREN" {
                        paren_count -= 1;
                        if paren_count == 0 {
                            break;
                        }
                    }
                    inner_tokens.push(inner_token.clone());
                    i += 1;
                }

                let inner_result = parse_more(inner_tokens);
                result.push(format!("(group {})", inner_result.join(" ")));
            }
            "MINUS" => {
                result.push("-".to_string());
                send_sign = true;
            }
            "PLUS" => {
                result.push("+".to_string());
                send_sign = true;
            }
            "SLASH" => {
                result.push("/".to_string());
                send_sign = true;
            }
            "STAR" => {
                result.push("*".to_string());
                send_sign = true;
            }
            "TRUE" | "FALSE" | "NIL" => {
                result.push(token.lexeme.clone());
            }
            "BANG" => {
                i += 1; // Advance to the next token after the first BANG
                let mut inner_tokens: Vec<Token> = Vec::new();

                // Collect consecutive BANG tokens
                while i < tokens.len() && tokens[i].token_type.as_str() == "BANG"
                    || tokens[i].token_type.as_str() == "LEFT_PAREN"
                {
                    inner_tokens.push(tokens[i].clone());
                    i += 1;
                }

                // Ensure there's a non-BANG token after the sequence
                if i < tokens.len() {
                    inner_tokens.push(tokens[i].clone());
                    i += 1; // Consume the final token
                } else {
                    panic!("Unexpected end of input after BANG tokens");
                }

                // Parse the collected tokens
                let inner_result = parse_more(inner_tokens);
                result.push(format!("(! {})", inner_result.join(" ")));
            }
            _ => {}
        }
        i += 1;
    }
    if debug {
        if send_sign {
            println!("callinf parse_sign on{:#?}", result);
        } else {
            println!("not calling parse sign on {:?}", result);
        }
    }
    if send_sign {
        parse_signs(result)
    } else {
        result
    }
}
