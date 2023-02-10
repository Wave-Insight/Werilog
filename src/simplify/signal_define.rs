use crate::prelude::{PortDeclaration, ModuleItemDeclaration};
use super::ast::*;

pub fn io_define(port: PortDeclaration) -> Vec<Signal> {
    match port {
        PortDeclaration::Inout(_, s) => s.3.into_iter().map(|x| {
            Signal {
                name: Some(x),
                data_types: SignalType::Inout,
                signed: s.1,
                driver: Expression::NoExpr,
            }
        }).collect(),
        PortDeclaration::Input(_, s) => s.3.into_iter().map(|x| {
            Signal {
                name: Some(x),
                data_types: SignalType::Input,
                signed: s.1,
                driver: Expression::NoExpr,
            }
        }).collect(),
        PortDeclaration::Output(_, s) => {
            let (signed, name) = match s {
                crate::prelude::OutputDeclaration::Wire(x) => {
                    (x.0.0.1, x.1)
                },
                crate::prelude::OutputDeclaration::Reg(x) => {
                    (x.0.0, x.1.into_iter().map(|y| y.0).collect())
                },
                crate::prelude::OutputDeclaration::Others(_) => {
                    (false, vec![])//TODO:?
                },
            };
            name.into_iter().map(|x| {
                Signal {
                    name: Some(x),
                    data_types: SignalType::Output,//TODO:OutputReg?
                    signed,
                    driver: Expression::NoExpr,
                }
            }).collect()
        },
    }
}

pub fn non_io_define(signal: ModuleItemDeclaration) -> Vec<Signal> {
    match signal {
        ModuleItemDeclaration::Net(x) => match x {
            crate::prelude::NetDeclaration::Simple(_nettype, signed, name_and_range) => name_and_range.into_iter()
                .map(|x| Signal {
                    name: Some(x.0),
                    data_types: SignalType::Combine,//TODO
                    signed,
                    driver: Expression::NoExpr,
                }).collect(),
            crate::prelude::NetDeclaration::Range(_, _, _, _, _) => Vec::new(),//TODO:
        },
        ModuleItemDeclaration::Reg(signed, _range, variable) => variable.into_iter()
            .map(|x| match x {
                crate::prelude::VariableDeclaration::Dimension(name, _) => Signal {
                    name: Some(name),
                    data_types: SignalType::Combine,//TODO:
                    signed,
                    driver: Expression::NoExpr,
                },
                crate::prelude::VariableDeclaration::ConstExp(name, _) => Signal {
                    name: Some(name),
                    data_types: SignalType::Combine,//TODO:
                    signed,
                    driver: Expression::NoExpr,
                }
            }).collect(),
        ModuleItemDeclaration::Integer(_) => todo!(),
        ModuleItemDeclaration::Real(_) => todo!(),
        ModuleItemDeclaration::RealTime(_) => todo!(),
        ModuleItemDeclaration::Event(_) => todo!(),
    }
}
