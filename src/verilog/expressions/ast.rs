use crate::verilog::general::ast::Attr;














pub enum ConstantOrNot<T> {
    Constant(T),
    NotConst(T),
}

pub enum ConstantExpression {
    ConstantPrimary(ConstantPrimary),
}

pub enum ConstantRangeExpression {
    Single(ConstantExpression),
    To((ConstantExpression, ConstantExpression)),
    Add((ConstantExpression, ConstantExpression)),
    Sub((ConstantExpression, ConstantExpression)),
}

pub enum MintypmaxExpression {
    ConstantExpression(ConstantExpression),
    Three((ConstantExpression, ConstantExpression, ConstantExpression))
}

pub enum Expression {
    Primary(Primary),
    Condition((Box<Expression>, Vec<Attr>, Box<Expression>, Box<Expression>)),
}

pub enum RangeExpression {
    Single(Expression),
    To((ConstantExpression, ConstantExpression)),
    Add((Expression, ConstantExpression)),
    Sub((Expression, ConstantExpression)),
}

pub enum ConstantPrimary {
    Number(Number),
    Parameter(),
}

pub enum ModulePathPrimary {
    Number(Number),
}

pub enum Primary {
    Number(Number),
}

pub enum NetLvalue {
    VariableRef((String, Option<(Vec<ConstantExpression>, ConstantRangeExpression)>)),
    Concatenation(Vec<NetLvalue>),
}

pub enum VariableLvalue {
    VariableRef((String, Option<(Vec<Expression>, RangeExpression)>)),
    Concatenation(Vec<VariableLvalue>),
}

pub enum Number {
    Decimal(String),
    Octal(String),
    Binary(String),
    Hex(String),
    Real(String),
}
