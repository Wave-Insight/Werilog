
use crate::verilog::{general::ast::Attr, declaration::ast::{NetType, Range, OutputDeclaration}, expressions::ast::ConstantRangeExpression};













// A 1.3 Module parameters and ports

pub struct Port(pub Option<String>, pub Option<Vec<(String, Option<ConstantRangeExpression>)>>);

#[derive(Debug)]
pub enum PortDeclaration {
    Inout(Vec<Attr>, (Option<NetType>, bool, Option<Range>, Vec<String>)),
    Input(Vec<Attr>, (Option<NetType>, bool, Option<Range>, Vec<String>)),
    Output(Vec<Attr>, OutputDeclaration),
}
