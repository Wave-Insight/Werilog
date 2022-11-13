
use crate::verilog::{general::ast::Attr, declaration::ast::{NetType, Range, OutputDeclaration, ParameterDeclaration, NetDeclaration, VariableDeclaration, RealDeclaration}, expressions::ast::ConstantRangeExpression, behavioral_statements::ast::{NetAssign, Statement}};





// A 1.2 verilog source


#[derive(Debug)]
pub enum ModuleDeclaration {
    Ports(Vec<Attr>, String, Option<Vec<ParameterDeclaration>>, Vec<Port>, Vec<ModuleItem>),
    NonPorts(Vec<Attr>, String, Option<Vec<ParameterDeclaration>>, Option<Vec<PortDeclaration>>, Vec<NonPortModuleItem>),
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

#[derive(Debug)]
pub enum ModuleGenetateItem {
    ModuleItemDeclaration(Vec<Attr>, ModuleItemDeclaration),


    ContinuousAssign(Vec<Attr>, Vec<NetAssign>),



    Initial(Vec<Attr>, Statement),
    Always(Vec<Attr>, Statement),
}

#[derive(Debug)]
pub enum ModuleItemDeclaration {
    Net(NetDeclaration),
    Reg(bool, Option<Range>, Vec<VariableDeclaration>),
    Integer(Vec<VariableDeclaration>),
    Real(Vec<RealDeclaration>),
    RealTime(Vec<RealDeclaration>),
    Event(Vec<(String, Vec<Range>)>),
}

#[derive(Debug)]
pub enum NonPortModuleItem {
    ModuleGenetateItem(ModuleGenetateItem),

}
