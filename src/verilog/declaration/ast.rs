use crate::verilog::expressions::ast::{ConstantExpression, ConstantOrNot, MintypmaxExpression};


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

