use parser_rust_simple::prelude::*;

use crate::verilog::general::{attributes::attribute_instance, identifiers::module_identifier};
/*
/// source_text ::= { description }
pub fn source_text() -> impl Parser<Out = String> {
    Many(description(), None)
}

/// description ::=
///     module_declaration
///   | udp_declaration
///   | config_declaration
pub fn description() -> impl Parser<Out = String> {
    module_declaration()
        //TODO: | udp_declaration()
        | config_declaration()
}

/// module_declaration ::=
///   { attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
///   list_of_ports ; { module_item }
///   endmodule
///   | { attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
///   [ list_of_port_declarations ] ; { non_port_module_item }
///   endmodule
pub fn module_declaration() -> impl Parser<Out = String> {
    (Many(attribute_instance(), None) * module_keyword() * module_identifier()
        * Try(module_parameter_port_list()) * (list_of_ports() << token(";"))
        * (Many(module_item()) << token("endmodule")))
        | (Many(attribute_instance(), None) * module_keyword() * module_identifier()
            * Try(module_parameter_port_list()) * Try(list_of_port_declarations()) << token(";")
            * Many(non_port_module_item(), None) << token("endmodule"))
}
*/
/// module_keyword ::= module | macromodule
pub fn module_keyword<'a>() -> impl Parser<Out = &'a str> {
    token("module") | token("macromodule")
}
