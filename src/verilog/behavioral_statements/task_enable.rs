use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::{system_task_identifier, hierarchical_task_identifier}, expressions::{expressions::expression, ast::Expression}};

///system_task_enable ::= system_task_identifier [ ( [ expression ] { , [ expression ] } ) ] ;
pub fn system_task_enable() -> impl Parser<Out = (String, Option<Vec<Expression>>)> {
    system_task_identifier().zip(Try(token("(") >> Many(expression(), Some(",")) << token(")")))
}

///task_enable ::= hierarchical_task_identifier [ ( expression { , expression } ) ] ; 
pub fn task_enable() -> impl Parser<Out = (String, Option<Vec<Expression>>)> {
    hierarchical_task_identifier().zip(Try(token("(") >> Many(expression(), Some(",")) << token(")")))
}
