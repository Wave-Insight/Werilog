use parser_rust_simple::prelude::*;
use crate::verilog::{general::{attributes::attribute_instance, white_space::white_space}, expressions::operators::{unary_operator, binary_operator}};
use super::{ast::*, primaries::{primary, constant_primary}};

/// base_expression ::= expression
fn base_expression() -> impl Parser<Out = Expression> {
    expression()
}

/// conditional_expression ::= expression ? { attribute_instance } expression2 : expression3
//fn conditional_expression() -> impl Parser<Out = (((Expression, Vec<Attr>), Expression), Expression)> {
//    (expression1().left(white_space()) << token("?"))
//        * Many(attribute_instance(), None)
//        * (expression2().left(white_space()) << token(":"))
//        * expression3().left(white_space())
//}

/// constant_base_expression ::= constant_expression
fn constant_base_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

/*
/// constant_expression ::=
///  constant_primary
/// | unary_operator { attribute_instance } constant_primary
/// | constant_expression binary_operator { attribute_instance } constant_expression
/// | constant_expression ? { attribute_instance } constant_expression : constant_expression
pub fn constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_primary().map(ConstantExpression::ConstantPrimary)
        //TODO
        /*.or(unary_operator() * Many(attribute_instance(), None) * constant_primary())
        .or(constant_expression() * binary_operator() * Many(
            attribute_instance(), None) * constant_expression())
        .or(constant_expression() * Token("?") * Many(attribute_instance(), None)
            * constant_expression() * Token(":") *constant_expression())*/
}*/

/* ----------------------------------- */
// parse left recursive text in parser combinatory is a little different
// TODO: operator precedence, such as * should be earlier than +

//#[inline]
pub fn constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression_binary().zip(Try(
        (token("?") >> Many(attribute_instance(), None))
        * (tobox!(constant_expression()) << token(":"))
        * tobox!(constant_expression())
    )).map(|(x, y)| match y {
        Some(z) => ConstantExpression::Condition(Box::new(x), z.0.0, Box::new(z.0.1), Box::new(z.1)),
        None => x,
    })
}

fn constant_expression_binary() -> impl Parser<Out = ConstantExpression> {
    constant_expression_unary().zip(Try(binary_operator().zip(Many(attribute_instance(), None)) * tobox!(constant_expression())))
        .map(|x| match x.1 {
            Some(y) => ConstantExpression::Binary(Box::new(x.0), y.0.0.to_string(), y.0.1, Box::new(y.1)),
            None => x.0,
        })
}

fn constant_expression_unary() -> impl Parser<Out = ConstantExpression> {
    (constant_primary().map(ConstantExpression::ConstantPrimary))
        | (unary_operator().zip(Many(attribute_instance(), None)) * constant_primary())
            .map(|x| ConstantExpression::Unary(x.0.0.to_string(), x.0.1, x.1))
}

/* ----------------------------------- */

/// constant_mintypmax_expression ::=
///  constant_expression
///  | constant_expression : constant_expression : constant_expression
pub fn constant_mintypmax_expression() -> impl Parser<Out = ConstantMintypmaxExpression> {
    constant_expression().map(|x| ConstantMintypmaxExpression::ConstantExpression(x))
        | ((constant_expression().left(token(":")))
            * (tobox!(constant_expression()).left(token(":")))
            * tobox!(constant_expression()))
            .map(|((a,b),c)| ConstantMintypmaxExpression::Three(a,b,c))
}

/// constant_range_expression ::=
///  constant_expression
///  | msb_constant_expression : lsb_constant_expression
///  | constant_base_expression +: width_constant_expression
///  | constant_base_expression -: width_constant_expression
pub fn constant_range_expression() -> impl Parser<Out = ConstantRangeExpression> {
    ((msb_constant_expression().left(white_space()) << token(":")) * lsb_constant_expression())
        .map(ConstantRangeExpression::To)
        | ((constant_base_expression().left(white_space()) << token("+:")) * width_constant_expression())
            .map(ConstantRangeExpression::Add)
        | ((constant_base_expression().left(white_space()) << token("-:")) * width_constant_expression())
            .map(ConstantRangeExpression::Sub)
        | tobox!(constant_expression()).map(ConstantRangeExpression::Single)
}

/// dimension_constant_expression ::= constant_expression
pub fn dimension_constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

/*
/// expression ::=
///  primary
///  | unary_operator { attribute_instance } primary
///  | expression binary_operator { attribute_instance } expression
///  | conditional_expression
#[inline]
pub fn expression() -> impl Parser<Out = Expression> {//impl Parser<Out = Expression> {
    (primary().map(Expression::Primary))
        | (unary_operator().zip(Many(attribute_instance(), None)) * primary())
            .map(|x| Expression::Unary(x.0.0.to_string(), x.0.1, x.1))
        | (tobox!(expression()) * binary_operator() * Many(attribute_instance(), None) * tobox!(expression()))
            .map(|x| Expression::Binary(Box::new(x.0.0.0), x.0.0.1.to_string(), x.0.1, Box::new(x.1)))
        | (tobox!(conditional_expression())
            .map(|(((a,b),c),d)| Expression::Condition((Box::new(a), b, Box::new(c), Box::new(d)))))
}*/

/* ----------------------------------- */
// parse left recursive text in parser combinatory is a little different
// TODO: operator precedence, such as * should be earlier than +

//#[inline]
pub fn expression() -> impl Parser<Out = Expression> {
    expression_binary().zip(Try(
        (token("?") >> Many(attribute_instance(), None))
        * (tobox!(expression2()) << token(":"))
        * tobox!(expression3())
    )).map(|(x, y)| match y {
        Some(z) => Expression::Condition(Box::new(x), z.0.0, Box::new(z.0.1), Box::new(z.1)),
        None => x,
    })
}

fn expression_binary() -> impl Parser<Out = Expression> {
    expression_unary().zip(Try(binary_operator().zip(Many(attribute_instance(), None)) * tobox!(expression())))
        .map(|x| match x.1 {
            Some(y) => Expression::Binary(Box::new(x.0), y.0.0.to_string(), y.0.1, Box::new(y.1)),
            None => x.0,
        })
}

fn expression_unary() -> impl Parser<Out = Expression> {
    (primary().map(Expression::Primary))
        | (unary_operator().zip(Many(attribute_instance(), None)) * primary())
            .map(|x| Expression::Unary(x.0.0.to_string(), x.0.1, x.1))
}

/* ----------------------------------- */

/// expression1 ::= expression
//fn expression1() -> impl Parser<Out = Expression> {
//    expression()
//}

/// expression2 ::= expression
fn expression2() -> impl Parser<Out = Expression> {
    expression()
}

/// expression3 ::= expression
fn expression3() -> impl Parser<Out = Expression> {
    expression()
}

/// lsb_constant_expression ::= constant_expression
pub fn lsb_constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

/// mintypmax_expression ::=
///  expression
///  | expression : expression : expression
pub fn mintypmax_expression() -> impl Parser<Out = MintypmaxExpression> {
    expression().map(|x| MintypmaxExpression::Expression(x))
        | ((expression().left(white_space()) << token(":"))
            * (tobox!(expression()).left(white_space()) << token(":"))
            * tobox!(expression()))
            .map(|((a,b),c)| MintypmaxExpression::Three(a,b,c))
}

/// module_path_conditional_expression ::= module_path_expression ? { attribute_instance } module_path_expression : module_path_expression
//TODO
/*fn module_path_conditional_expression() -> impl Parser<Out = String> {
    module_path_expression() * Token("?") * Many(attribute_instance(), None)
        * module_path_expression() * Token(":") * module_path_expression()
}*/

/// module_path_expression ::=
///  module_path_primary
///  | unary_module_path_operator { attribute_instance } module_path_primary
///  | module_path_expression binary_module_path_operator { attribute_instance } module_path_expression
///  | module_path_conditional_expression
//TODO
/*pub fn module_path_expression() -> impl Parser<Out = String> {
    module_path_primary()
        .or(unary_module_path_operator() * Many(attribute_instance(), None) * module_path_primary())
        .or(module_path_expression() * binary_module_path_operator() * Many(attribute_instance(), None) * module_path_expression())    
        .or(module_path_conditional_expression())
}*/

/// module_path_mintypmax_expression ::=
///  module_path_expression
///  | module_path_expression : module_path_expression : module_path_expression
//TODO
/*pub fn module_path_mintypmax_expression() -> impl Parser<Out = String> {
    module_path_expression()
        .or(module_path_expression() * Token(":") * module_path_expression() * Token(":") * module_path_expression())
}*/

/// msb_constant_expression ::= constant_expression
pub fn msb_constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

/// range_expression ::=
///  expression
///  | msb_constant_expression : lsb_constant_expression
///  | base_expression +: width_constant_expression
///  | base_expression -: width_constant_expression
pub fn range_expression() -> impl Parser<Out = RangeExpression> {
    ((msb_constant_expression().left(white_space()) << token(":")) * lsb_constant_expression())
            .map(RangeExpression::To)
        | ((base_expression().left(white_space()) << token("+:")) * width_constant_expression())
            .map(RangeExpression::Add)
        | ((base_expression().left(white_space()) << token("-:")) * width_constant_expression())
            .map(RangeExpression::Sub)
        | tobox!(expression()).map(RangeExpression::Single)//TODO:should be parse first?
}

/// width_constant_expression ::= constant_expression
fn width_constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

#[test]
fn test() {
    let input = "5";
    println!("{:?}", expression().run_with_out(input, Location::new()));
    let input = "some_signal";
    println!("{:?}", expression().run_with_out(input, Location::new()));
    let input = "some_signal[5]";
    println!("{:?}", expression().run_with_out(input, Location::new()));
    let input = "7:0";
    let parse = range_expression();
    println!("{:?}", parse.run_with_out(input, Location::new()));
    let input = "_zz_Tout_getTAU_SboxOut[15 : 8]";
    println!("{:?}", expression().run_with_out(input, Location::new()));
    let input = "(io_K_0_delay_3_1 ^ Tout_ret)";
    println!("{:?}", expression().run_with_out(input, Location::new()));
}
