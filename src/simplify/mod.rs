use std::collections::HashMap;



use crate::prelude::{ModuleDeclaration, PortDeclaration, NonPortModuleItem};
use self::{ast::{Module, Signal}, signal_define::{io_define, non_io_define}};

pub mod ast;
pub mod signal_define;

pub fn simplify_verilog_module(module: ModuleDeclaration) -> Module {
    match module {
        ModuleDeclaration::Ports(_, _, _, _, _) => todo!(),
        ModuleDeclaration::NonPorts(_, name, _, port, item) => {
            let (signal_table, signal) = signal_gen_non(port, item);
            Module {
                name,
                signal_table,
                signal
            }
        },
    }
}

fn signal_gen_non(port: Option<Vec<PortDeclaration>>, item: Vec<NonPortModuleItem>) -> (HashMap<String, u32>, HashMap<u32, Signal>) {
    let mut signal_table = HashMap::new();
    let mut signal = HashMap::new();
    let mut idx = 0;

    if let Some(valid_port) = port {
        valid_port.into_iter()
            .map(io_define)
            .for_each(|signal_vec| signal_vec.into_iter().for_each(|s| {
                s.name.clone().map(|signal_name| signal_table.insert(signal_name, idx));
                signal.insert(idx, s);
                idx += 1;
            }))
    }
    item.into_iter()
        .flat_map(|i| match i {
            NonPortModuleItem::ModuleGenetateItem(it) => match it {
                crate::prelude::ModuleGenetateItem::ModuleItemDeclaration(_, sig) => Some(non_io_define(sig)),
                crate::prelude::ModuleGenetateItem::ContinuousAssign(_, _) => None,
                crate::prelude::ModuleGenetateItem::Initial(_, _) => None,
                crate::prelude::ModuleGenetateItem::Always(_, _) => None,
            },
        })
        .for_each(|signal_vec| signal_vec.into_iter().for_each(|s| {
            s.name.clone().map(|signal_name| signal_table.insert(signal_name, idx));
            signal.insert(idx, s);
            idx += 1;
        }));
    (signal_table, signal)
}

#[test]
fn test() {
    
    use parser_rust_simple::prelude::Parser;
    use crate::verilog::source_text::verilog_source::module_declaration;

    let src = r"module getK (
  input      [31:0]   io_K_0,
  input      [31:0]   io_K_1,
  input      [31:0]   io_K_2,
  input      [31:0]   io_K_3,
  output     [31:0]   io_Klist_0,
  output     [31:0]   io_Klist_1,
  output     [31:0]   io_Klist_2,
  output     [31:0]   io_Klist_3,
  output     [31:0]   io_Klist_4,
  input               clk,
  input               reset
);
  reg        [7:0]    _zz_Tout_getTAU_Sbox_port0;
  reg        [7:0]    _zz_Tout_getTAU_Sbox_port1;
  reg        [7:0]    _zz_Tout_getTAU_Sbox_port2;
  reg        [7:0]    _zz_Tout_getTAU_Sbox_port3;
  wire                _zz_Tout_getTAU_Sbox_port;
  wire                _zz_Tout_getTAU_SboxOut_5;
  wire                _zz_Tout_getTAU_Sbox_port_1;
  wire                _zz_Tout_getTAU_SboxOut_6;
  wire                _zz_Tout_getTAU_Sbox_port_2;
  wire                _zz_Tout_getTAU_SboxOut_7;
  wire                _zz_Tout_getTAU_Sbox_port_3;
  wire                _zz_Tout_getTAU_SboxOut_8;
  reg        [31:0]   Klist_0;
  reg        [31:0]   Klist_1;
  reg        [31:0]   Klist_2;
  reg        [31:0]   Klist_3;
  reg        [31:0]   Klist_4;
  reg        [31:0]   io_K_0_delay_1;
  reg        [31:0]   io_K_0_delay_2;
  reg        [31:0]   io_K_0_delay_3;
  reg        [31:0]   io_K_1_delay_1;
  reg        [31:0]   io_K_1_delay_2;
  reg        [31:0]   io_K_1_delay_3;
  reg        [31:0]   io_K_2_delay_1;
  reg        [31:0]   io_K_2_delay_2;
  reg        [31:0]   io_K_2_delay_3;
  reg        [31:0]   io_K_3_delay_1;
  reg        [31:0]   io_K_3_delay_2;
  reg        [31:0]   io_K_3_delay_3;
  reg        [31:0]   _zz_Tout_getTAU_SboxOut;
  reg        [31:0]   Tout_getTAU_SboxOut;
  wire       [7:0]    _zz_Tout_getTAU_SboxOut_1;
  wire       [7:0]    _zz_Tout_getTAU_SboxOut_2;
  wire       [7:0]    _zz_Tout_getTAU_SboxOut_3;
  wire       [7:0]    _zz_Tout_getTAU_SboxOut_4;
  reg        [31:0]   Tout_ret;
  reg        [31:0]   io_K_0_delay_1_1;
  reg        [31:0]   io_K_0_delay_2_1;
  reg        [31:0]   io_K_0_delay_3_1;
  reg [7:0] Tout_getTAU_Sbox [0:255];

  assign _zz_Tout_getTAU_SboxOut_5 = 1'b1;
  assign _zz_Tout_getTAU_SboxOut_6 = 1'b1;
  assign _zz_Tout_getTAU_SboxOut_7 = 1'b1;
  assign _zz_Tout_getTAU_SboxOut_8 = 1'b1;
  always @(posedge clk) begin
    if(_zz_Tout_getTAU_SboxOut_5) begin
      _zz_Tout_getTAU_Sbox_port0 <= Tout_getTAU_Sbox[_zz_Tout_getTAU_SboxOut_1];
    end
  end

  always @(posedge clk) begin
    if(_zz_Tout_getTAU_SboxOut_6) begin
      _zz_Tout_getTAU_Sbox_port1 <= Tout_getTAU_Sbox[_zz_Tout_getTAU_SboxOut_2];
    end
  end

  always @(posedge clk) begin
    if(_zz_Tout_getTAU_SboxOut_7) begin
      _zz_Tout_getTAU_Sbox_port2 <= Tout_getTAU_Sbox[_zz_Tout_getTAU_SboxOut_3];
    end
  end

  always @(posedge clk) begin
    if(_zz_Tout_getTAU_SboxOut_8) begin
      _zz_Tout_getTAU_Sbox_port3 <= Tout_getTAU_Sbox[_zz_Tout_getTAU_SboxOut_4];
    end
  end

  assign _zz_Tout_getTAU_SboxOut_1 = _zz_Tout_getTAU_SboxOut[7 : 0];
  always @(*) begin
    Tout_getTAU_SboxOut[7 : 0] = _zz_Tout_getTAU_Sbox_port0;
    Tout_getTAU_SboxOut[15 : 8] = _zz_Tout_getTAU_Sbox_port1;
    Tout_getTAU_SboxOut[23 : 16] = _zz_Tout_getTAU_Sbox_port2;
    Tout_getTAU_SboxOut[31 : 24] = _zz_Tout_getTAU_Sbox_port3;
  end

  assign _zz_Tout_getTAU_SboxOut_2 = _zz_Tout_getTAU_SboxOut[15 : 8];
  assign _zz_Tout_getTAU_SboxOut_3 = _zz_Tout_getTAU_SboxOut[23 : 16];
  assign _zz_Tout_getTAU_SboxOut_4 = _zz_Tout_getTAU_SboxOut[31 : 24];
  assign io_Klist_0 = Klist_0;
  assign io_Klist_1 = Klist_1;
  assign io_Klist_2 = Klist_2;
  assign io_Klist_3 = Klist_3;
  assign io_Klist_4 = Klist_4;
  always @(posedge clk) begin
    io_K_0_delay_1 <= io_K_0;
    io_K_0_delay_2 <= io_K_0_delay_1;
    io_K_0_delay_3 <= io_K_0_delay_2;
    Klist_0 <= io_K_0_delay_3;
    io_K_1_delay_1 <= io_K_1;
    io_K_1_delay_2 <= io_K_1_delay_1;
    io_K_1_delay_3 <= io_K_1_delay_2;
    Klist_1 <= io_K_1_delay_3;
    io_K_2_delay_1 <= io_K_2;
    io_K_2_delay_2 <= io_K_2_delay_1;
    io_K_2_delay_3 <= io_K_2_delay_2;
    Klist_2 <= io_K_2_delay_3;
    io_K_3_delay_1 <= io_K_3;
    io_K_3_delay_2 <= io_K_3_delay_1;
    io_K_3_delay_3 <= io_K_3_delay_2;
    Klist_3 <= io_K_3_delay_3;
    _zz_Tout_getTAU_SboxOut <= (((io_K_1 ^ io_K_2) ^ io_K_3) ^ 32'h00070e15);
    Tout_ret <= ((Tout_getTAU_SboxOut ^ {Tout_getTAU_SboxOut[18 : 0],Tout_getTAU_SboxOut[31 : 19]}) ^ {Tout_getTAU_SboxOut[8 : 0],Tout_getTAU_SboxOut[31 : 9]});
    io_K_0_delay_1_1 <= io_K_0;
    io_K_0_delay_2_1 <= io_K_0_delay_1_1;
    io_K_0_delay_3_1 <= io_K_0_delay_2_1;
    Klist_4 <= (io_K_0_delay_3_1 ^ Tout_ret);
  end


endmodule";

    let ast = module_declaration().run(src);
    let to_module = ast.map(simplify_verilog_module);
    println!("{to_module:#?}");

}
