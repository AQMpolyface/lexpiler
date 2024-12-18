mod lexer;
mod parser;

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

            // Print the tokens for verification
            for token in tokens {
                //println!("{} {} {}", token.token_type, token.lexeme, token.literal);
                match token.token_type.as_str() {
                    "TRUE" => println!("true"),
                    "FALSE" => println!("false"),
                    "NIL" => println!("nil"),
                    _ => {}
                }
            }

            quit::with_code(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            quit::with_code(64);
        }
    }
}
