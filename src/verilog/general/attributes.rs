use parser_rust_simple::prelude::*;

use super::identifiers::identifier;

// attribute_instance ::= (* attr_spec { , attr_spec } *)
pub fn attribute_instance() -> impl Parser<Out = Vec<String>> {
    //TODO: check `attr_spec` has at least one
    (Token("(*") << whitespace()) >> Many(attr_spec(), Some(",")) << Token("*)")
}

/// attr_spec ::= attr_name [ = constant_expression ]
fn attr_spec() -> impl Parser<Out = String> {
    attr_name().zip(
        Try(whitespace() >> Token("=") << whitespace()
            //TODO:* constant_expression()
        )
    ).map(|x| x.0 + x.1.unwrap_or("")).left(whitespace())
}

/// attr_name ::= identifier 
fn attr_name() -> impl Parser<Out = String> {
    identifier()
}
