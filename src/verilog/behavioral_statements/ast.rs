use crate::verilog::{expressions::ast::{VariableLvalue, Expression, NetLvalue, MintypmaxExpression}, general::ast::Attr, declaration::ast::BlockItemDeclaration};

#[derive(Debug)]
pub struct BlockAssign(pub VariableLvalue, pub Option<DelayOrEventCtrl>, pub Expression);

#[derive(Debug)]
pub struct NonBlockAssign(pub VariableLvalue, pub Option<DelayOrEventCtrl>, pub Expression);

#[derive(Debug)]
pub enum ProceduralContinuous {
    Assign(VariableAssign),
    Deassign(VariableLvalue),
    ForceVar(VariableAssign),
    ForceNet(NetAssign),
    ReleaseVar(VariableLvalue),
    ReleaseNet(NetLvalue),
}

#[derive(Debug)]
pub struct VariableAssign(pub VariableLvalue, pub Expression);

#[derive(Debug)]
pub struct NetAssign(pub NetLvalue, pub Expression);

#[derive(Debug)]
pub struct SeqBlock {
    pub block_item: Option<(String, Vec<BlockItemDeclaration>)>,
    pub statement: Vec<Statement>,
}
impl SeqBlock {
    pub fn new(input: (Option<(String, Vec<BlockItemDeclaration>)>, Vec<Statement>)) -> Self {
        Self {
            block_item: input.0,
            statement: input.1,
        }
    }
}


#[derive(Debug)]
pub struct Statement {
    pub attribute: Vec<Attr>,
    pub item: StatementItem,
}
impl Statement {
    pub fn blocking_assignment(a: (Vec<Attr>, BlockAssign)) -> Self {
        Self { attribute: a.0, item: StatementItem::BlockAssign(a.1) }
    }
    pub fn case_statement(a: (Vec<Attr>, CaseState)) -> Self {
        Self { attribute: a.0, item: StatementItem::CaseStatement(a.1) }
    }
    pub fn conditional_statement(a: (Vec<Attr>, Conditional)) -> Self {
        Self { attribute: a.0, item: StatementItem::ConditionalStatement(a.1) }
    }

    pub fn loop_statement(a: (Vec<Attr>, Loop)) -> Self {
        Self { attribute: a.0, item: StatementItem::Loop(Box::new(a.1)) }
    }
    pub fn nonblocking_assignment(a: (Vec<Attr>, NonBlockAssign)) -> Self {
        Self { attribute: a.0, item: StatementItem::NonBlockAssign(a.1) }
    }

    pub fn procedural_continuous_assignments(a: (Vec<Attr>, ProceduralContinuous)) -> Self {
        Self { attribute: a.0, item: StatementItem::ProceduralContinuous(a.1) }
    }
    pub fn procedural_timing_control_statement(a: (Vec<Attr>, (ProceduralTimingCtrl, StatementOrNull))) -> Self {
        Self { attribute: a.0, item: StatementItem::ProceduralTimingCtrlStatement(a.1.0, Box::new(a.1.1)) }
    }
    pub fn seq_block(a: (Vec<Attr>, SeqBlock)) -> Self {
        Self { attribute: a.0, item: StatementItem::SeqBlock(a.1) }
    }
}

#[derive(Debug)]
pub enum StatementItem {
    BlockAssign(BlockAssign),
    CaseStatement(CaseState),
    ConditionalStatement(Conditional),

    Loop(Box<Loop>),
    NonBlockAssign(NonBlockAssign),

    ProceduralContinuous(ProceduralContinuous),
    ProceduralTimingCtrlStatement(ProceduralTimingCtrl, Box<StatementOrNull>),
    SeqBlock(SeqBlock),
}

#[derive(Debug)]
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
#[derive(Debug)]
pub enum DelayCtrl {
    Value(String),
    Expr(MintypmaxExpression),
}

#[derive(Debug)]
pub enum DelayOrEventCtrl {
    Delay(DelayCtrl),
    Event(EventCtrl),
    Repeat(Expression, EventCtrl),
}

#[derive(Debug)]
pub enum EventCtrl {
    HierarchicalEvent(String),
    EventExpression(EventExpression),
    Auto1,
    Auto2,
}

#[derive(Debug)]
pub enum EventExpression {
    Expression(Expression),
    Posedge(Expression),
    Negedge(Expression),
    Or((Box<EventExpression>, Box<EventExpression>)),
    And((Box<EventExpression>, Box<EventExpression>)),
}

#[derive(Debug)]
pub enum ProceduralTimingCtrl {
    Delay(DelayCtrl),
    Event(EventCtrl),
}

// conditional
#[derive(Debug)]
pub struct Conditional {
    pub if_state: Vec<(Expression, StatementOrNull)>,
    pub else_state: Option<Box<StatementOrNull>>,
}

//case
#[derive(Debug)]
pub enum CaseState {
    Case((Expression, Vec<CaseItem>)),
    Casez((Expression, Vec<CaseItem>)),
    Casex((Expression, Vec<CaseItem>)),
}

#[derive(Debug)]
pub enum CaseItem {
    Express((Vec<Expression>, StatementOrNull)),
    Default(StatementOrNull)
}

//looping
#[derive(Debug)]
pub enum Loop {
    Forever(Statement),
    Repeat((Expression, Statement)),
    While((Expression, Statement)),
    For((VariableAssign, Expression, VariableAssign, Statement)),
}
