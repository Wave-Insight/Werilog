use parser_rust_simple::prelude::*;

/// string ::= " { Any_ASCII_Characters_except_new_line } "
/// 
/// # Example
/// 
/// ```
/// use crate::verilog_parser::verilog::expressions::strings::string;
/// use parser_rust_simple::prelude::*;
/// let input1 = r#""abcd"useless"#;
/// let input2 = "\"ab\ncd\"useless";
/// let parser = string();
/// assert_eq!("abcd".to_string(), parser.run(input1).unwrap());
/// assert!(parser.run(input2).is_err());
/// ```
pub fn string() -> impl Parser<Out = String> {
    Token("\"") >> ParseRegex(r#"([^"\n]*)"#) << Token("\"")
}
