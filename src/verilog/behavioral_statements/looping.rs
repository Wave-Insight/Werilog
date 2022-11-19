use parser_rust_simple::prelude::*;

use crate::verilog::expressions::expressions::expression;

use super::{statements::statement, procedural_blocks::variable_assignment, ast::Loop};

/// loop_statement ::=
///  forever statement
///  | repeat ( expression ) statement
///  | while ( expression ) statement
///  | for ( variable_assignment ; expression ; variable_assignment )
///       statement
pub fn loop_statement() -> impl Parser<Out = Loop> {
    let in_for = (variable_assignment().left(token(";")))
        * (tobox!(expression()).left(token(";")))
        * variable_assignment();
    (token("forever") >> tobox!(statement())).map(Loop::Forever)
        | ((token("repeat") >> token("(") >> tobox!(expression()) << token(")")) * tobox!(statement())).map(Loop::Repeat)
        | ((token("while") >> token("(") >> tobox!(expression()) << token(")")) * tobox!(statement())).map(Loop::While)
        | ((token("for") >> token("(") >> in_for << token(")")) * tobox!(statement()))
            .map(|(((v1,e),v2),s)| Loop::For((v1, e, v2, s)))
}
