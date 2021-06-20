mod cli;
use cli::get_input;
use cli::parse_input;
use cli::print_stack;
use cli::welcome;

mod calc;
use calc::apply_tokens;
use calc::stack::new_stack;
use calc::token::Token;

fn main() {
    welcome();
    let mut stack = new_stack();
    loop {
        let tokens = parse_input(get_input());
        apply_tokens(tokens, &mut stack);
        print_stack(&stack);
    }
}
