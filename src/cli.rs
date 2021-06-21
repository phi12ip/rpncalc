use crate::Token;
use crate::Token::*;

/// Print stdout char to clear screen on POSIX terminals
fn clear_screen() {
    print!("{}[2J", 27 as char);
}

/// Get input from the user to effect the stack
pub fn get_input() -> String {
    use std::io::{self, Write};

    // Print prompt and make sure it's flushed
    print!("> ");
    io::stdout().flush().unwrap();

    // Read user input string
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input
}

/// Parse a user input string into separate Tokens that can be applied to the stack
pub fn parse_input(input: String) -> Vec<Token> {
    // Create new token collection
    let mut tokens: Vec<Token> = Vec::new();

    // Split the input string on spaces and pack into a collection
    let split: Vec<&str> = input.trim().split(' ').collect();

    // For each segment, push the appropriate token
    for seg in split {
        match seg {
            // Regular operators
            "/" => tokens.push(Divide),
            "*" => tokens.push(Multiply),
            "-" => tokens.push(Subtract),
            "+" => tokens.push(Add),

            // Special operators
            "pow" | "power" => tokens.push(Pow),
            "root" => tokens.push(Root),

            // Rounding
            "f" | "floor" => tokens.push(Floor),
            "c" | "ceiling" => tokens.push(Ceiling),

            // Stack management
            "clear" => tokens.push(ClearAll),
            "d" | "duplicate" | "e" | "enter" => tokens.push(Duplicate),
            "rm" | "remove" => tokens.push(Pop),

            // CLI
            "h" | "help" => tokens.push(Help),

            // Raw number
            _ => match seg.parse() {
                Ok(n) => tokens.push(Value(n)),
                Err(e) => eprintln!("Not a number: {}\n", e),
            },
        }
    }

    tokens
}

pub fn print_help() {
    println!("Help:");
    println!("--------------------------------------------");
    println!("Any number gets put on the top of the stack, \nindicated by [0]\n");
    println!("Op\t\tFunction");
    println!("--\t\t--------");
    println!("+\t\tAdd");
    println!("-\t\tSubtract");
    println!("*\t\tMultiply");
    println!("/\t\tDivide");
    println!("pow\t\tPow ([1]^[0])");
    println!("root\t\tRoot ([1] root [0]) ");
    println!("f\t\tFloor");
    println!("c\t\tCeiling");
    println!("d\t\tDuplicate");
    println!("clear\t\tClearAll");
    println!("rm\t\tPop");
    println!("--------------------------------------------");
}

pub fn print_stack(stack: &Vec<f64>) {
    clear_screen();
    if stack.len() > 0 {
        let mut pointer: usize = stack.len();
        let mut max_stack = 8;

        // Print empty stack spots
        while max_stack > pointer {
            println!("{:2}:      ", max_stack);
            max_stack -= 1;
        }

        // Print stack elements
        for element in stack.iter() {
            println!("{:2}:      {:10}", pointer, element);
            pointer -= 1;
        }
        print!(" __________________\n");
    }
}

pub fn welcome() {
    clear_screen();
    println!("Welcome to the Rust RPN Calculator!\n");
}
