pub mod stack;
pub mod token;

use crate::welcome;
use crate::cli::print_help;
use token::Token;

pub fn apply_tokens(tokens: Vec<Token>, stack: &mut Vec<f64>) {
    for token in tokens {
        match token {
            Token::Value(n) => stack.push(n),
            Token::Duplicate => {
                let a = stack.pop().unwrap();
                stack.push(a);
                stack.push(a);
            }
            Token::Pop => {
                let _ = stack.pop().unwrap();
            }
            Token::ClearAll => {
                stack.clear();
                // TODO: do this some other way
                welcome();
            }
            Token::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Subtract => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            Token::Multiply => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            Token::Divide => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            Token::Pow => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b.powf(a));
            }
            Token::Root => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b.powf(1.0 / a));
            }
            Token::Floor => {
                let a = stack.pop().unwrap();
                if a < 0.0 {
                    stack.push(a.floor() + 1.0)
                } else {
                    stack.push(a.floor());
                }
            }
            Token::Ceiling => {
                let a = stack.pop().unwrap();
                if a < 0.0 {
                    stack.push(a.floor() - 1.0)
                } else {
                    stack.push(a.floor() + 1.0);
                }
            }
            Token::Help => {
                // TODO: do this some other way
                print_help();
            }
        }
    }
}
