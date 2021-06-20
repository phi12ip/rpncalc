use crate::Token;
use crate::calc::stack::Stack;

pub struct State {
    stack: Stack,
    history: Vec<Event>,
}

pub struct Event {
    stack: Stack,
    tokens: Vec<Token>,
}

pub fn apply_tokens(state: State, tokens: Vec<Token>) -> State {
    
}