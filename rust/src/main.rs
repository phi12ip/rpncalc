use std::io::{self, Write};
use std::convert::TryInto;

#[derive(Debug)]
enum Token {
    Value(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow, 
    Root,
    Floor,
    Ceiling,
    Duplicate,
    ClearAll,
    Pop,
    Help,
    // TODO:
    // Log,
    // RollUp,
    // RollDown,
} 

fn main() {
    welcome();
    let mut stack: Vec<f64> = Vec::new();
    loop {
        let tokens = parse_input(get_input());
        apply_tokens(tokens, &mut stack);
        print_stack(&stack);
    }
}

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");
    input
}

fn parse_input(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let split: Vec<&str> = input.trim().split(' ').collect();

    for seg in split {
        match seg {
            "-" => { tokens.push(Token::Subtract) },
            "+" => { tokens.push(Token::Add) },
            "*" => { tokens.push(Token::Multiply) },
            "/" => { tokens.push(Token::Divide) },

            "pow" | "power" => { tokens.push(Token::Pow) },
            "root" => { tokens.push(Token::Root) },

            "f" | "floor" => { tokens.push(Token::Floor) },
            "c" | "ceiling" => { tokens.push(Token::Ceiling) },

            "clear" => { tokens.push(Token::ClearAll) },
            "d" | "duplicate" => { tokens.push(Token::Duplicate) },
            "rm" | "remove" => { tokens.push(Token::Pop) },
            
            "h" | "help" => { tokens.push(Token::Help) }
            // Raw number
            _ => {
                match seg.parse() {
                    Ok(n) => tokens.push(Token::Value(n)),
                    Err(e) => eprintln!("Not a number: {}\n", e),
                }
            }
        }
    }

    tokens
}

fn apply_tokens(tokens: Vec<Token>, stack: &mut Vec<f64>) {
    for token in tokens {
        match token {
            Token::Value(n) => {
                stack.push(n) 
            },
            Token::Duplicate => {
                let a = stack.pop().unwrap();
                stack.push(a);
                stack.push(a);
            },
            Token::Pop => {
                let _ = stack.pop().unwrap();
            },
            Token::ClearAll => {
                stack.clear();
                welcome();
            },
            Token::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            Token::Subtract => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push( b - a );
            },
            Token::Multiply => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push( a * b);
            },
            Token::Divide => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push( b / a );
            },
            Token::Pow => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push( b.powf(a));
            },
            Token::Root => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push( b.powf(1.0/a));
            },
            Token::Floor => {
                let a = stack.pop().unwrap();
                if a < 0.0 {
                    stack.push(a.floor() + 1.0)
                } else {
                    stack.push(a.floor());
                }
            },
            Token::Ceiling => {
                let a = stack.pop().unwrap();
                if a < 0.0 {
                    stack.push(a.floor() - 1.0)
                } else {
                    stack.push(a.floor() + 1.0);
                }
            },
            Token::Help => {
                print_help();
            }
        }
    }
}

fn print_stack(stack: &Vec<f64>) {
    if stack.len() > 0 {
        let mut x: isize = (stack.len() - 1).try_into().unwrap(); 
        for elem in stack.iter() {
            println!("|[{}]: {}", x, elem);
            x -= 1;
        }
        print!("----------------------\n");
    }
}

fn print_help() {
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

fn welcome() {
    print!("{}[2J", 27 as char);
    println!("Welcome to the Rust RPN Calculator!\n");
}