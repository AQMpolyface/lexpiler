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
    //let chars: Vec<char> = content.chars().collect();
    let mut bad = false;
    let mut token = "";

    let mut skip_next = false;
    let mut skipidi = false;
    for (num, char) in content.chars().enumerate() {
        if skip_next {
            (bad, skipidi, token) = tokenize_more(char);
            skip_next = skipidi;
        }
    }

    println!("EOF  null");

    if bad {
        65
    } else {
        0
    }
}

fn tokenize_more(char: char) -> (bool, bool, &str) {
    let mut bad = false;
    let mut skipidi = false;
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
        '=' => {
            if num + 1 < chars.len() && chars[num + 1] == '=' {
                println!("EQUAL_EQUAL == null");
                // Skip the next iteration since we've handled both characters
                if num + 1 < chars.len() {
                    continue;
                }
            } else {
                println!("EQUAL = null");
            }
        }

        _ => {
            if !char.is_whitespace() {
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
    (bad,)
}
