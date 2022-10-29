use parser_rust_simple::prelude::*;
use crate::verilog::general::{attributes::attribute_instance, white_space::white_space, ast::Attr};
use super::{ast::*, primaries::{primary, constant_primary}};

/// base_expression ::= expression
fn base_expression() -> impl Parser<Out = Expression> {
    expression()
}

/// conditional_expression ::= expression ? { attribute_instance } expression2 : expression3
fn conditional_expression() -> impl Parser<Out = (((Expression, Vec<Attr>), Expression), Expression)> {
    (expression1().left(white_space()) << token("?"))
        * Many(attribute_instance(), None)
        * (expression2().left(white_space()) << token(":"))
        * expression3()
}

/// constant_base_expression ::= constant_expression
fn constant_base_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

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
}

/// constant_mintypmax_expression ::=
///  constant_expression
///  | constant_expression : constant_expression : constant_expression
pub fn constant_mintypmax_expression() -> impl Parser<Out = ConstantOrNot<MintypmaxExpression>> {
    constant_expression().map(|x| ConstantOrNot::Constant(MintypmaxExpression::ConstantExpression(x)))
        | ((constant_expression().left(white_space()) << token(":"))
            * (constant_expression().left(white_space()) << token(":"))
            * constant_expression())
            .map(|((a,b),c)| ConstantOrNot::Constant(MintypmaxExpression::Three((a,b,c))))
}

/// constant_range_expression ::=
///  constant_expression
///  | msb_constant_expression : lsb_constant_expression
///  | constant_base_expression +: width_constant_expression
///  | constant_base_expression -: width_constant_expression
pub fn constant_range_expression() -> impl Parser<Out = ConstantRangeExpression> {
    constant_expression().map(ConstantRangeExpression::Single)
        | ((msb_constant_expression().left(white_space()) << token(":")) * lsb_constant_expression())
            .map(ConstantRangeExpression::To)
        | ((constant_base_expression().left(white_space()) << token("+:")) * width_constant_expression())
            .map(ConstantRangeExpression::Add)
        | ((constant_base_expression().left(white_space()) << token("-:")) * width_constant_expression())
            .map(ConstantRangeExpression::Sub)
}

/// dimension_constant_expression ::= constant_expression
pub fn dimension_constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}

/// expression ::=
///  primary
///  | unary_operator { attribute_instance } primary
///  | expression binary_operator { attribute_instance } expression
///  | conditional_expression
pub fn expression() -> impl Parser<Out = Expression> {
    primary().map(Expression::Primary)
        //TODO
        /*.or(unary_operator() * Many(attribute_instance()) * primary())
        .or(expression() * binary_operator() * Many(attribute_instance()) * expression())
        .or(conditional_expression())*/
}

/// expression1 ::= expression
fn expression1() -> impl Parser<Out = Expression> {
    expression()
}

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
pub fn mintypmax_expression() -> impl Parser<Out = ConstantOrNot<MintypmaxExpression>> {
    constant_expression().map(|x| ConstantOrNot::NotConst(MintypmaxExpression::ConstantExpression(x)))
        | ((constant_expression().left(white_space()) << token(":"))
            * (constant_expression().left(white_space()) << token(":"))
            * constant_expression())
            .map(|((a,b),c)| ConstantOrNot::NotConst(MintypmaxExpression::Three((a,b,c))))
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
    expression().map(RangeExpression::Single)
        | ((msb_constant_expression().left(white_space()) << token(":")) * lsb_constant_expression())
            .map(RangeExpression::To)
        | ((base_expression().left(white_space()) << token("+:")) * width_constant_expression())
            .map(RangeExpression::Add)
        | ((base_expression().left(white_space()) << token("-:")) * width_constant_expression())
            .map(RangeExpression::Sub)
}

/// width_constant_expression ::= constant_expression
fn width_constant_expression() -> impl Parser<Out = ConstantExpression> {
    constant_expression()
}
