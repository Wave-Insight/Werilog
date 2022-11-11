use parser_rust_simple::prelude::*;

/// white_space ::= space | tab | newline | eof
pub fn white_space<'a>() -> ParseRegex<'a> {
    ParseRegex(r"[ \t\n]*")
}
