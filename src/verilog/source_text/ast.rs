
use crate::verilog::{general::ast::Attr, declaration::ast::{NetType, Range, OutputDeclaration, ParameterDeclaration}, expressions::ast::ConstantRangeExpression};





// A 1.2 verilog source


#[derive(Debug)]
pub enum ModuleDeclaration {
    Ports(Vec<Attr>, String, Option<Vec<ParameterDeclaration>>, Vec<Port>, Vec<ModuleItem>),
}







// A 1.3 Module parameters and ports

#[derive(Debug)]
pub struct Port(pub Option<String>, pub Option<Vec<(String, Option<ConstantRangeExpression>)>>);

#[derive(Debug)]
pub enum PortDeclaration {
    Inout(Vec<Attr>, (Option<NetType>, bool, Option<Range>, Vec<String>)),
    Input(Vec<Attr>, (Option<NetType>, bool, Option<Range>, Vec<String>)),
    Output(Vec<Attr>, OutputDeclaration),
}

// A 1.4 module items
#[derive(Debug)]
pub enum ModuleItem {
    PortDeclaration(PortDeclaration),
}
