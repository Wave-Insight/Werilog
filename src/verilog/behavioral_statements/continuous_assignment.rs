use parser_rust_simple::prelude::*;

use crate::verilog::expressions::{expression_left_side_values::net_lvalue, expressions::expression};

use super::ast::NetAssign;

/// continuous_assign ::= assign [ drive_strength ] [ delay3 ] list_of_net_assignments ;
pub fn continuous_assign() -> impl Parser<Out = Vec<NetAssign>> {
    token("assign") >>
        //Try(drive_strength()) *//TODO
        //Try(delay3()) *//TODO
        list_of_net_assignments() << token(";")
}


/// list_of_net_assignments ::= net_assignment { , net_assignment }
pub fn list_of_net_assignments() -> impl Parser<Out = Vec<NetAssign>> {
    Many(net_assignment(), Some(","))
}

/// net_assignment ::= net_lvalue = expression
pub fn net_assignment() -> impl Parser<Out = NetAssign> {
    (whitespace() >> net_lvalue().left(whitespace())
        .zip(token("=") >> expression().left(whitespace())))
        .map(|(a,b)| NetAssign(a, b))
}
