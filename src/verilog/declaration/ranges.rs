use parser_rust_simple::prelude::*;

use crate::verilog::expressions::{expressions::{dimension_constant_expression, msb_constant_expression, lsb_constant_expression}, ast::ConstantExpression};

use super::ast::Range;


/// dimension ::= [ dimension_constant_expression : dimension_constant_expression ]
pub fn dimension() -> impl Parser<Out = Range> {//impl Parser<Out = (ConstantExpression, ConstantExpression)> {
    ((token("[") >> dimension_constant_expression() << token(":"))
        * (dimension_constant_expression().left(token("]"))))
        .map(|x| Range(x.0, x.1))
}

/// range ::= [ msb_constant_expression : lsb_constant_expression ]
pub fn range() -> impl Parser<Out = Range> {
    ((token("[") >> msb_constant_expression() << token(":"))
        * (lsb_constant_expression().left(token("]"))))
        .map(|x| Range(x.0, x.1))
}
