use parser_rust_simple::prelude::*;

use crate::verilog::general::attributes::attribute_instance;

use super::{module_parameters_ports::port_declaration, ast::ModuleItem};



/// module_item ::=
///     port_declaration ;
///   | non_port_module_item
pub fn module_item() -> impl Parser<Out = ModuleItem> {
    port_declaration().left(token(";")).map(ModuleItem::PortDeclaration)
        //TODO:| non_port_module_item()
}
/* TODO
/// module_or_generate_item ::=
///     { attribute_instance } module_or_generate_item_declaration
///   | { attribute_instance } local_parameter_declaration ;
///   | { attribute_instance } parameter_override
///   | { attribute_instance } continuous_assign
///   | { attribute_instance } gate_instantiation
///   | { attribute_instance } udp_instantiation
///   | { attribute_instance } module_instantiation
///   | { attribute_instance } initial_construct
///   | { attribute_instance } always_construct
///   | { attribute_instance } loop_generate_construct
///   | { attribute_instance } conditional_generate_construct
pub fn module_or_generate_item() -> impl Parser<Out = String> {
    (Many(attribute_instance(), None) * module_or_generate_item_declaration())
        | (Many(attribute_instance(), None) * local_parameter_declaration() << token(";"))
        | (Many(attribute_instance(), None) * parameter_override())
        | (Many(attribute_instance(), None) * continuous_assign())
        | (Many(attribute_instance(), None) * gate_instantiation())
        | (Many(attribute_instance(), None) * udp_instantiation())
        | (Many(attribute_instance(), None) * module_instantiation())
        | (Many(attribute_instance(), None) * initial_construct())
        | (Many(attribute_instance(), None) * always_construct())
        | (Many(attribute_instance(), None) * loop_generate_construct())
        | (Many(attribute_instance(), None) * conditional_generate_construct())
}

/// module_or_generate_item_declaration ::=
///     net_declaration
///   | reg_declaration
///   | integer_declaration
///   | real_declaration
///   | time_declaration
///   | realtime_declaration
///   | event_declaration
///   | genvar_declaration
///   | task_declaration
///   | function_declaration 
pub fn module_or_generate_item_declaration() -> impl Parser<Out = String> {
    net_declaration()
        | reg_declaration()
        | integer_declaration()
        | real_declaration()
        | time_declaration()
        | realtime_declaration()
        | event_declaration()
        | genvar_declaration()
        | task_declaration()
        | function_declaration()
}

/// non_port_module_item ::=
///     module_or_generate_item
///   | generate_region
///   | specify_block
///   | { attribute_instance } parameter_declaration ;
///   | { attribute_instance } specparam_declaration 
pub fn non_port_module_item() -> impl Parser<Out = String> {
    module_or_generate_item()
        | generate_region()
        | specify_block()
        | Many(attribute_instance(), None) * parameter_declaration() << token(";")
        | Many(attribute_instance(), None) * specparam_declaration()
}

/// parameter_override ::= defparam list_of_defparam_assignments ;
pub fn parameter_override() -> impl Parser<Out = String> {
    token("defparam") >> list_of_defparam_assignments() << token(";")
}*/
