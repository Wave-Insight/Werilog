use parser_rust_simple::prelude::*;
use super::ast::*;
use super::numbers::number;
//use crate::verilog::general::identifiers::*;
//use super::expressions::*;
//use super::concatenations::*;
//use super::function_calls::*;
//use super::strings::string;

/// constant_primary ::=
///  number
///  | parameter_identifier [ [ constant_range_expression ] ]
///  | specparam_identifier [ [ constant_range_expression ] ]
///  | constant_concatenation
///  | constant_multiple_concatenation
///  | constant_function_call
///  | constant_system_function_call
///  | ( constant_mintypmax_expression )
///  | string
pub fn constant_primary() -> impl Parser<Out = ConstantPrimary> {
    number().map(ConstantPrimary::Number)
        //TODO
        /*.or(parameter_identifier().zip(Try(Token("[") >> constant_range_expression() << Token("]"))))
        .or(specparam_identifier().zip(Try(Token("[") >> constant_range_expression() << Token("]"))))
        .or(constant_concatenation())
        .or(constant_multiple_concatenation())
        .or(constant_function_call())
        .or(constant_system_function_call())
        .or(Token("(") >> constant_mintypmax_expression() << Token(")"))
        .or(string())*/
}

/// module_path_primary ::=
///  number
///  | identifier
///  | module_path_concatenation
///  | module_path_multiple_concatenation
///  | function_call
///  | system_function_call
///  | ( module_path_mintypmax_expression )
pub fn module_path_primary() -> impl Parser<Out = ModulePathPrimary> {
    number().map(ModulePathPrimary::Number)
        //TODO
        /*.or(identifier())
        .or(module_path_concatenation())
        .or(module_path_multiple_concatenation())
        .or(function_call())
        .or(system_function_call())
        .or(Token("(") >> module_path_mintypmax_expression() << Token(")"))*/
}

/// primary ::=
///  number
///  | hierarchical_identifier [ { [ expression ] } [ range_expression ] ]
///  | concatenation
///  | multiple_concatenation
///  | function_call
///  | system_function_call
///  | ( mintypmax_expression )
///  | string
pub fn primary() -> impl Parser<Out = Primary> {
    number().map(Primary::Number)
        //TODO
        /*.or(hierarchical_identifier() * Try(
            Many(Token("[") >> expression() << Token("]") ) * (Token("[") >> range_expression() << Token("]"))
        ))
        .or(concatenation())
        .or(multiple_concatenation())
        .or(function_call())
        .or(system_function_call())
        .or(Token("(") >> mintypmax_expression << Token(")"))
        .or(string())*/
}
