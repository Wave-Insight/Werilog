/*use parser_rust_simple::prelude::*;
use crate::verilog::general::{identifiers::*, white_space::white_space};
use super::expressions::*;

/// constant_function_call ::= function_identifier { attribute_instance } ( constant_expression { , constant_expression } )
pub fn constant_function_call() -> impl Parser<Out = String> {
    function_identifier() * Many(attribute_instance(), None)
        * (Token("(") >> Many(constant_expression(), Some(",")) << Token(")"))
}

/// constant_system_function_call ::= system_function_identifier ( constant_expression { , constant_expression } )
pub fn constant_system_function_call() -> impl Parser<Out = String> {
    system_function_identifier()
        .zip(token("(") >> Many(constant_expression().left(white_space()), Some(",")) << token(")"))
}

/// function_call ::= hierarchical_function_identifier { attribute_instance } ( expression { , expression } )  
pub fn function_call() -> impl Parser<Out = String> {
    hierarchical_function_identifier() * Many(attribute_instance(), None)
        * (Token("(") >> Many(expression(), Some(",")) << Token(")"))
}

/// system_function_call ::= system_function_identifier [ ( expression { , expression } ) ]
pub fn system_function_call() -> impl Parser<Out = String> {
    system_function_identifier()
        .zip(Try(Token("(") >> Many(expression(), Some(",")) << Token(")")))
}*/
