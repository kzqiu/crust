pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
}

pub struct Expression {
    pub val: i32,
}

pub struct Return {
    pub expr: Option<Expression>,
}

// pub enum NodeType {
//     Program(Program),
//     Function(Function),
//     Expression(Expression),
//     Return(Return),
// }
