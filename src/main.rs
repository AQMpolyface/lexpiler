use std::env;
use std::fs;
use std::io::{self, Write};
static mut BAD: bool = false;
static mut LINE: u32 = 1;
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
        }
    }
}

fn tokenize(content: &str) -> u8 {
    //let chars: Vec<char> = content.chars().collect();
    let mut token = String::new();
    let chars: Vec<char> = content.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '=' {
            if i + 1 < chars.len() && chars[i + 1] == '=' {
                token = String::from("EQUAL_EQUAL == null");
                println!("{}", token);
                i += 2; // Skip the current and the next character
                continue;
            } else {
                token = String::from("EQUAL = null");
                println!("{}", token);
                i += 1; // Skip the current and the next character
                continue;
            }
        }
        if chars[i] == '!' {
            if i + 1 < chars.len() && chars[i + 1] == '=' {
                token = String::from("BANG_EQUAL != null");
                println!("{}", token);
                i += 2; // Skip the current and the next character
                continue;
            } else {
                token = String::from("BANG ! null");
                println!("{}", token);
                i += 1; // Skip the current and the next character
                continue;
            }
        }
        if chars[i] == '<' {
            if i + 1 < chars.len() && chars[i + 1] == '=' {
                token = String::from("LESS_EQUAL <= null");
                println!("{}", token);
                i += 2; // Skip the current and the next character
                continue;
            } else {
                token = String::from("LESS < null");
                println!("{}", token);
                i += 1; // Skip the current and the next character
                continue;
            }
        }
        if chars[i] == '>' {
            if i + 1 < chars.len() && chars[i + 1] == '=' {
                token = String::from("GREATER_EQUAL >= null");
                println!("{}", token);
                i += 2; // Skip the current and the next character
                continue;
            } else {
                token = String::from("GREATER > null");
                println!("{}", token);
                i += 1; // Skip the current and the next character
                continue;
            }
        }
        if chars[i] == '/' {
            if i + 1 < chars.len() && chars[i + 1] == '/' {
                //token = String::from("");
                //println!("{}", token);
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }

                continue;
            } else {
                token = String::from("SLASH / null");
                println!("{}", token);
                i += 1; // Skip the current and the next character
                continue;
            }
        }
        token = tokenize_more(chars[i]);

        if !token.is_empty() {
            println!("{}", token);
        }
        i += 1;
    }

    println!("EOF  null");
    unsafe {
        if BAD {
            65
        } else {
            0
        }
    }
}

fn tokenize_more(char: char) -> String {
    let mut token = "";
    match char {
        '(' => token = "LEFT_PAREN ( null",
        ')' => token = "RIGHT_PAREN ) null",
        '{' => token = "LEFT_BRACE { null",
        '}' => token = "RIGHT_BRACE } null",
        '*' => token = "STAR * null",
        '.' => token = "DOT . null",
        ',' => token = "COMMA , null",
        '+' => token = "PLUS + null",
        '-' => token = "MINUS - null",
        ';' => token = "SEMICOLON ; null",
        '/' => token = "SLASH / null",
        '\n' => unsafe {
            LINE += 1;
        },
        _ => {
            if !char.is_whitespace() {
                unsafe {
                    eprintln!("[line {}] Error: Unexpected character: {}", LINE, char);
                    BAD = true;
                }
            }
        }
    }
    String::from(token)
}
