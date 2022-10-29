/*use parser_rust_simple::prelude::*;

//TODO:all

/// net_lvalue ::=
///  hierarchical_net_identifier [ { [ constant_expression ] } [ constant_range_expression ] ]
///  | { net_lvalue { , net_lvalue } }
pub fn net_lvalue() -> impl Parser<Out = String> {
    (hierarchical_net_identifier() * Try(
        Many(Token("[") >> constant_expression() << Token("]"), None) * (Token("[") >> constant_range_expression() << Token("]"))
    ))
    .or( Token("{") >> Many(net_lvalue(), Some(",")) << Token("}") )
}

//variable_lvalue ::=
// hierarchical_variable_identifier [ { [ expression ] } [ range_expression ] ]
// | { variable_lvalue { , variable_lvalue } }
pub fn variable_lvalue() -> impl Parser<Out = String> {
    (hierarchical_variable_identifier() * Try(
        Many(Token("[") >> expression() << Token("]"), None) * (Token("[") >> range_expression() << Token("]"))
    ))
    .or( Token("{") >> Many(variable_lvalue(), Some(",")) << Token("}") )
}*/
 