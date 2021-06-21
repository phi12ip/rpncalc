use crate::cli;
use crate::calc::token::Token;
use crate::calc::token::Token::*;

pub struct Stack {
    elements: Vec<f64>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            elements: Vec::<f64>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.elements.iter()
    }

    pub fn apply(&mut self, tokens: Vec<Token>) {
        for token in tokens {
            match token {
                Value(n) => self.elements.push(n),
                Duplicate => {
                    let a = self.elements.pop().unwrap();
                    self.elements.push(a);
                    self.elements.push(a);
                }
                Pop => {
                    let _ = self.elements.pop().unwrap();
                }
                ClearAll => {
                    self.elements.clear();
                    // TODO: do this some other way
                    cli::welcome();
                }
                Add => {
                    let a = self.elements.pop().unwrap();
                    let b = self.elements.pop().unwrap();
                    self.elements.push(a + b);
                }
                Subtract => {
                    let a = self.elements.pop().unwrap();
                    let b = self.elements.pop().unwrap();
                    self.elements.push(b - a);
                }
                Multiply => {
                    let a = self.elements.pop().unwrap();
                    let b = self.elements.pop().unwrap();
                    self.elements.push(a * b);
                }
                Divide => {
                    let a = self.elements.pop().unwrap();
                    let b = self.elements.pop().unwrap();
                    self.elements.push(b / a);
                }
                Pow => {
                    let a = self.elements.pop().unwrap();
                    let b = self.elements.pop().unwrap();
                    self.elements.push(b.powf(a));
                }
                Root => {
                    let a = self.elements.pop().unwrap();
                    let b = self.elements.pop().unwrap();
                    self.elements.push(b.powf(1.0 / a));
                }
                Floor => {
                    let a = self.elements.pop().unwrap();
                    if a < 0.0 {
                        self.elements.push(a.floor() + 1.0)
                    } else {
                        self.elements.push(a.floor());
                    }
                }
                Ceiling => {
                    let a = self.elements.pop().unwrap();
                    if a < 0.0 {
                        self.elements.push(a.floor() - 1.0)
                    } else {
                        self.elements.push(a.floor() + 1.0);
                    }
                }
                Token::Help => {
                    // TODO: do this some other way
                    cli::print_help();
                }
            }
        }
    }
}
