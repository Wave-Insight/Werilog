use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::hierarchical_event_identifier, expressions::{expressions::expression, ast::Expression}};

use super::{ast::{EventCtrl, EventExpression, StatementOrNull}, statements::statement_or_null};

/// delay_control ::=
///    # delay_value
///  | # ( mintypmax_expression )
// TODO:
//pub fn delay_control() -> impl Parser<Out = String> {
//    token("#") >> delay_value()
//        | token("#") >> token("(") >> mintypmax_expression() << token(")")
//}

/// delay_or_event_control ::=
///    delay_control
///  | event_control
///  | repeat ( expression ) event_control
// TODO:
//pub fn delay_or_event_control() -> impl Parser<Out = String> {
//    delay_control()
//        .or(event_control())
//        .or(token("repeat") >> token("(") >> expression() << token(")") * event_control())
//}

/// disable_statement ::=
///     disable hierarchical_task_identifier ;
///   | disable hierarchical_block_identifier ;
// TODO
//pub fn disable_statement() -> impl Parser<Out = String> {
//    token("disable") >> hierarchical_task_identifier() << token(";")
//        | (token("disable") >> hierarchical_block_identifier() << token(";"))
//}

/// event_control ::=
///    @ hierarchical_event_identifier
///  | @ ( event_expression )
///  | @*
///  | @ (*) 
pub fn event_control() -> impl Parser<Out = EventCtrl> {
    (token("@") >> hierarchical_event_identifier()).map(EventCtrl::HierarchicalEvent)
        | (token("@") >> token("(") >> event_expression() << token(")")).map(EventCtrl::EventExpression)
        | (token("@*")).map(|_| EventCtrl::Auto1)
        | (token("@") >> token("(*)")).map(|_| EventCtrl::Auto2)//TODO: (*) should split to three token?
}

/// event_trigger ::=
///     -> hierarchical_event_identifier { [ expression ] } ;
// TODO
//pub fn event_trigger() -> impl Parser<Out = String> {
//    Token("->")hierarchical_event_identifier()Many(Try(expression()))Token(";")
//}
 
/// event_expression ::=
///    expression
///  | posedge expression
///  | negedge expression
///  | event_expression or event_expression
///  | event_expression , event_expression
pub fn event_expression() -> impl Parser<Out = EventExpression> {
    expression().map(EventExpression::Expression)
        | (token("posedge") >> expression()).map(EventExpression::Posedge)
        | (token("negedge") >> expression()).map(EventExpression::Negedge)
        | (tobox!(event_expression()) * (token("or") >> tobox!(event_expression())))
            .map(|x| EventExpression::Or((Box::new(x.0), Box::new(x.1))))
        | (tobox!(event_expression()) * (token(",") >> tobox!(event_expression())))
            .map(|x| EventExpression::And((Box::new(x.0), Box::new(x.1))))
}

/// procedural_timing_control ::=
///     delay_control
///   | event_control
// TODO
//pub fn procedural_timing_control() -> impl Parser<Out = String> {
//    delay_control()
//        | event_control()
//}

/// procedural_timing_control_statement ::=
///   procedural_timing_control statement_or_null
// TODO
//pub fn procedural_timing_control_statement() -> impl Parser<Out = String> {
//    procedural_timing_control() * statement_or_null()
//}

/// wait_statement ::=
///   wait ( expression ) statement_or_null 
pub fn wait_statement() -> impl Parser<Out = (Expression, StatementOrNull)> {
    (token("wait") >> token("(") >> expression() << token(")")) * statement_or_null()
}
