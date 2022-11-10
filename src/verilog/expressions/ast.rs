use crate::verilog::general::ast::Attr;














#[derive(Debug)]
pub enum ConstantOrNot<T> {
    Constant(T),
    NotConst(T),
}

#[derive(Debug)]
pub enum ConstantExpression {
    ConstantPrimary(ConstantPrimary),
}

#[derive(Debug)]
pub enum ConstantRangeExpression {
    Single(ConstantExpression),
    To((ConstantExpression, ConstantExpression)),
    Add((ConstantExpression, ConstantExpression)),
    Sub((ConstantExpression, ConstantExpression)),
}

#[derive(Debug)]
pub enum MintypmaxExpression {
    ConstantExpression(ConstantExpression),
    Three((ConstantExpression, ConstantExpression, ConstantExpression))
}

#[derive(Debug)]
pub enum Expression {
    Primary(Primary),
    Condition((Box<Expression>, Vec<Attr>, Box<Expression>, Box<Expression>)),
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
    Parameter(),
}

#[derive(Debug)]
pub enum ModulePathPrimary {
    Number(Number),
}

#[derive(Debug)]
pub enum Primary {
    Number(Number),
}

#[derive(Debug)]
pub enum NetLvalue {
    VariableRef((String, Option<(Vec<ConstantExpression>, ConstantRangeExpression)>)),
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
