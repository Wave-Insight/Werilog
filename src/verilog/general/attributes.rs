use parser_rust_simple::prelude::*;
use super::ast::*;
use super::identifiers::identifier;

// attribute_instance ::= (* attr_spec { , attr_spec } *)
pub fn attribute_instance() -> impl Parser<Out = Attr> {
    //TODO: check `attr_spec` has at least one
    Token("(*") >> whitespace() >>
        Many(attr_spec(), Some(",")).map(Attr)
        << Token("*)")
        << whitespace()
}

/// attr_spec ::= attr_name [ = constant_expression ]
fn attr_spec() -> impl Parser<Out = AttrSpec> {
    attr_name().zip(
        Try(whitespace() >> Token("=") << whitespace()
            //TODO:* constant_expression()
        )
    ).map(|x|
        match x.1 {
            Some(s) => AttrSpec::Equa(x.0, s.to_string()),
            None => AttrSpec::Single(x.0)
        }
    ).left(whitespace())
}

/// attr_name ::= identifier 
fn attr_name() -> impl Parser<Out = String> {
    identifier()
}
