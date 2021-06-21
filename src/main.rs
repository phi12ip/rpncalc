mod cli;
mod calc;

fn main() {
    cli::welcome();
    let mut stack = calc::stack::Stack::new();
    loop {
        let user_input = cli::get_input();
        let tokens = cli::parse_input(user_input);
        stack.apply(tokens);
        cli::print_stack(&stack);
    }
}
