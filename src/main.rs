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
        let token_type = token.token_type.as_str();

        match token_type {
            "TRUE" => result.push_str("true"),
            "FALSE" => result.push_str("false"),
            "NIL" => result.push_str("nil"),
            "NUMBER" | "STRING" => {
                result.push_str(&token.literal);
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
                result.push_str(&format!("(group {})", inner_result));
            }
            "BANG" => {
                let mut nested_result = String::new();
                let mut count = 1; // Starting with the first '!'

                // Process subsequent '!' tokens
                while let Some(next_token) = tokens.get(i + count) {
                    if next_token.lexeme.as_str() == "!" {
                        nested_result.push_str("!");
                        count += 1; // Skip the next '!' token
                    } else {
                        break; // Stop if it's not another '!'
                    }
                }

                // Now process the token after the sequence of '!'
                if let Some(token) = tokens.get(i + count) {
                    let token_type = token.lexeme.as_str();
                    nested_result.push_str(&format!(" ({})", token_type));
                }

                i += count; // Move the index forward by the number of '!' processed
                result.push_str(&format!("(! {})", nested_result));
            }
            "MINUS" => {
                let token1 = &tokens[i + 1];
                let token_type1 = token1.literal.as_str();

                i += 1;
                result.push_str(&format!("(- {})", token_type1));
            }
            _ => {}
        }
        i += 1;
    }

    result
}
