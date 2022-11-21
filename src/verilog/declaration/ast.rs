use crate::verilog::{expressions::ast::{ConstantExpression, MintypmaxExpression, ConstantMintypmaxExpression}, general::ast::Attr};


#[derive(Debug)]
pub enum Delay3 {
    Value(String),
    Expr1(MintypmaxExpression),
    Expr2(MintypmaxExpression, MintypmaxExpression),
    Expr3(MintypmaxExpression, MintypmaxExpression, MintypmaxExpression),
}

#[derive(Debug)]
pub enum Delay2 {
    Value(String),
    Expr1(MintypmaxExpression),
    Expr2(MintypmaxExpression, MintypmaxExpression),
}

#[derive(Debug)]
pub enum ParameterDeclaration {
    Notype(bool, Option<Range>, Vec<(String, ConstantMintypmaxExpression)>),
    Integer(Vec<(String, ConstantMintypmaxExpression)>),
    Real(Vec<(String, ConstantMintypmaxExpression)>),
    Realtime(Vec<(String, ConstantMintypmaxExpression)>),
    Time(Vec<(String, ConstantMintypmaxExpression)>),
}


#[derive(Debug)]
pub enum OutputDeclaration {
    Wire((((Option<NetType>, bool), Option<Range>), Vec<String>)),
    Reg(((bool, Option<Range>), Vec<(String, Option<ConstantExpression>)>)),
    Others((OutputVariableType, Vec<(String, Option<ConstantExpression>)>)),
}

#[derive(Debug)]
pub struct Range(pub ConstantExpression, pub ConstantExpression);

#[derive(Debug)]
pub enum NetType {
    Supply0,
    Supply1,
    Tri,
    Triand,
    Trior,
    Tri0,
    Tri1,
    Uwire,
    Wire,
    Wand,
    Wor,
}

#[derive(Debug)]
pub enum OutputVariableType {
    Integer,
    Time,
}

#[derive(Debug)]
pub enum NetDeclaration {
    Simple(NetType, bool, Vec<(String, Vec<Range>)>),

    Range(NetType, Option<bool>, bool, Range, Vec<(String, Vec<Range>)>),
}

#[derive(Debug)]
pub enum RealDeclaration {
    Dimension(String, Vec<Range>),
    ConstExp(String, ConstantExpression),
}

#[derive(Debug)]
pub enum VariableDeclaration {
    Dimension(String, Vec<Range>),
    ConstExp(String, ConstantExpression),
}

// block item
#[derive(Debug)]
pub enum BlockItemDeclaration {
    Reg(Vec<Attr>, bool, Option<Range>, Vec<(String, Vec<Range>)>),
    Integer(Vec<Attr>, Vec<(String, Vec<Range>)>),
    Time(Vec<Attr>, Vec<(String, Vec<Range>)>),
    Real(Vec<Attr>, Vec<(String, Vec<Range>)>),
    Realtime(Vec<Attr>, Vec<(String, Vec<Range>)>),
    Event(Vec<Attr>, Vec<(String, Vec<Range>)>),
}
