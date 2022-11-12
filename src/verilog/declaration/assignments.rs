use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::{hierarchical_parameter_identifier, net_identifier, parameter_identifier}, expressions::{expressions::{constant_mintypmax_expression, expression}, ast::{ConstantOrNot, MintypmaxExpression, Expression}}};


/// defparam_assignment ::= hierarchical_parameter_identifier = constant_mintypmax_expression
pub fn defparam_assignment() -> impl Parser<Out = (String, ConstantOrNot<MintypmaxExpression>)> {
    hierarchical_parameter_identifier().left(token("=")) * constant_mintypmax_expression()
}

/// net_decl_assignment ::= net_identifier = expression
pub fn net_decl_assignment() -> impl Parser<Out = (String, Expression)> {
    net_identifier().left(token("=")) * expression()
}

/// param_assignment ::= parameter_identifier = constant_mintypmax_expression
pub fn param_assignment() -> impl Parser<Out = (String, ConstantOrNot<MintypmaxExpression>)> {
    parameter_identifier().left(token("=")) * constant_mintypmax_expression()
}
/*
/// specparam_assignment ::=
///     specparam_identifier = constant_mintypmax_expression
///   | pulse_control_specparam 
// TODO
//pub fn specparam_assignment() -> impl Parser<Out = String> {
//    (specparam_identifier().left(token("=")) * constant_mintypmax_expression())
//        | pulse_control_specparam()
//}
*/

/// pulse_control_specparam ::=
///    PATHPULSE$ = ( reject_limit_value [ , error_limit_value ] )
///    | PATHPULSE$specify_input_terminal_descriptor$specify_output_terminal_descriptor
///    = ( reject_limit_value [ , error_limit_value ] )
// TODO
//fn pulse_control_specparam() -> impl Parser<Out = String> {
//    
//}

/// error_limit_value ::= limit_value
pub fn error_limit_value() -> impl Parser<Out = ConstantOrNot<MintypmaxExpression>> {
    limit_value()
}

/// reject_limit_value ::= limit_value
pub fn reject_limit_value() -> impl Parser<Out = ConstantOrNot<MintypmaxExpression>> {
    limit_value()
}

/// limit_value ::= constant_mintypmax_expression
pub fn limit_value() -> impl Parser<Out = ConstantOrNot<MintypmaxExpression>> {
    constant_mintypmax_expression()
}
