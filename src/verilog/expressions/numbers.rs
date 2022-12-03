use parser_rust_simple::prelude::*;
use super::ast::*;

/// number ::= decimal_number | octal_number | binary_number | hex_number | real_number
pub fn number() -> impl Parser<Out = Number> {
    octal_number().map(Number::Octal)
        | binary_number().map(Number::Binary)
        | hex_number().map(Number::Hex)
        | real_number().map(Number::Real)
        | decimal_number().map(Number::Decimal)
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
    ((Try(size()) * decimal_base().left(whitespace()) * unsigned_number()).map(|((a,b),c)|
            format!("{}{}{}", a.unwrap_or_default(), b, c)
        ))
        .or((Try(size()) * decimal_base().left(whitespace()) * x_digit() * ParseRegex(r"[_]*")).map(|(((a,b),c),d)|
            format!("{}{}{}{}", a.unwrap_or_default(), b, c, d)
        ))
        .or((Try(size()) * decimal_base().left(whitespace()) * z_digit() * ParseRegex(r"[_]*")).map(|(((a,b),c),d)|
            format!("{}{}{}{}", a.unwrap_or_default(), b, c, d)
        ))
        .or(unsigned_number())
}

/// binary_number ::= [ size ] binary_base binary_value
fn binary_number() -> impl Parser<Out = String> {
    (Try(size()) * binary_base().left(whitespace()) * binary_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_default(), b, c)
    )
}

/// octal_number ::= [ size ] octal_base octal_value
fn octal_number() -> impl Parser<Out = String> {
    (Try(size()) * octal_base().left(whitespace()) * octal_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_default(), b, c)
    )
}

/// hex_number ::= [ size ] hex_base hex_value
fn hex_number() -> impl Parser<Out = String> {
    (Try(size()) * hex_base().left(whitespace()) * hex_value()).map(|((a,b),c)|
        format!("{}{}{}", a.unwrap_or_default(), b, c)
    )
}

/// sign ::= + | -
//fn sign() -> impl Parser<Out = String> {
//    ParseRegex(r"[+-]")
//}

/// size ::= non_zero_unsigned_number
fn size() -> impl Parser<Out = String> {
    non_zero_unsigned_number().left(whitespace())
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
    println!("{:?}", number().run("659")); // is a decimal number
    println!("{:?}", number().run("'h 837FF")); // is a hexadecimal number
    println!("{:?}", number().run("'o7460")); // is an octal number
    println!("{:?}", number().run("4af")); //TODO: is illegal (hexadecimal format requires 'h)

    println!("{:?}", number().run("4'b1001")); // is a 4-bit binary number
    println!("{:?}", number().run("5 'D 3")); // is a 5-bit decimal number
    println!("{:?}", number().run("3'b01x")); // is a 3-bit number with the least
                                              // significant bit unknown
    println!("{:?}", number().run("12'hx")); // is a 12-bit unknown number
    println!("{:?}", number().run("16'hz")); // is a 16-bit high-impedance number

    println!("{:?}", number().run("8 'd -6")); // this is illegal syntax
    println!("{:?}", number().run("-8 'd 6")); // TODO: this defines the two's complement of 6,
                                               // held in 8 bits—equivalent to -(8'd 6)
    println!("{:?}", number().run("4 'shf")); // this denotes the 4-bit number '1111', to
                                              // be interpreted as a 2's complement number,
                                              // or '-1'. This is equivalent to -4'h 1
    println!("{:?}", number().run("-4 'sd15")); // TODO: this is equivalent to -(-4'd 1), or '0001'
    println!("{:?}", number().run("16'sd?")); // the same as 16'sbz

    println!("{:?}", number().run("27_195_000")); // TODO: should be dec but found real
    println!("{:?}", number().run("16'b0011_0101_0001_1111"));
    println!("{:?}", number().run("32 'h 12ab_f001"));

    println!("{:?}", number().run("1.2"));
    println!("{:?}", number().run("0.1"));
    println!("{:?}", number().run("2394.26331"));
    println!("{:?}", number().run("1.2E12")); // TODO: (the exponent symbol can be e or E)
    println!("{:?}", number().run("1.30e-2"));
    println!("{:?}", number().run("0.1e-0"));
    println!("{:?}", number().run("23E10"));
    println!("{:?}", number().run("29E-2"));
    println!("{:?}", number().run("236.123_763_e-12")); // TODO: (underscores are ignored)

    //invalid:
    //.12
    //9.
    //4.E3
    //.2e-7

    

    let input = r"'sh";
    let parser = hex_base();
    println!("{:?}", parser.run(input));
    let input = r"'b";
    println!("{:?}", binary_base().run_with_out(input, Location::new()));
    let input = r"1'b1";
    println!("{:?}", binary_number().run_with_out(input, Location::new()));
}
