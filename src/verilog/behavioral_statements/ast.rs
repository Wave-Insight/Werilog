use crate::verilog::expressions::ast::{VariableLvalue, Expression, NetLvalue};


pub struct BlockAssign(pub VariableLvalue, pub Expression);

pub struct NonBlockAssign(pub VariableLvalue, pub Expression);

pub enum ProceduralContinuous {
    Assign(VariableAssign),
    Deassign(VariableLvalue),
    ForceVar(VariableAssign),
    ForceNet(NetAssign),
    ReleaseVar(VariableLvalue),
    ReleaseNet(NetLvalue),
}

pub struct VariableAssign(pub VariableLvalue, pub Expression);

pub struct NetAssign(pub NetLvalue, pub Expression);
