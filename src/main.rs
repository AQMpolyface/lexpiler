mod lexer;
mod parser;

use parser::Token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};

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

            let result = parse_more(tokens);
            println!("{}", result);
            quit::with_code(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            quit::with_code(64);
        }
    }
}

fn parse_more(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        match token.token_type.as_str() {
            "TRUE" => result.push_str("true"),
            "FALSE" => result.push_str("false"),
            "NIL" => result.push_str("nil"),
            "NUMBER" | "STRING" => {
                result.push_str(&token.literal);
            }
            "LEFT_PAREN" => {
                let mut inner_tokens = Vec::new();
                let mut paren_count = 1;
                i += 1;

                while i < tokens.len() && paren_count > 0 {
                    match tokens[i].token_type.as_str() {
                        "LEFT_PAREN" => {
                            paren_count += 1;
                            inner_tokens.push(tokens[i].clone());
                        }
                        "RIGHT_PAREN" => {
                            paren_count -= 1;
                            if paren_count > 0 {
                                inner_tokens.push(tokens[i].clone());
                            }
                        }
                        _ => inner_tokens.push(tokens[i].clone()),
                    }
                    i += 1;
                }
                i -= 1; // Adjust for the outer loop increment

                let inner_result = parse_more(inner_tokens);
                result.push_str(&format!("(group {})", inner_result));
            }
            "BANG" => {
                // Look ahead to collect all tokens that should be under this BANG
                let mut remaining_tokens = Vec::new();
                let mut j = i + 1;

                while j < tokens.len() {
                    remaining_tokens.push(tokens[j].clone());
                    j += 1;
                }

                let inner_result = parse_more(remaining_tokens);
                result.push_str(&format!("(! {})", inner_result));
                break; // Exit after processing all remaining tokens
            }
            "MINUS" => {
                // Similar to BANG handling
                i += 1;
                if i < tokens.len() {
                    let mut inner_tokens = vec![tokens[i].clone()];
                    let inner_result = parse_more(inner_tokens);
                    result.push_str(&format!("(- {})", inner_result));
                }
            }
            _ => {}
        }
        i += 1;
    }

    result
}
