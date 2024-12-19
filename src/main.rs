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
            /*for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            } */
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

    // First pass: handle multiplication and division left to right
    let mut i = 1; // Start at 1 since we need to look at previous token
    while i < result.len() {
        match result[i].as_str() {
            "*" | "/" => {
                if i > 0 && i < result.len() - 1 {
                    let operator = result[i].clone();
                    let lhs = result[i - 1].clone();
                    let rhs = result[i + 1].clone();

                    // Create the new expression
                    let new_expr = format!("({} {} {})", operator, lhs, rhs);

                    // Replace the three tokens with the new expression
                    result.splice(i - 1..=i + 1, vec![new_expr]);

                    // Don't increment i since we need to check the next operator
                    // from the current position after modification
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Second pass: handle remaining operators
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
    while i < tokens.len() {
        let token = &tokens[i];
        let token_type = token.token_type.as_str();

        match token_type {
            "TRUE" => result.push("true".to_string()),
            "FALSE" => result.push("false".to_string()),
            "NIL" => result.push("nil".to_string()),
            "NUMBER" | "STRING" => {
                result.push(token.literal.clone());
            }
            "LEFT_PAREN" => {
                // Find all tokens until matching RIGHT_PAREN
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

            "MINUS" => {
                i += 1;

                let mut inner_tokens: Vec<Token> = Vec::new();
                inner_tokens.push(tokens[i].clone());
                let inner_result = parse_more(inner_tokens);
                result.push(format!("(- {})", inner_result.join(" ")));
                /*  i += 1; // Advance to the next token after the first MINUS
                let mut inner_tokens: Vec<Token> = Vec::new();

                // Collect consecutive MINUS tokens

                while i < tokens.len()
                    && (tokens[i].token_type.as_str() == "MINUS"
                        || tokens[i].token_type.as_str() == "LEFT_PAREN")
                {
                    inner_tokens.push(tokens[i].clone());
                    i += 1;
                }

                // Ensure there's a non-MINUS token after the sequence
                if i < tokens.len() {
                    inner_tokens.push(tokens[i].clone());
                    i += 1; // Consume the final token
                } else {
                    panic!("Unexpected end of input after MINUS tokens");
                }

                // Parse the collected tokens
                let inner_result = parse_more(inner_tokens);
                result.push(format!("(- {})", inner_result.join(" ")));*/
            }
            "SLASH" => {
                result.push("/".to_string());
                send_sign = true;
            }
            "STAR" => {
                result.push("*".to_string());

                send_sign = true;
            }

            _ => {
                /*   println!(
                    "unknown token: {} {} {}",
                    token.lexeme, token.literal, token.token_type
                );
                */
            }
        }
        i += 1;
    }
    if send_sign {
        parse_signs(result)
    } else {
        result
    }
}
