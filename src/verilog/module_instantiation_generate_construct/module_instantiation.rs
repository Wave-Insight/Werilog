use parser_rust_simple::prelude::*;

use crate::verilog::{general::{identifiers::{module_identifier, parameter_identifier, module_instance_identifier, port_identifier}, attributes::attribute_instance, ast::Attr}, expressions::{expressions::{expression, mintypmax_expression}, ast::{Expression, ConstantOrNot, MintypmaxExpression}}};

/*TODO
/// module_instantiation ::=
///   module_identifier [ parameter_value_assignment ]
///     module_instance { , module_instance } ;
pub fn module_instantiation() -> impl Parser<Out = String> {
    module_identifier().zip(Try(parameter_value_assignment()))
        .zip(Many(module_instance(), Some(","))) << token(";")
}

/// parameter_value_assignment ::= # ( list_of_parameter_assignments )
pub fn parameter_value_assignment() -> impl Parser<Out = String> {
    token("#(") >> list_of_parameter_assignments() << token(")")
}

/// list_of_parameter_assignments ::=
///     ordered_parameter_assignment { , ordered_parameter_assignment }
///   | named_parameter_assignment { , named_parameter_assignment }
pub fn list_of_parameter_assignments() -> impl Parser<Out = String> {
    Many(ordered_parameter_assignment(), Some(","))
        | Many(named_parameter_assignment(), Some(","))
}

/// ordered_parameter_assignment ::= expression
pub fn ordered_parameter_assignment() -> impl Parser<Out = Expression> {
    expression()
}

/// named_parameter_assignment ::= . parameter_identifier ( [ mintypmax_expression ] )
pub fn named_parameter_assignment() -> impl Parser<Out = (String, Option<ConstantOrNot<MintypmaxExpression>>)> {
    (token(".") >> parameter_identifier() << token("("))
        * (Try(mintypmax_expression()) << token(")"))
}

/// module_instance ::= name_of_module_instance ( [ list_of_port_connections ] )
pub fn module_instance() -> impl Parser<Out = String> {
    (name_of_module_instance().left(token("("))) * (Try(list_of_port_connections()) << token(")"))
}

/// name_of_module_instance ::= module_instance_identifier [ range ]
pub fn name_of_module_instance() -> impl Parser<Out = String> {
    module_instance_identifier().zip(Try(range()))
}

/// list_of_port_connections ::=
///     ordered_port_connection { , ordered_port_connection }
///   | named_port_connection { , named_port_connection } 
pub fn list_of_port_connections() -> impl Parser<Out = String> {
    Many(ordered_port_connection(), Some(","))
        | Many(named_port_connection(), Some(","))
}

/// ordered_port_connection ::= { attribute_instance } [ expression ]
pub fn ordered_port_connection() -> impl Parser<Out = (Vec<Attr>, Option<Expression>)> {
    Many(attribute_instance(), None) * Try(expression())
}

/// named_port_connection ::= { attribute_instance } . port_identifier ( [ expression ] )
pub fn named_port_connection() -> impl Parser<Out = ((Vec<Attr>, String), Option<Expression>)> {
    (Many(attribute_instance(), None) << token(".")) * port_identifier()
        * (token("(") >> Try(expression()) << token(")"))
}*/
