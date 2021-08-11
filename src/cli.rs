use crate::calc::Stack;
use crate::calc::Token;
use crate::calc::Token::*;

/// Print stdout char to clear screen on POSIX terminals
fn clear_screen() {
    // print!("{}[2J", 27 as char);
    for _ in 1..10 {
        println!("");
    }
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
    let split: Vec<&str>;
    split = input.trim().split(' ').collect();

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

/// Print out usage to user
pub fn print_help() {
    let message = "
    Help:
_____________________________________________
 Any number gets put on the top of the stack.

 Op           Function
----         ----------
 +              Add
 -              Subtract
 *              Multiply
 /              Divide
 pow            Power
 root           top of stack is exponent
 f              Floor
 c              Ceiling
 d              Duplicate
 clear          Clear All
 rm             Pop
    ";

    println!("{}", message);
}

/// Print out the content of the stack to stdout
pub fn print_stack(stack: &Stack) {
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

/// Print welcome message to user
pub fn welcome() {
    clear_screen();
    println!("Welcome to the Rust RPN Calculator!\n");
}
