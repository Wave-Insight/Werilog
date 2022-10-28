use parser_rust_simple::prelude::*;

/// comment ::= one_line_comment | block_comment
pub fn comment() -> impl Parser<Out = String> {
    one_line_comment().or(block_comment())
}
/// one_line_comment ::= // comment_text \n
fn one_line_comment() -> impl Parser<Out = String> {
    Token(r"//") >> ParseRegex(r".*") << Token("\n")
}
/// block_comment ::= /* comment_text */
fn block_comment() -> impl Parser<Out = String> {
    Token(r"/*") >> ParseRegex(r"[\s\S]*?\*/").map(|x| x.strip_suffix("*/").unwrap().to_string())
}
// comment_text ::= { Any_ASCII_character }


#[test]
fn test() {
    let input_oneline = "//abcdefg\nabc";
    let parser = one_line_comment();
    println!("{:?}", parser.run(input_oneline));
    let input_block = "/*abc*d/e\nfg*/abc";
    let parser = block_comment();
    println!("{:?}", parser.run(input_block));
    println!("{:?}", comment().run(input_oneline));
    println!("{:?}", comment().run(input_block));
}
