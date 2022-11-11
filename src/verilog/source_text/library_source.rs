use parser_rust_simple::prelude::*;

use crate::verilog::general::identifiers::library_identifier;

/* TODO
/// library_text ::= { library_description }
pub fn library_text() -> impl Parser<Out = String> {
    Many(library_description(), None)
}

/// library_description ::=
///     library_declaration
///   | include_statement
///   | config_declaration
pub fn library_description() -> impl Parser<Out = String> {
    library_declaration()
        .or(include_statement())
        .or(config_declaration())
}*/

/// library_declaration ::=
///   library library_identifier file_path_spec [ { , file_path_spec } ]
///   [ -incdir file_path_spec { , file_path_spec } ] ;
pub fn library_declaration() -> impl Parser<Out = ((String, Vec<String>), Option<Vec<String>>)> {
    ((token("library") >> library_identifier()) * Many(file_path_spec(), Some(",")))
         * (Try(token("-incdir") >> Many(file_path_spec(), Some(","))) << token(";"))
}

/// include_statement ::= include file_path_spec ;
pub fn include_statement() -> impl Parser<Out = String> {
    token("include") >> file_path_spec() << token(";")
}

pub fn file_path_spec() -> impl Parser<Out = String> {
    ParseRegex(r"[a-zA-Z.*/\]*")//TODO:not in ieee 1364, a temp impl
}
