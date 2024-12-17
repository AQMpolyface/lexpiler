use std::collections::HashSet;
use std::convert::identity;
use std::env;
use std::fs;
use std::io::{self, Write};
//flag to exit withcode 65 compile error
static mut BAD: bool = false;
//tracking the number of lines
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
    let mut token = String::new();
    let chars: Vec<char> = content.chars().collect();

    let mut i = 0;
    let valid_chars: HashSet<char> = ['(', ')', '{', '}', '*', '.', ',', '+', '-', ';', '/', '\n']
        .iter()
        .cloned()
        .collect();

    let keywords: HashSet<&str> = [
        "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return",
        "super", "this", "true", "var", "while",
    ]
    .iter()
    .cloned()
    .collect();
    while i < chars.len() {
        if chars[i] == '\n' {
            unsafe {
                LINE += 1;
            }
        }
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        } else if chars[i] == '=' {
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
        } else if chars[i] == '!' {
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
        } else if chars[i] == '<' {
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
        } else if chars[i] == '>' {
            if i + 1 < chars.len() && chars[i + 1] == '=' {
                token = String::from("GREATER_EQUAL >= null");
                println!("{}", token);
                i += 2;
                continue;
            } else {
                token = String::from("GREATER > null");
                println!("{}", token);
                i += 1; // Skip the current and the next character
                continue;
            }
        } else if chars[i] == '/' {
            if i + 1 < chars.len() && chars[i + 1] == '/' {
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
        } else if chars[i] == '"' {
            let mut string_vec = String::new();
            i += 1; // Start after the opening quote
            let mut is_terminated = false; // Flag to check if the string is terminated

            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\n' {
                    // Handle the case where a newline is encountered before the closing quote
                    unsafe {
                        eprintln!("[line {}] Error: Unterminated string.", LINE);
                        BAD = true;
                    }
                    break; // Exit the loop, don't push the string
                }
                string_vec.push(chars[i]);
                i += 1;
            }
            if i < chars.len() && chars[i] == '"' {
                i += 1; // Skip the closing quote
                is_terminated = true;
            }

            if is_terminated {
                token = format!("STRING \"{}\" {}", string_vec, string_vec);
                println!("{}", token);
            } else {
                unsafe {
                    eprintln!("[line {}] Error: Unterminated string.", LINE);
                    BAD = true;
                }
            }
            continue;
        } else if valid_chars.contains(&chars[i]) {
            token = tokenize_more(chars[i]);
            if !token.is_empty() {
                println!("{}", token);
            }
            i += 1;
            continue;
        } else if chars[i].is_numeric() {
            let mut numbers = String::new();
            //var to track weather the number has multiple points
            //let mut has_point = false;
            while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                numbers.push(chars[i]);
                i += 1;
            }
            /*   if numbers.ends_with(".") {
                unsafe {
                    eprintln!(
                        "[line {}] Error: Unterminated number. Numbers can't end with \".\"",
                        LINE
                    );
                    BAD = true;
                }
            }*/
            if numbers.contains(".") {
                let parts: Vec<&str> = numbers.split('.').collect();
                if parts.len() == 2 && parts[1].chars().all(|c| c == '0') {
                    println!("NUMBER {} {}.0", numbers, parts[0]);
                } else {
                    // Otherwise keep the original number
                    println!("NUMBER {} {}", numbers, numbers);
                }
            } else {
                println!("NUMBER {} {}.0", numbers, numbers);
            }
            continue;
        } else if chars[i].is_alphabetic()
            || chars[i] == '_' && !chars[i].is_whitespace() && !valid_chars.contains(&chars[i])
        {
            let mut identifier = String::new();
            while i < chars.len() && !chars[i].is_whitespace() && !valid_chars.contains(&chars[i]) {
                identifier.push(chars[i]);
                i += 1;
            }
            if keywords.contains(identifier.as_str()) {
                let temp_token = check_word(identifier.as_str());
                println!("{}", temp_token);
            } else {
                println!("IDENTIFIER {} null", identifier);
            }
            continue;
        } else {
            unsafe {
                eprintln!("[line {}] Error: Unexpected character: {}", LINE, chars[i]);
                BAD = true;
                i += 1;
            }
        }
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
fn check_word(word: &str) -> &str {
    match word {
        "and" => "AND and null",
        "class" => "CLASS class null",
        "else" => "ELSE else null",
        "false" => "FALSE false null",
        "for" => "FOR for null",
        "fun" => "FUN fun null",
        "if" => "IF if null",
        "nil" => "NIL nil null",
        "or" => "OR or null",
        "print" => "PRINT print null",
        "return" => "RETURN return null",
        "super" => "SUPER super null",
        "this" => "THIS this null",
        "true" => "TRUE true null",
        "var" => "VAR var null",
        "while" => "WHILE while null",

        _ => "",
    }
}
