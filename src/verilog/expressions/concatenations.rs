use parser_rust_simple::prelude::*;
use crate::verilog::general::white_space::white_space;

use super::{expressions::*, ast::*};

/// concatenation ::= { expression { , expression } }
pub fn concatenation() -> impl Parser<Out = Vec<Expression>> {
    token("{") >> Many(tobox!(expression()).left(white_space()), Some(",")) << token("}")
}

/// constant_concatenation ::= { constant_expression { , constant_expression } }
pub fn constant_concatenation() -> impl Parser<Out = Vec<ConstantExpression>> {
    token("{") >> Many(constant_expression().left(white_space()), Some(",")) << token("}")
}

/// constant_multiple_concatenation ::= { constant_expression constant_concatenation }
pub fn constant_multiple_concatenation() -> impl Parser<Out = (ConstantExpression, Vec<ConstantExpression>)> {
    token("{") >> (constant_expression().zip(constant_concatenation())) << token("}")
}

/// module_path_concatenation ::= { module_path_expression { , module_path_expression } }
//TODO 
/*pub fn module_path_concatenation() -> impl Parser<Out = String> {
    Token("{") >> Many(module_path_expression(), Some(",")) << Token("}")
}*/

/// module_path_multiple_concatenation ::= { constant_expression module_path_concatenation }
//TODO
/*pub fn module_path_multiple_concatenation() -> impl Parser<Out = String> {
    token("{") >> (constant_expression().left(white_space()).zip(module_path_concatenation().left(white_space()))) << token("}")
}*/

/// multiple_concatenation ::= { constant_expression concatenation } 
pub fn multiple_concatenation() -> impl Parser<Out = (ConstantExpression, Vec<Expression>)> {
    token("{") >> (constant_expression().left(white_space()).zip(concatenation().left(white_space()))) << token("}")
}
