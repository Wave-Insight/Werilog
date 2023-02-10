use std::collections::HashMap;

type SigIdx = u32;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub signal_table: HashMap<String, SigIdx>,
    pub signal: HashMap<SigIdx, Signal>
}

#[derive(Debug)]
pub struct Signal {
    pub name: Option<String>,
    pub data_types: SignalType,
    pub signed: bool,
    //pub size
    pub driver: Expression,
}

#[derive(Debug)]
pub enum SignalType {
    Input,
    Output,
    OutputReg,//flipflop
    Inout,
    Combine,
    FlipFlop,
    Latch,
    Unknown,
}

impl SignalType {
    pub fn update(self, that: Self) -> Result<Self,Self> {
        match self {
            SignalType::Unknown => Ok(that),
            x => Err(x),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    NoExpr,
    Unary(String, SigIdx),//TODO:String -> Op
    Binary(String, SigIdx, SigIdx),//TODO:String -> Op
    Mux(SigIdx, SigIdx, SigIdx),
}
