use parser_rust_simple::prelude::*;

use crate::verilog::expressions::ast::ConstantExpression;

use super::{data_types::{net_type, output_variable_type}, lists::{list_of_port_identifiers, list_of_variable_port_identifiers}, ranges::range, ast::{NetType, OutputDeclaration}};

// Parameter

/// local_parameter_declaration ::=
///  localparam [ signed ] [ range ] list_of_param_assignments
///  | localparam parameter_type list_of_param_assignments
// TODO
//pub fn local_parameter_declaration() -> impl Parser<Out = String> {
//    (token("localparam").zip(Try(token("signed"))) * Try(range()) * list_of_param_assignments())
//        | (token("localparam") * parameter_type() * list_of_param_assignments())
//}

/// parameter_declaration ::=
///  parameter [ signed ] [ range ] list_of_param_assignments
///  | parameter parameter_type list_of_param_assignments
// TODO
//pub fn parameter_declaration() -> impl Parser<Out = String> {
//    (token("parameter").zip(Try(token("signed"))) * Try(range()) * list_of_param_assignments())
//        | (token("parameter") * parameter_type() * list_of_param_assignments())
//}

/// specparam_declaration ::= specparam [ range ] list_of_specparam_assignments ;
// TODO
//pub fn specparam_declaration() -> impl Parser<Out = String> {
//    token("specparam") >> Try(range()) * list_of_specparam_assignments() << token(";")
//}

/// parameter_type ::=
///  integer | real | realtime | time
pub fn parameter_type<'a>() -> impl Parser<Out = &'a str> {
    token("integer") | token("real") | token("realtime") | token("time")
}

// Port

/// inout_declaration ::= inout [ net_type ] [ signed ] [ range ] list_of_port_identifiers
pub fn inout_declaration() -> impl Parser<Out = (((Option<NetType>, bool), Option<(ConstantExpression, ConstantExpression)>), Vec<String>)> {
    (token("inout") >> Try(net_type())) * Try(token("signed")).map(|x| x.is_some())
        * Try(range()) * list_of_port_identifiers()
}

/// input_declaration ::= input [ net_type ] [ signed ] [ range ] list_of_port_identifiers
pub fn input_declaration() -> impl Parser<Out = (((Option<NetType>, bool), Option<(ConstantExpression, ConstantExpression)>), Vec<String>)> {
    (token("input") >> Try(net_type())) * Try(token("signed")).map(|x| x.is_some())
        * Try(range()) * list_of_port_identifiers()
}

/// output_declaration ::=
///    output [ net_type ] [ signed ] [ range ]
///     list_of_port_identifiers
///  | output reg [ signed ] [ range ]
///     list_of_variable_port_identifiers
///  | output output_variable_type
///     list_of_variable_port_identifiers 
pub fn output_declaration() -> impl Parser<Out = OutputDeclaration> {
    ((token("output") >> Try(net_type())) * Try(token("signed")).map(|x| x.is_some())
        * Try(range()) * list_of_port_identifiers())
        .map(OutputDeclaration::Wire)
        | ((token("output") >> token("reg") >> Try(token("signed")).map(|x| x.is_some()))
            * Try(range()) * list_of_variable_port_identifiers())
            .map(OutputDeclaration::Reg)
        | ((token("output") >> output_variable_type()) * list_of_variable_port_identifiers())
            .map(OutputDeclaration::Others)
}

// Type
/*
/// event_declaration ::= event list_of_event_identifiers ;
// TODO
//pub fn event_declaration() -> impl Parser<Out = String> {
//    token("event") >> list_of_event_identifiers() << token(";")
//}

/// integer_declaration ::= integer list_of_variable_identifiers ;
pub fn integer_declaration() -> impl Parser<Out = String> {
    token("integer") >> list_of_variable_identifiers() << token(";")
}

/// net_declaration ::=
pub fn net_declaration() -> impl Parser<Out = String> {
    net_type()Try(signed())Try(delay3())list_of_net_identifiers()Token(";")net_type()Try(drive_strength())Try(signed())Try(delay3())list_of_net_decl_assignments()Token(";")
.or(net_type()Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_identifiers()Token(";")   
.or(net_type()Try(drive_strength())Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_decl_assignments()Token(";")
.or(trireg()Try(charge_strength())Try(signed())Try(delay3())list_of_net_identifiers()Token(";")
.or(trireg()Try(drive_strength())Try(signed())Try(delay3())list_of_net_decl_assignments()Token(";")
.or(trireg()Try(charge_strength())Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_identifiers()Token(";")
.or(trireg()Try(drive_strength())Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_decl_assignments()Token(";")
}

/// real_declaration ::= real list_of_real_identifiers ;
pub fn real_declaration() -> impl Parser<Out = String> {
    real()list_of_real_identifiers()Token(";")
}

/// realtime_declaration ::= realtime list_of_real_identifiers ;
pub fn realtime_declaration() -> impl Parser<Out = String> {
    realtime()list_of_real_identifiers()Token(";")
}

/// reg_declaration ::= reg [ signed ] [ range ] list_of_variable_identifiers ;
pub fn reg_declaration() -> impl Parser<Out = String> {
    reg()Try(signed())Try(range())list_of_variable_identifiers()Token(";")
}

/// time_declaration ::= time list_of_variable_identifiers ;
pub fn time_declaration() -> impl Parser<Out = String> {
    time()list_of_variable_identifiers()Token(";")Token("")
}*/

#[test]
fn test() {
    println!("{:?}", inout_declaration().run("inout [5:0] signal_inout"));
    println!("{:?}", input_declaration().run("input signed [7:0] signal_input"));
}
