use std::env;
use std::fs;
use std::io::{self, Write};

#[quit::main]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
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

            let exit_code = tokenize(&file_contents);

            quit::with_code(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            quit::with_code(64);
        }
    }
}

fn tokenize(content: &str) -> u8 {
    let mut bad = false;
    for char in content.chars() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            '.' => println!("DOT . null"),
            ',' => println!("COMMA , null"),
            '+' => println!("PLUS + null"),
            '-' => println!("MINUS - null"),
            ';' => println!("SEMICOLON ; null"),
            '/' => println!("SLASH / null"),
            _ => {
                // Write errors to stderr instead of stdout
                writeln!(
                    io::stderr(),
                    "[line 1] Error: Unexpected character: {}",
                    char
                )
                .unwrap();
                bad = true;
            }
        }
    }

    println!("EOF  null");
    if bad {
        65
    } else {
        0
    }
}
