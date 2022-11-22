use parser_rust_simple::prelude::*;

use crate::verilog::expressions::expressions::expression;

use super::{statements::statement_or_null, ast::Conditional};


/// conditional_statement ::=
///  if ( expression )
///  statement_or_null [ else statement_or_null ]
///  | if_else_if_statement
pub fn conditional_statement() -> impl Parser<Out = Conditional> {
    ((token("if") >> token("(") >> tobox!(expression()) << token(")"))
        * tobox!(statement_or_null())
        * Try(token("else") >> tobox!(statement_or_null())))
        .map(|x| {
            Conditional{
                if_state: vec![x.0],
                else_state: x.1.map(|a| Box::new(a))
            }
        })
        | if_else_if_statement()
}

/// if_else_if_statement ::=
///  if ( expression ) statement_or_null
///  { else if ( expression ) statement_or_null }
///  [ else statement_or_null ] 
pub fn if_else_if_statement() -> impl Parser<Out = Conditional> {
    ((token("if") >> token("(") >> tobox!(expression()) << token(")"))
        * tobox!(statement_or_null())
        * Many((token("else ") >> token("if") >> token("(") >> tobox!(expression()) << token(")"))
            * tobox!(statement_or_null()), None)
        * Try(token("else") >> tobox!(statement_or_null())))
        .map(|mut x| {
            let mut ret = vec![x.0.0];
            ret.append(&mut x.0.1);
            Conditional{
                if_state: ret,
                else_state: x.1.map(|a| Box::new(a))
            }
        })
}

#[test]
fn test() {
    let input = r"if(_zz_Tout_getTAU_SboxOut_5) begin
      _zz_Tout_getTAU_Sbox_port0 <= Tout_getTAU_Sbox[_zz_Tout_getTAU_SboxOut_1];
    end
    ";
    let parse = token("if") >> token("(") >> expression() << token(")");
    println!("{:?}", parse.run_with_out(input, Location::new()))
}
