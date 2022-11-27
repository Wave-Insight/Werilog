use parser_rust_simple::prelude::*;
use crate::verilog::general::identifiers::{hierarchical_identifier, parameter_identifier};

use super::ast::*;
use super::concatenations::{concatenation, multiple_concatenation};
use super::expressions::{range_expression, mintypmax_expression, constant_range_expression};
use super::numbers::number;
//use crate::verilog::general::identifiers::*;
//use super::expressions::*;
//use super::concatenations::*;
//use super::function_calls::*;
use super::strings::string;

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
        | (parameter_identifier().zip(Try(Token("[") >> tobox!(constant_range_expression()) << Token("]"))))
            .map(|x| ConstantPrimary::Parameter(x.0))//TODO:range expression
        //TODO
        /*.or(specparam_identifier().zip(Try(Token("[") >> constant_range_expression() << Token("]"))))
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
#[inline]
pub fn primary() -> impl Parser<Out = Primary> {
    number().map(Primary::Number)
        | (hierarchical_identifier().zip(Try(
            //TODO:Many(token("[") >> tobox!(expression()) << token("]"), None) * (token("[") >> tobox!(range_expression()) << token("]"))
            Many(token("[") >> tobox!(range_expression()) << token("]"), None)
        ))).map(|x| Primary::Hierarchical(x.0, Box::new(x.1)))
        //TODO
        | concatenation().map(Primary::Concatenation)
        | multiple_concatenation().map(|x| Primary::MultipleConcatenation(x.0, x.1))
        /*.or(function_call())
        .or(system_function_call())*/
        | (token("(") >> tobox!(mintypmax_expression()) << token(")"))
            .map(|x| Primary::MintypmaxExpression(Box::new(x)))
        | string().map(Primary::String)
}

#[test]
fn test() {
    let input = "signal[range]";
    println!("{:?}", primary().run_with_out(input, Location::new()));
    let input = "signal[7:0]";
    println!("{:?}", primary().run_with_out(input, Location::new()));
    let input = "(io_K_0_delay_3_1 ^ Tout_ret)";
    println!("{:?}", primary().run_with_out(input, Location::new()));
}
