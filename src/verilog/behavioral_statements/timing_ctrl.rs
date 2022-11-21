use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::hierarchical_event_identifier, expressions::{expressions::{expression, mintypmax_expression}, ast::Expression}, declaration::data_types::delay_value};

use super::{ast::{EventCtrl, EventExpression, StatementOrNull, DelayOrEventCtrl, DelayCtrl, ProceduralTimingCtrl}, statements::statement_or_null};

/// delay_control ::=
///    # delay_value
///  | # ( mintypmax_expression )
pub fn delay_control() -> impl Parser<Out = DelayCtrl> {
    token("#") >> delay_value().map(DelayCtrl::Value)
        | token("#") >> token("(") >> mintypmax_expression().map(DelayCtrl::Expr) << token(")")
}

/// delay_or_event_control ::=
///    delay_control
///  | event_control
///  | repeat ( expression ) event_control
pub fn delay_or_event_control() -> impl Parser<Out = DelayOrEventCtrl> {
    delay_control().map(DelayOrEventCtrl::Delay)
        | event_control().map(DelayOrEventCtrl::Event)
        | ((token("repeat") >> token("(") >> tobox!(expression()) << token(")")) * event_control())
            .map(|x| DelayOrEventCtrl::Repeat(x.0, x.1))
}

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
        | (token("@") >> token("(") >> token("*") >> token(")")).map(|_| EventCtrl::Auto2)//TODO: (*) should split to three token?
        | (token("@") >> token("(") >> tobox!(event_expression()) << token(")")).map(EventCtrl::EventExpression)
        | (token("@*")).map(|_| EventCtrl::Auto1)
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
    (token("posedge") >> expression()).map(EventExpression::Posedge)
        | (token("negedge") >> expression()).map(EventExpression::Negedge)
        | (tobox!(event_expression()) * (token("or") >> tobox!(event_expression())))
            .map(|x| EventExpression::Or((Box::new(x.0), Box::new(x.1))))
        | (tobox!(event_expression()) * (token(",") >> tobox!(event_expression())))
            .map(|x| EventExpression::And((Box::new(x.0), Box::new(x.1))))
        | expression().map(EventExpression::Expression)
}

/// procedural_timing_control ::=
///     delay_control
///   | event_control
pub fn procedural_timing_control() -> impl Parser<Out = ProceduralTimingCtrl> {
    delay_control().map(ProceduralTimingCtrl::Delay)
        | event_control().map(ProceduralTimingCtrl::Event)
}

/// procedural_timing_control_statement ::=
///   procedural_timing_control statement_or_null
pub fn procedural_timing_control_statement() -> impl Parser<Out = (ProceduralTimingCtrl, StatementOrNull)> {
    procedural_timing_control().zip(tobox!(statement_or_null()))
}

/// wait_statement ::=
///   wait ( expression ) statement_or_null 
pub fn wait_statement() -> impl Parser<Out = (Expression, StatementOrNull)> {
    (token("wait") >> token("(") >> tobox!(expression()) << token(")")) * statement_or_null()
}

#[test]
fn test() {
    let input = r"@(posedge clk)";
    println!("{:?}", procedural_timing_control().run_with_out(input, Location::new()))
}
