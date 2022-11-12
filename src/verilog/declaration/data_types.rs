use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::{real_identifier, variable_identifier}, expressions::expressions::constant_expression};

use super::{ranges::dimension, ast::*};


// Net and variable

/// net_type ::=
///     supply0 | supply1
///   | tri | triand | trior | tri0 | tri1
///   | uwire | wire | wand | wor
pub fn net_type() -> impl Parser<Out = NetType> {
    token("supply0").map(|_| NetType::Supply0)
        | token("supply1").map(|_| NetType::Supply1)
        | token("tri").map(|_| NetType::Tri)
        | token("triand").map(|_| NetType::Triand)
        | token("trior").map(|_| NetType::Trior)
        | token("tri0").map(|_| NetType::Tri0)
        | token("tri1").map(|_| NetType::Tri1)
        | token("uwire").map(|_| NetType::Uwire)
        | token("wire").map(|_| NetType::Wire)
        | token("wand").map(|_| NetType::Wand)
        | token("wor").map(|_| NetType::Wor)
}

/// output_variable_type ::= integer | time
pub fn output_variable_type() -> impl Parser<Out = OutputVariableType> {
    token("integer").map(|_| OutputVariableType::Integer)
        | token("time").map(|_| OutputVariableType::Time)
}

/// real_type ::=
///     real_identifier { dimension }
///   | real_identifier = constant_expression
pub fn real_type() -> impl Parser<Out = RealDeclaration> {
    (real_identifier().zip(Many(dimension(), None)))
        .map(|x| RealDeclaration::Dimension(x.0, x.1))
        | ((real_identifier().left(token("="))) * constant_expression())
            .map(|x| RealDeclaration::ConstExp(x.0, x.1))
}

/// variable_type ::=
///     variable_identifier { dimension }
///   | variable_identifier = constant_expression
pub fn variable_type() -> impl Parser<Out = VariableDeclaration> {
    (variable_identifier().zip(Many(dimension(), None)))
        .map(|x| VariableDeclaration::Dimension(x.0, x.1))
        | (variable_identifier().left(token("=")) * constant_expression())
            .map(|x| VariableDeclaration::ConstExp(x.0, x.1))
}

// Strengths

// TODO

// Delays

//TODO
