use crate::verilog::{expressions::ast::{VariableLvalue, Expression, NetLvalue}, general::ast::Attr};


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

#[derive(Debug)]
pub struct NetAssign(pub NetLvalue, pub Expression);

pub struct Statement {
    pub attribute: Vec<Attr>,
    pub item: StatementItem,
}
impl Statement {
    pub fn blocking_assignment(a: (Vec<Attr>, BlockAssign)) -> Self {
        Self { attribute: a.0, item: StatementItem::BlockAssign(a.1)}
    }
}
pub enum StatementItem {
    BlockAssign(BlockAssign)
}

pub struct StatementOrNull {
    pub attribute: Vec<Attr>,
    pub item: Option<StatementItem>,
}
impl StatementOrNull {
    pub fn from_statement(s: Statement) -> Self {
        Self { attribute: s.attribute, item: Some(s.item) }
    }
    pub fn from_attr(a: Vec<Attr>) -> Self {
        Self { attribute: a, item: None }
    }
}

// timing ctrl

pub enum EventCtrl {
    HierarchicalEvent(String),
    EventExpression(EventExpression),
    Auto1,
    Auto2,
}

pub enum EventExpression {
    Expression(Expression),
    Posedge(Expression),
    Negedge(Expression),
    Or((Box<EventExpression>, Box<EventExpression>)),
    And((Box<EventExpression>, Box<EventExpression>)),
}

// conditional
pub struct Conditional {
    pub if_state: Vec<(Expression, StatementOrNull)>,
    pub else_state: Option<StatementOrNull>,
}

//case
pub enum CaseState {
    Case((Expression, Vec<CaseItem>)),
    Casez((Expression, Vec<CaseItem>)),
    Casex((Expression, Vec<CaseItem>)),
}

pub enum CaseItem {
    Express((Vec<Expression>, StatementOrNull)),
    Default(StatementOrNull)
}

//looping
pub enum Loop {
    Forever(Statement),
    Repeat((Expression, Statement)),
    While((Expression, Statement)),
    For((VariableAssign, Expression, VariableAssign, Statement)),
}
