use parser_rust_simple::prelude::*;

use crate::verilog::expressions::expressions::{
    constant_expression,
    constant_range_expression,
    expression,
    range_expression
};
use crate::verilog::general::identifiers::{hierarchical_net_identifier, hierarchical_variable_identifier};

use super::ast::{NetLvalue, VariableLvalue};

//TODO:all

/// net_lvalue ::=
///  hierarchical_net_identifier [ { [ constant_expression ] } [ constant_range_expression ] ]
///  | { net_lvalue { , net_lvalue } }
pub fn net_lvalue() -> impl Parser<Out = NetLvalue> {
    (hierarchical_net_identifier().zip(Try(
        Many(token("[") >> tobox!(constant_expression()) << token("]"), None)
            * (token("[") >> tobox!(constant_range_expression()) << token("]"))
    ))).map(NetLvalue::VariableRef)
    .or( token("{") >> Many(tobox!(net_lvalue()), Some(",")).map(NetLvalue::Concatenation) << token("}") )
}

/// variable_lvalue ::=
///  hierarchical_variable_identifier [ { [ expression ] } [ range_expression ] ]
///  | { variable_lvalue { , variable_lvalue } }
pub fn variable_lvalue() -> impl Parser<Out = VariableLvalue> {
    (hierarchical_variable_identifier().zip(Try(
        Many(token("[") >> tobox!(expression()) << token("]"), None)
            * (token("[") >> tobox!(range_expression()) << token("]"))
    ))).map(VariableLvalue::VariableRef)
    .or( token("{") >> Many(tobox!(variable_lvalue()), Some(",")).map(VariableLvalue::Concatenation) << token("}") )
}
 