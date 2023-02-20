use std::collections::HashMap;

type SigIdx = u32;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub signal_table: HashMap<String, SigIdx>,
    pub signal: HashMap<SigIdx, Signal>
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            signal_table: HashMap::new(),
            signal: HashMap::new(),
        }
    }
    pub fn update(self, operate: ModuleOperate, sig_idx: SigIdx) -> (Self,SigIdx) {
        match operate {
            ModuleOperate::SignalAdd(signal_vec) => {
                let mut module = self;
                let mut idx = sig_idx;
                signal_vec.into_iter().for_each(|s| {
                    s.name.clone().map(|signal_name| module.signal_table.insert(signal_name, idx));
                    module.signal.insert(idx, s);
                    idx += 1;});
                (module, idx)
            },
            ModuleOperate::Todo => todo!(),
        }
    }
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

pub enum ModuleOperate {
    SignalAdd(Vec<Signal>),
    Todo,
}
