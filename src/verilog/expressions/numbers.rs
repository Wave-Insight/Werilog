use parser_rust_simple::prelude::*;

/// number ::= decimal_number | octal_number | binary_number | hex_number | real_number
pub fn number() -> impl Parser<Out = String> {
    decimal_number().or(octal_number()) | binary_number() | hex_number() | real_number()
}

/// real_number ::= unsigned_number . unsigned_number | unsigned_number [ . unsigned_number ] exp [ sign ] unsigned_number
pub fn real_number() -> impl Parser<Out = String> {
    ParseRegex(r"[-+]?([0-9]*\.)?[0-9]+([eE][-+]?[0-9]+)?")//TODO
}

/// exp ::= e | E
pub fn exp() -> impl Parser<Out = String> {
    ParseRegex("[eE]")
}

/// decimal_number ::= unsigned_number | [ size ] decimal_base unsigned_number | [ size ] decimal_base x_digit { _ } | [ size ] decimal_base z_digit { _ }
pub fn decimal_number() -> impl Parser<Out = String> {
    unsigned_number()
        .or((Try(size()) * decimal_base() * unsigned_number()).map(|((a,b),c)|
            format!("{}{}{}", a.unwrap_or_else(|| "".to_string()), b, c)
        ))
        .or((Try(size()) * decimal_base() * x_digit() * ParseRegex(r"[_]*")).map(|(((a,b),c),d)|
            format!("{}{}{}{}", a.unwrap_or_else(|| "".to_string()), b, c, d)
        ))
        .or((Try(size()) * decimal_base() * z_digit() * ParseRegex(r"[_]*")).map(|(((a,b),c),d)|
            format!("{}{}{}{}", a.unwrap_or_else(|| "".to_string()), b, c, d)
        ))
}

/// binary_number ::= [ size ] binary_base binary_value
pub fn binary_number() -> impl Parser<Out = String> {
    (Try(size()) * binary_base() * binary_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_else(|| "".to_string()), b, c)
    )
}

/// octal_number ::= [ size ] octal_base octal_value
pub fn octal_number() -> impl Parser<Out = String> {
    (Try(size()) * octal_base() * octal_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_else(|| "".to_string()), b, c)
    )
}

/// hex_number ::= [ size ] hex_base hex_value
pub fn hex_number() -> impl Parser<Out = String> {
    (Try(size()) * hex_base() * hex_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_else(|| "".to_string()), b, c)
    )
}

/// sign ::= + | -
pub fn sign() -> impl Parser<Out = String> {
    ParseRegex(r"[+-]")
}

/// size ::= non_zero_unsigned_number
pub fn size() -> impl Parser<Out = String> {
    non_zero_unsigned_number()
}

/// non_zero_unsigned_number ::= non_zero_decimal_digit { _ | decimal_digit }
pub fn non_zero_unsigned_number() -> impl Parser<Out = String> {
    ParseRegex(r"[1-9][1-9_]")
}

/// unsigned_number ::= decimal_digit { _ | decimal_digit }
pub fn unsigned_number() -> impl Parser<Out = String> {
    ParseRegex(r"[0-9][0-9_]*")
}

/// binary_value ::= binary_digit { _ | binary_digit }
pub fn binary_value() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?01][xXzZ\?01_]*")
}

/// octal_value ::= octal_digit { _ | octal_digit }
pub fn octal_value() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?0-7][xXzZ\?0-7_]*")
}

/// hex_value ::= hex_digit { _ | hex_digit }
pub fn hex_value() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?0-9a-fA-F][xXzZ\?0-9a-fA-F_]*")
}

/// decimal_base ::= '[s|S]d | '[s|S]D
pub fn decimal_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[dD]")
}

/// binary_base ::= '[s|S]b | '[s|S]B
pub fn binary_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[bB]")
}

/// octal_base ::= '[s|S]o | '[s|S]O
pub fn octal_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[oO]")
}

/// hex_base ::= '[s|S]h | '[s|S]H
pub fn hex_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[hH]")
}

/// non_zero_decimal_digit ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
pub fn non_zero_decimal_digit() -> impl Parser<Out = String> {
    ParseRegex(r"[1-9]")
}

/// decimal_digit ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
pub fn decimal_digit() -> impl Parser<Out = String> {
    ParseRegex(r"[0-9]")
}

/// binary_digit ::= x_digit | z_digit | 0 | 1
pub fn binary_digit() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?01]")
}

/// octal_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
pub fn octal_digit() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?0-7]")
}

/// hex_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | a | b | c | d | e | f | A | B | C | D | E | F 
pub fn hex_digit() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?0-9a-fA-F]")
}

/// x_digit ::= x | X
pub fn x_digit() -> impl Parser<Out = String> {
    ParseRegex("[xX]")
}

/// z_digit ::= z | Z | ?
pub fn z_digit() -> impl Parser<Out = String> {
    ParseRegex(r"[zZ\?]")
}

#[test]
fn bnf() {
    use crate::ebnf_tools::ebnf;
    let input = r"
number ::= decimal_number | octal_number | binary_number | hex_number | real_number
real_number2 ::= unsigned_number . unsigned_number | unsigned_number [ . unsigned_number ] exp [ sign ] unsigned_number
exp ::= e | E
decimal_number ::= unsigned_number | [ size ] decimal_base unsigned_number | [ size ] decimal_base x_digit { _ } | [ size ] decimal_base z_digit { _ }
binary_number ::= [ size ] binary_base binary_value
octal_number ::= [ size ] octal_base octal_value
hex_number ::= [ size ] hex_base hex_value
sign ::= + | -
size ::= non_zero_unsigned_number
non_zero_unsigned_number2 ::= non_zero_decimal_digit { _ | decimal_digit }
unsigned_number2 ::= decimal_digit { _ | decimal_digit }
binary_value2 ::= binary_digit { _ | binary_digit }
octal_value2 ::= octal_digit { _ | octal_digit }
hex_value2 ::= hex_digit { _ | hex_digit }
decimal_base2 ::= '[s|S]d | '[s|S]D
binary_base2 ::= '[s|S]b | '[s|S]B
octal_base2 ::= '[s|S]o | '[s|S]O
hex_base2 ::= '[s|S]h | '[s|S]H
non_zero_decimal_digit ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
decimal_digit ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
binary_digit ::= x_digit | z_digit | 0 | 1
octal_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
hex_digit ::=
 x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | a | b | c | d | e | f | A | B | C | D | E | F
x_digit ::= x | X
z_digit ::= z | Z | ?
    ";
    let parser = ebnf(input);
    match parser {
        Ok(x) => println!("{}", x),
        Err(e) => println!("error at {:?}:\n{}", e, input.lines().nth(e.line-1).unwrap()),
    }
}

#[test]
fn test() {
    let input = r"'sh";
    let parser = hex_base();
    println!("{:?}", parser.run(input));
}
