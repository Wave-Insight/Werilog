use parser_rust_simple::prelude::*;

use crate::verilog::general::attributes::attribute_instance;

use super::{procedural_blocks::blocking_assignment, ast::{Statement, StatementOrNull}, case::case_statement, conditional::conditional_statement};

/// statement ::=
///    { attribute_instance } blocking_assignment ;
///  | { attribute_instance } case_statement
///  | { attribute_instance } conditional_statement
///  | { attribute_instance } disable_statement
///  | { attribute_instance } event_trigger
///  | { attribute_instance } loop_statement
///  | { attribute_instance } nonblocking_assignment ;
///  | { attribute_instance } par_block
///  | { attribute_instance } procedural_continuous_assignments ;
///  | { attribute_instance } procedural_timing_control_statement
///  | { attribute_instance } seq_block
///  | { attribute_instance } system_task_enable
///  | { attribute_instance } task_enable
///  | { attribute_instance } wait_statement
pub fn statement() -> impl Parser<Out = Statement> {
    ((Many(attribute_instance(), None) * blocking_assignment()) << token(";"))
        .map(Statement::blocking_assignment)
        //TODO
        | (Many(attribute_instance(), None) * tobox!(case_statement()))
            .map(Statement::case_statement)
        | (Many(attribute_instance(), None) * conditional_statement())
            .map(Statement::conditional_statement)
        /*.or(Many(attribute_instance()) * disable_statement())
        .or(Many(attribute_instance()) * event_trigger())
        .or(Many(attribute_instance()) * loop_statement())
        .or(Many(attribute_instance()) * nonblocking_assignment()token(";"))
        .or(Many(attribute_instance()) * par_block())
        .or(Many(attribute_instance()) * procedural_continuous_assignments()token(";"))
        .or(Many(attribute_instance()) * procedural_timing_control_statement())
        .or(Many(attribute_instance()) * seq_block())
        .or(Many(attribute_instance()) * system_task_enable())
        .or(Many(attribute_instance()) * task_enable())
        .or(Many(attribute_instance()) * wait_statement())*/
}

/// statement_or_null ::=
///    statement
///  | { attribute_instance } ;
pub fn statement_or_null() -> impl Parser<Out = StatementOrNull> {
    statement().map(StatementOrNull::from_statement)
        | (Many(attribute_instance(), None).map(StatementOrNull::from_attr) << token(";"))
}

/// function_statement1 ::= statement
pub fn function_statement1() -> impl Parser<Out = Statement> {
    statement()
}
