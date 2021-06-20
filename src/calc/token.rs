#[derive(Debug)]
pub enum Token {
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
