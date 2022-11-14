use crate::verilog::{expressions::ast::{ConstantExpression, ConstantOrNot, MintypmaxExpression}, general::ast::Attr};


#[derive(Debug)]
pub enum Delay3 {
    Value(String),
    Expr1(ConstantOrNot<MintypmaxExpression>),
    Expr2(ConstantOrNot<MintypmaxExpression>, ConstantOrNot<MintypmaxExpression>),
    Expr3(ConstantOrNot<MintypmaxExpression>, ConstantOrNot<MintypmaxExpression>, ConstantOrNot<MintypmaxExpression>),
}

#[derive(Debug)]
pub enum Delay2 {
    Value(String),
    Expr1(ConstantOrNot<MintypmaxExpression>),
    Expr2(ConstantOrNot<MintypmaxExpression>, ConstantOrNot<MintypmaxExpression>),
}

#[derive(Debug)]
pub enum ParameterDeclaration {
    Notype(bool, Option<Range>, Vec<(String, ConstantOrNot<MintypmaxExpression>)>),
    Integer(Vec<(String, ConstantOrNot<MintypmaxExpression>)>),
    Real(Vec<(String, ConstantOrNot<MintypmaxExpression>)>),
    Realtime(Vec<(String, ConstantOrNot<MintypmaxExpression>)>),
    Time(Vec<(String, ConstantOrNot<MintypmaxExpression>)>),
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
