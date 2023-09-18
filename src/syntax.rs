struct Identifier(String);

enum Enclosure {
    ParenthForm,
    // list_display,
    // dict_display,
    // set_display,
    // generator_expression,
    // yield_atom,
}

enum Literal {
    Bool(bool),
    Number(i32),
    String(String),
}

enum Value {
    Literal(Literal),
}

enum ArithmeticOperation {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
}

enum Expr {
    Identifier(Identifier),
    Literal(Literal),
    ArithmeticOperation(Box<ArithmeticOperation>),
}

enum Stmt {
    Assignment(AssignmentStmt),
}

struct AssignmentStmt(Identifier, Expr);

struct Program(Vec<Stmt>, Expr);
