use parser_rust_simple::prelude::*;

use crate::verilog::expressions::{
    expression_left_side_values::{variable_lvalue, net_lvalue},
    expressions::expression
};
use super::{continuous_assignment::net_assignment, statements::statement, ast::Statement};
use super::ast::{BlockAssign, NonBlockAssign, VariableAssign, ProceduralContinuous};

/// initial_construct ::= initial statement
pub fn initial_construct() -> impl Parser<Out = Statement> {
    token("initial") >> statement()
}

/// always_construct ::= always statement
pub fn always_construct() -> impl Parser<Out = Statement> {
    token("always") >> statement()
}

/// blocking_assignment ::= variable_lvalue = [ delay_or_event_control ] expression
pub fn blocking_assignment() -> impl Parser<Out = BlockAssign> {
    (variable_lvalue().left(whitespace())
        * (token("=") >>
        //Try(delay_or_event_control()) *   ////TODO
        expression())).map(|(a,b)| BlockAssign(a, b))
}

/// nonblocking_assignment ::= variable_lvalue <= [ delay_or_event_control ] expression
pub fn nonblocking_assignment() -> impl Parser<Out = NonBlockAssign> {
    (variable_lvalue().left(whitespace())
        * (token("<=") >>
        //Try(delay_or_event_control()) *    ////TODO
        expression())).map(|(a,b)| NonBlockAssign(a, b))
}

/// procedural_continuous_assignments ::=
///       assign variable_assignment
///     | deassign variable_lvalue
///     | force variable_assignment
///     | force net_assignment
///     | release variable_lvalue
///     | release net_lvalue
pub fn procedural_continuous_assignments() -> impl Parser<Out = ProceduralContinuous> {
    (token("assign") >> variable_assignment()).map(ProceduralContinuous::Assign)
        .or(token("deassign") >> variable_lvalue().map(ProceduralContinuous::Deassign))
        .or(token("force") >> variable_assignment().map(ProceduralContinuous::ForceVar))
        .or(token("force") >> net_assignment().map(ProceduralContinuous::ForceNet))
        .or(token("release") >> variable_lvalue().map(ProceduralContinuous::ReleaseVar))
        .or(token("release") >> net_lvalue().map(ProceduralContinuous::ReleaseNet))
}

/// variable_assignment ::= variable_lvalue = expression
pub fn variable_assignment() -> impl Parser<Out = VariableAssign> {
    (variable_lvalue().left(whitespace())
        * (token("=") >> expression())).map(|(a,b)| VariableAssign(a, b))
}
