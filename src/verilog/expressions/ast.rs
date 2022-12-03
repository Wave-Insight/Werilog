use crate::verilog::general::ast::Attr;














#[derive(Debug)]
pub enum ConstantOrNot<T> {
    Constant(T),
    NotConst(T),
}

#[derive(Debug)]
pub enum ConstantExpression {
    ConstantPrimary(ConstantPrimary),
    Unary(String, Vec<Attr>, ConstantPrimary),
    Binary(Box<ConstantExpression>, String, Vec<Attr>, Box<ConstantExpression>),
    Condition(Box<ConstantExpression>, Vec<Attr>, Box<ConstantExpression>, Box<ConstantExpression>),
}

#[derive(Debug)]
pub enum ConstantRangeExpression {
    Single(ConstantExpression),
    To((ConstantExpression, ConstantExpression)),
    Add((ConstantExpression, ConstantExpression)),
    Sub((ConstantExpression, ConstantExpression)),
}

#[derive(Debug)]
pub enum ConstantMintypmaxExpression {
    ConstantExpression(ConstantExpression),
    Three(ConstantExpression, ConstantExpression, ConstantExpression),
}

#[derive(Debug)]
pub enum MintypmaxExpression {
    Expression(Expression),
    Three(Expression, Expression, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Primary(Primary),
    Unary(String, Vec<Attr>, Primary),
    Binary(Box<Expression>, String, Vec<Attr> , Box<Expression>),
    Condition(Box<Expression>, Vec<Attr>, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum RangeExpression {
    Single(Expression),
    To((ConstantExpression, ConstantExpression)),
    Add((Expression, ConstantExpression)),
    Sub((Expression, ConstantExpression)),
}

#[derive(Debug)]
pub enum ConstantPrimary {
    Number(Number),
    Parameter(String),
}

#[derive(Debug)]
pub enum ModulePathPrimary {
    Number(Number),
}

#[derive(Debug)]
pub enum Primary {
    Number(Number),
    //TODO:Hierarchical(String, Box<Option<(Vec<Expression>, RangeExpression)>>),
    Hierarchical(String, Box<Option<Vec<RangeExpression>>>),
    Concatenation(Vec<Expression>),
    MultipleConcatenation(ConstantExpression, Vec<Expression>),

    MintypmaxExpression(Box<MintypmaxExpression>),
    String(String),
}

#[derive(Debug)]
pub enum NetLvalue {
    VariableRef((String, Option<Vec<ConstantRangeExpression>>)),//ConstantRangeExpression
    Concatenation(Vec<NetLvalue>),
}

#[derive(Debug)]
pub enum VariableLvalue {
    VariableRef((String, Option<(Vec<Expression>, RangeExpression)>)),
    Concatenation(Vec<VariableLvalue>),
}

#[derive(Debug)]
pub enum Number {
    Decimal(String),
    Octal(String),
    Binary(String),
    Hex(String),
    Real(String),
}
