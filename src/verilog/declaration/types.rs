use parser_rust_simple::prelude::*;

use super::{data_types::{net_type, output_variable_type}, lists::{list_of_port_identifiers, list_of_variable_port_identifiers, list_of_param_assignments, list_of_event_identifiers, list_of_variable_identifiers, list_of_real_identifiers, list_of_net_identifiers}, ranges::range, ast::{NetType, OutputDeclaration, Range, ParameterDeclaration, VariableDeclaration, RealDeclaration, NetDeclaration}};

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
pub fn parameter_declaration() -> impl Parser<Out = ParameterDeclaration> {
    ((token("parameter") >> Try(token("signed")).map(|x| x.is_some())) * Try(range()) * list_of_param_assignments())
        .map(|x| ParameterDeclaration::Notype(x.0.0, x.0.1, x.1))
        | ((token("parameter") >> parameter_type()) * list_of_param_assignments())
            .map(|(ptype, x)| match ptype {
                "integer" => ParameterDeclaration::Integer(x),
                "real" => ParameterDeclaration::Real(x),
                "realtime" => ParameterDeclaration::Realtime(x),
                _ => ParameterDeclaration::Time(x),
            })
}

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
pub fn inout_declaration() -> impl Parser<Out = (Option<NetType>, bool, Option<Range>, Vec<String>)> {
    ((token("inout") >> Try(net_type())) * Try(token("signed")).map(|x| x.is_some())
        * Try(range()) * list_of_port_identifiers())
        .map(|x| (x.0.0.0, x.0.0.1, x.0.1, x.1))
}

/// input_declaration ::= input [ net_type ] [ signed ] [ range ] list_of_port_identifiers
pub fn input_declaration() -> impl Parser<Out = (Option<NetType>, bool, Option<Range>, Vec<String>)> {
    ((token("input") >> Try(net_type())) * Try(token("signed")).map(|x| x.is_some())
        * Try(range()) * list_of_port_identifiers())
        .map(|x| (x.0.0.0, x.0.0.1, x.0.1, x.1))
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

/// event_declaration ::= event list_of_event_identifiers ;
pub fn event_declaration() -> impl Parser<Out = Vec<(String, Vec<Range>)>> {
    token("event") >> list_of_event_identifiers() << token(";")
}

/// integer_declaration ::= integer list_of_variable_identifiers ;
pub fn integer_declaration() -> impl Parser<Out = Vec<VariableDeclaration>> {
    token("integer") >> list_of_variable_identifiers() << token(";")
}

/// net_declaration ::=
///     net_type [ signed ]
///     [ delay3 ] list_of_net_identifiers ;
///   | net_type [ drive_strength ] [ signed ]
///     [ delay3 ] list_of_net_decl_assignments ;
///   | net_type [ vectored | scalared ] [ signed ]
///     range [ delay3 ] list_of_net_identifiers ;
///   | net_type [ drive_strength ] [ vectored | scalared ] [ signed ]
///     range [ delay3 ] list_of_net_decl_assignments ;
///   | trireg [ charge_strength ] [ signed ]
///     [ delay3 ] list_of_net_identifiers ;
///   | trireg [ drive_strength ] [ signed ]
///     [ delay3 ] list_of_net_decl_assignments ;
///   | trireg [ charge_strength ] [ vectored | scalared ] [ signed ]
///     range [ delay3 ] list_of_net_identifiers ;
///   | trireg [ drive_strength ] [ vectored | scalared ] [ signed ]
///     range [ delay3 ] list_of_net_decl_assignments ; 
pub fn net_declaration() -> impl Parser<Out = NetDeclaration> {
    (net_type().zip(Try(token("signed")).map(|x| x.is_some()))
        //TODO * Try(delay3())
        * list_of_net_identifiers().left(token(";")))
        .map(|x| NetDeclaration::Simple(x.0.0, x.0.1, x.1))
        //| (net_type().zip(Try(drive_strength())) * Try(signed())
        //    * Try(delay3()) * list_of_net_decl_assignments() << token(";"))
        //| (net_type().zip(Try(vectored() | scalared()))Try(signed())range()Try(delay3())list_of_net_identifiers()Token(";")   )
        //| (net_type()Try(drive_strength())Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_decl_assignments()Token(";"))
        //| (trireg()Try(charge_strength())Try(signed())Try(delay3())list_of_net_identifiers()Token(";"))
        //| (trireg()Try(drive_strength())Try(signed())Try(delay3())list_of_net_decl_assignments()Token(";"))
        //| (trireg()Try(charge_strength())Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_identifiers()Token(";"))
        //| (trireg()Try(drive_strength())Try(vectored() | scalared())Try(signed())range()Try(delay3())list_of_net_decl_assignments()Token(";"))
}

/// real_declaration ::= real list_of_real_identifiers ;
pub fn real_declaration() -> impl Parser<Out = Vec<RealDeclaration>> {
    token("real") >> list_of_real_identifiers() << token(";")
}

/// realtime_declaration ::= realtime list_of_real_identifiers ;
pub fn realtime_declaration() -> impl Parser<Out = Vec<RealDeclaration>> {
    token("realtime") >> list_of_real_identifiers() << token(";")
}

/// reg_declaration ::= reg [ signed ] [ range ] list_of_variable_identifiers ;
pub fn reg_declaration() -> impl Parser<Out = ((bool, Option<Range>), Vec<VariableDeclaration>)> {
    (token("reg") >> Try(token("signed")).map(|x| x.is_some()))
        * Try(range()) * (list_of_variable_identifiers().left(token(";")))
}
/*TODO
/// time_declaration ::= time list_of_variable_identifiers ;
pub fn time_declaration() -> impl Parser<Out = String> {
    time()list_of_variable_identifiers()Token(";")Token("")
}*/

#[test]
fn test() {
    println!("{:?}", inout_declaration().run("inout [5:0] signal_inout"));
    println!("{:?}", input_declaration().run("input signed [7:0] signal_input"));
    println!("{:?}", input_declaration().run("input signed       signal_input"));
    println!("{:?}", input_declaration().run("input              signal_input"));
}
