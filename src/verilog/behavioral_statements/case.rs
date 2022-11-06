use parser_rust_simple::prelude::*;

use crate::verilog::expressions::expressions::expression;

use super::{statements::statement_or_null, ast::{CaseItem, CaseState}};

/// case_statement ::=
///  case ( expression )
///      case_item { case_item } endcase
///  | casez ( expression )
///      case_item { case_item } endcase
///  | casex ( expression )
///      case_item { case_item } endcase
pub fn case_statement() -> impl Parser<Out = CaseState> {
    ((token("case") >> token("(") >> expression() << token(")"))
        * (Many(case_item(), None) << token("endcase")))
        .map(CaseState::Case)
        | ((token("casez") >> token("(") >> expression() << token(")"))
            * (Many(case_item(), None) << token("endcase")))
            .map(CaseState::Casez)
        | ((token("casex") >> token("(") >> expression() << token(")"))
            * (Many(case_item(), None) << token("endcase")))
            .map(CaseState::Casex)
}

/// case_item ::=
///  expression { , expression } : statement_or_null
///  | default [ : ] statement_or_null
pub fn case_item() -> impl Parser<Out = CaseItem> {
    (Many(expression(), Some(",")) * (token(":") >> statement_or_null()))
        .map(CaseItem::Express)
        | (token("default") >> Try(token(":")) >> statement_or_null())
            .map(CaseItem::Default)
}
