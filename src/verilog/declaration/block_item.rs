use parser_rust_simple::prelude::*;

use crate::verilog::general::{attributes::attribute_instance, identifiers::{variable_identifier, real_identifier}};

use super::{ranges::{dimension, range}, ast::{Range, BlockItemDeclaration}, types::event_declaration};


/// block_item_declaration ::=
///     { attribute_instance } reg [ signed ] [ range ] list_of_block_variable_identifiers ;
///   | { attribute_instance } integer list_of_block_variable_identifiers ;
///   | { attribute_instance } time list_of_block_variable_identifiers ;
///   | { attribute_instance } real list_of_block_real_identifiers ;
///   | { attribute_instance } realtime list_of_block_real_identifiers ;
///   | { attribute_instance } event_declaration
///   | { attribute_instance } local_parameter_declaration ;
///   | { attribute_instance } parameter_declaration ;
pub fn block_item_declaration() -> impl Parser<Out = BlockItemDeclaration> {
    ((Many(attribute_instance(), None) << token("reg")) * Try(token("signed")).map(|x| x.is_some())
        * Try(range()) * list_of_block_variable_identifiers().left(token(";")))
        .map(|x| BlockItemDeclaration::Reg(x.0.0.0, x.0.0.1, x.0.1, x.1))
        | ((Many(attribute_instance(), None) << token("integer")) * list_of_block_variable_identifiers().left(token(";")))
            .map(|x| BlockItemDeclaration::Integer(x.0, x.1))
        | ((Many(attribute_instance(), None) << token("time")) * list_of_block_variable_identifiers().left(token(";")))
            .map(|x| BlockItemDeclaration::Time(x.0, x.1))
        | ((Many(attribute_instance(), None) << token("real")) * list_of_block_real_identifiers().left(token(";")))
            .map(|x| BlockItemDeclaration::Real(x.0, x.1))
        | ((Many(attribute_instance(), None) << token("realtime")) * list_of_block_real_identifiers().left(token(";")))
            .map(|x| BlockItemDeclaration::Realtime(x.0, x.1))
        | (Many(attribute_instance(), None) * event_declaration())
            .map(|x| BlockItemDeclaration::Event(x.0, x.1))
        //| Many(attribute_instance(), None) * local_parameter_declaration() << token(";")
        //| Many(attribute_instance(), None) * parameter_declaration() << token(";")
}

/// list_of_block_variable_identifiers ::= block_variable_type { , block_variable_type }
pub fn list_of_block_variable_identifiers() -> impl Parser<Out = Vec<(String, Vec<Range>)>> {
    Many(block_variable_type(), Some(","))
}

/// list_of_block_real_identifiers ::= block_real_type { , block_real_type }
pub fn list_of_block_real_identifiers() -> impl Parser<Out = Vec<(String, Vec<Range>)>> {
    Many(block_real_type(), Some(","))
}

/// block_variable_type ::= variable_identifier { dimension }
pub fn block_variable_type() -> impl Parser<Out = (String, Vec<Range>)> {
    variable_identifier().zip(Many(dimension(), None))
}

/// block_real_type ::= real_identifier { dimension }
pub fn block_real_type() -> impl Parser<Out = (String, Vec<Range>)> {
    real_identifier().zip(Many(dimension(), None))
}
