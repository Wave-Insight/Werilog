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
        | (net_type().zip(Try(token("vectored").map(|_| true) | token("scalared").map(|_| false)))
            * Try(token("signed")).map(|x| x.is_some()) * range()
            //TODO:* Try(delay3())
            * list_of_net_identifiers().left(token(";")))
            .map(|x| NetDeclaration::Range(x.0.0.0.0, x.0.0.0.1, x.0.0.1, x.0.1, x.1))
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
    println!("{:?}", net_declaration().run("wand w;")); // a scalar net of type "wand"
    println!("{:?}", net_declaration().run("tri [15:0] busa;")); // a three-state 16-bit bus
    //println!("{:?}", net_declaration().run("trireg (small) storeit;")); // a charge storage node of strength small
    println!("{:?}", reg_declaration().run("reg a;")); // a scalar reg
    println!("{:?}", reg_declaration().run("reg[3:0] v;")); // a 4-bit vector reg made up of (from most to
    // least significant)v[3], v[2], v[1], and v[0]
    println!("{:?}", reg_declaration().run("reg signed [3:0] signed_reg;")); // a 4-bit vector in range -8 to 7
    println!("{:?}", reg_declaration().run("reg [-1:4] b;")); // a 6-bit vector reg
    println!("{:?}", net_declaration().run("wire w1, w2;")); // declares two wires
    println!("{:?}", reg_declaration().run("reg [4:0] x, y, z;")); // declares three 5-bit regs

    println!("{:?}", net_declaration().run("tri1 scalared [63:0] bus64;")); //a bus that will be expanded
    println!("{:?}", net_declaration().run("tri vectored [31:0] data;")); //a bus that may or may not be expanded

    // println!("{:?}", net_declaration().run("trireg a;")); // trireg net of charge strength medium
    // println!("{:?}", net_declaration().run("trireg (large) #(0,0,50) cap1;")); // trireg net of charge strength large
    //                                                                            // with charge decay time 50 time units
    // println!("{:?}", net_declaration().run("trireg (small)signed [3:0] cap2;")); // signed 4-bit trireg vector of
    //                                                                              // charge strength small 

    println!("{:?}", parameter_declaration().run("parameter msb = 7;")); // defines msb as a constant value 7
    println!("{:?}", parameter_declaration().run("parameter e = 25, f = 9;")); // defines two constant numbers
    println!("{:?}", parameter_declaration().run("parameter r = 5.7;")); // declares r as a real parameter
    println!("{:?}", parameter_declaration().run("parameter byte_size = 8,"));
    //println!("{:?}", parameter_declaration().run("byte_mask = byte_size - 1;"));
    println!("{:?}", parameter_declaration().run("parameter average_delay = (r + f) / 2;")); // TODO:
    println!("{:?}", parameter_declaration().run("parameter signed [3:0] mux_selector = 0;"));
    println!("{:?}", parameter_declaration().run("parameter real r1 = 3.5e17;")); // TODO: real parameter
    println!("{:?}", parameter_declaration().run("parameter p1 = 13'h7e;"));
    println!("{:?}", parameter_declaration().run("parameter [31:0] dec_const = 1'b1;")); // value converted to 32 bits
    println!("{:?}", parameter_declaration().run("parameter newconst = 3'h4;")); // implied range of [2:0]
    println!("{:?}", parameter_declaration().run("parameter newconst = 4;")); // implied range of at least [31:0]

    println!("{:?}", inout_declaration().run("inout [5:0] signal_inout"));
    println!("{:?}", input_declaration().run("input signed [7:0] signal_input"));
    println!("{:?}", input_declaration().run("input signed       signal_input"));
    println!("{:?}", input_declaration().run("input              signal_input"));
}
