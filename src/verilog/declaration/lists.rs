use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::{net_identifier, port_identifier}, expressions::{ast::ConstantExpression, expressions::constant_expression}};

use super::ranges::dimension;


/// list_of_defparam_assignments ::= defparam_assignment { , defparam_assignment }
// TODO
//pub fn list_of_defparam_assignments() -> impl Parser<Out = String> {
//    Many(defparam_assignment(), Some(","))
//}

/// list_of_event_identifiers ::= event_identifier { dimension } { , event_identifier { dimension } }
// TODO
//pub fn list_of_event_identifiers() -> impl Parser<Out = String> {
//    event_identifier()Many(dimension())Many(Token(",")event_identifier()Many(dimension()))
//}

/// list_of_net_decl_assignments ::= net_decl_assignment { , net_decl_assignment }
// TODO
//pub fn list_of_net_decl_assignments() -> impl Parser<Out = String> {
//    net_decl_assignment()Many(Token(",")net_decl_assignment())
//}

/// list_of_net_identifiers ::= net_identifier { dimension } { , net_identifier { dimension } }
pub fn list_of_net_identifiers() -> impl Parser<Out = Vec<(String, Vec<(ConstantExpression, ConstantExpression)>)>> {
    Many(net_identifier().zip(Many(dimension(), None)), Some(","))
}

/// list_of_param_assignments ::= param_assignment { , param_assignment }
// TODO
//pub fn list_of_param_assignments() -> impl Parser<Out = String> {
//    Many(param_assignment(), Some(","))
//}

/// list_of_port_identifiers ::= port_identifier { , port_identifier }
pub fn list_of_port_identifiers() -> impl Parser<Out = Vec<String>> {
    Many(port_identifier(), Some(","))
}

/// list_of_real_identifiers ::= real_type { , real_type }
// TODO
//pub fn list_of_real_identifiers() -> impl Parser<Out = String> {
//    Many(real_type(), Some(","))
//}

/// list_of_specparam_assignments ::= specparam_assignment { , specparam_assignment }
// TODO
//pub fn list_of_specparam_assignments() -> impl Parser<Out = String> {
//    Many(specparam_assignment(), Some(","))
//}

/// list_of_variable_identifiers ::= variable_type { , variable_type }
// TODO
//pub fn list_of_variable_identifiers() -> impl Parser<Out = String> {
//    Many(variable_type(), Some(","))
//}

/// list_of_variable_port_identifiers ::= port_identifier [ = constant_expression ] { , port_identifier [ = constant_expression ] }
pub fn list_of_variable_port_identifiers() -> impl Parser<Out = Vec<(String, Option<ConstantExpression>)>> {
    let single = port_identifier().zip(Try(token("=") >> constant_expression()));
    Many(single, Some(","))
}
