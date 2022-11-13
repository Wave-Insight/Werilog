use parser_rust_simple::prelude::*;
use super::ast::*;

/// number ::= decimal_number | octal_number | binary_number | hex_number | real_number
pub fn number() -> impl Parser<Out = Number> {
    octal_number().map(Number::Octal)
        | binary_number().map(Number::Binary)
        | hex_number().map(Number::Hex)
        | decimal_number().map(Number::Decimal)
        | real_number().map(Number::Real)
}

/// real_number ::= unsigned_number . unsigned_number | unsigned_number [ . unsigned_number ] exp [ sign ] unsigned_number
pub fn real_number() -> impl Parser<Out = String> {
    ParseRegex(r"[-+]?([0-9]*\.)?[0-9]+([eE][-+]?[0-9]+)?")//TODO
}

/// exp ::= e | E
//fn exp() -> impl Parser<Out = String> {
//    ParseRegex("[eE]")
//}

/// decimal_number ::= unsigned_number | [ size ] decimal_base unsigned_number | [ size ] decimal_base x_digit { _ } | [ size ] decimal_base z_digit { _ }
fn decimal_number() -> impl Parser<Out = String> {
    ((Try(size()) * decimal_base() * unsigned_number()).map(|((a,b),c)|
            format!("{}{}{}", a.unwrap_or_default(), b, c)
        ))
        .or((Try(size()) * decimal_base() * x_digit() * ParseRegex(r"[_]*")).map(|(((a,b),c),d)|
            format!("{}{}{}{}", a.unwrap_or_default(), b, c, d)
        ))
        .or((Try(size()) * decimal_base() * z_digit() * ParseRegex(r"[_]*")).map(|(((a,b),c),d)|
            format!("{}{}{}{}", a.unwrap_or_default(), b, c, d)
        ))
        .or(unsigned_number())
}

/// binary_number ::= [ size ] binary_base binary_value
fn binary_number() -> impl Parser<Out = String> {
    (Try(size()) * binary_base() * binary_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_default(), b, c)
    )
}

/// octal_number ::= [ size ] octal_base octal_value
fn octal_number() -> impl Parser<Out = String> {
    (Try(size()) * octal_base() * octal_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_default(), b, c)
    )
}

/// hex_number ::= [ size ] hex_base hex_value
fn hex_number() -> impl Parser<Out = String> {
    (Try(size()) * hex_base() * hex_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_default(), b, c)
    )
}

/// sign ::= + | -
//fn sign() -> impl Parser<Out = String> {
//    ParseRegex(r"[+-]")
//}

/// size ::= non_zero_unsigned_number
fn size() -> impl Parser<Out = String> {
    non_zero_unsigned_number()
}

/// non_zero_unsigned_number ::= non_zero_decimal_digit { _ | decimal_digit }
fn non_zero_unsigned_number() -> impl Parser<Out = String> {
    ParseRegex(r"[1-9][0-9_]*")
}

/// unsigned_number ::= decimal_digit { _ | decimal_digit }
pub fn unsigned_number() -> impl Parser<Out = String> {
    ParseRegex(r"[0-9][0-9_]*")
}

/// binary_value ::= binary_digit { _ | binary_digit }
fn binary_value() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?01][xXzZ\?01_]*")
}

/// octal_value ::= octal_digit { _ | octal_digit }
fn octal_value() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?0-7][xXzZ\?0-7_]*")
}

/// hex_value ::= hex_digit { _ | hex_digit }
fn hex_value() -> impl Parser<Out = String> {
    ParseRegex(r"[xXzZ\?0-9a-fA-F][xXzZ\?0-9a-fA-F_]*")
}

/// decimal_base ::= '[s|S]d | '[s|S]D
fn decimal_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[dD]")
}

/// binary_base ::= '[s|S]b | '[s|S]B
fn binary_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[bB]")
}

/// octal_base ::= '[s|S]o | '[s|S]O
fn octal_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[oO]")
}

/// hex_base ::= '[s|S]h | '[s|S]H
fn hex_base() -> impl Parser<Out = String> {
    ParseRegex(r"'[sS]?[hH]")
}

/// non_zero_decimal_digit ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
//fn non_zero_decimal_digit() -> impl Parser<Out = String> {
//    ParseRegex(r"[1-9]")
//}

/// decimal_digit ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
//fn decimal_digit() -> impl Parser<Out = String> {
//    ParseRegex(r"[0-9]")
//}

/// binary_digit ::= x_digit | z_digit | 0 | 1
//fn binary_digit() -> impl Parser<Out = String> {
//    ParseRegex(r"[xXzZ\?01]")
//}

/// octal_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
//fn octal_digit() -> impl Parser<Out = String> {
//    ParseRegex(r"[xXzZ\?0-7]")
//}

/// hex_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | a | b | c | d | e | f | A | B | C | D | E | F 
//fn hex_digit() -> impl Parser<Out = String> {
//    ParseRegex(r"[xXzZ\?0-9a-fA-F]")
//}

/// x_digit ::= x | X
fn x_digit() -> impl Parser<Out = String> {
    ParseRegex("[xX]")
}

/// z_digit ::= z | Z | ?
fn z_digit() -> impl Parser<Out = String> {
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
    let input = r"'b";
    println!("{:?}", binary_base().run_with_out(input, Location::new()));
    let input = r"1'b1";
    println!("{:?}", binary_number().run_with_out(input, Location::new()));
}
