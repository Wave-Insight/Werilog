use parser_rust_simple::prelude::*;

use crate::verilog::{general::attributes::attribute_instance, behavioral_statements::{continuous_assignment::continuous_assign, procedural_blocks::{initial_construct, always_construct}}, declaration::types::{reg_declaration, integer_declaration, real_declaration, realtime_declaration, event_declaration, net_declaration}};

use super::{module_parameters_ports::port_declaration, ast::{ModuleItem, ModuleItemDeclaration, ModuleGenetateItem, NonPortModuleItem}};



/// module_item ::=
///     port_declaration ;
///   | non_port_module_item
pub fn module_item() -> impl Parser<Out = ModuleItem> {
    port_declaration().left(token(";")).map(ModuleItem::PortDeclaration)
        //TODO:| non_port_module_item()
}

/// module_or_generate_item ::=
///     { attribute_instance } module_or_generate_item_declaration
///   | { attribute_instance } local_parameter_declaration ;
///   | { attribute_instance } parameter_override
///   | { attribute_instance } continuous_assign
///   | { attribute_instance } gate_instantiation
///   | { attribute_instance } udp_instantiation
///   | { attribute_instance } module_instantiation
///   | { attribute_instance } initial_construct
///   | { attribute_instance } always_construct
///   | { attribute_instance } loop_generate_construct
///   | { attribute_instance } conditional_generate_construct
pub fn module_or_generate_item() -> impl Parser<Out = ModuleGenetateItem> {
    (Many(attribute_instance(), None) * module_or_generate_item_declaration())
        .map(|x| ModuleGenetateItem::ModuleItemDeclaration(x.0, x.1))
        //| (Many(attribute_instance(), None) * (local_parameter_declaration() << token(";")))
        //| (Many(attribute_instance(), None) * parameter_override())
        | (Many(attribute_instance(), None) * continuous_assign())
            .map(|x| ModuleGenetateItem::ContinuousAssign(x.0, x.1))
        //| (Many(attribute_instance(), None) * gate_instantiation())
        //TODO| (Many(attribute_instance(), None) * udp_instantiation())
        //TODO| (Many(attribute_instance(), None) * module_instantiation())
        | (Many(attribute_instance(), None) * initial_construct())
            .map(|x| ModuleGenetateItem::Initial(x.0, x.1))
        | (Many(attribute_instance(), None) * always_construct())
            .map(|x| ModuleGenetateItem::Always(x.0, x.1))
        //TODO| (Many(attribute_instance(), None) * loop_generate_construct())
        //TODO| (Many(attribute_instance(), None) * conditional_generate_construct())
}

/// module_or_generate_item_declaration ::=
///     net_declaration
///   | reg_declaration
///   | integer_declaration
///   | real_declaration
///   | time_declaration
///   | realtime_declaration
///   | event_declaration
///   | genvar_declaration
///   | task_declaration
///   | function_declaration 
pub fn module_or_generate_item_declaration() -> impl Parser<Out = ModuleItemDeclaration> {
    net_declaration().map(ModuleItemDeclaration::Net)
        | reg_declaration().map(|x| ModuleItemDeclaration::Reg(x.0.0, x.0.1, x.1))
        | integer_declaration().map(ModuleItemDeclaration::Integer)
        | real_declaration().map(ModuleItemDeclaration::Real)
        //TODO:| time_declaration()
        | realtime_declaration().map(ModuleItemDeclaration::RealTime)
        | event_declaration().map(ModuleItemDeclaration::Event)
        //TODO:| genvar_declaration()
        //TODO:| task_declaration()
        //TODO:| function_declaration()
}

/// non_port_module_item ::=
///     module_or_generate_item
///   | generate_region
///   | specify_block
///   | { attribute_instance } parameter_declaration ;
///   | { attribute_instance } specparam_declaration 
pub fn non_port_module_item() -> impl Parser<Out = NonPortModuleItem> {
    module_or_generate_item().map(NonPortModuleItem::ModuleGenetateItem)
        //TODO:| generate_region()
        //TODO:| specify_block()
        //TODO:| Many(attribute_instance(), None) * parameter_declaration() << token(";")
        //TODO:| Many(attribute_instance(), None) * specparam_declaration()
}
/*TODO
/// parameter_override ::= defparam list_of_defparam_assignments ;
pub fn parameter_override() -> impl Parser<Out = String> {
    token("defparam") >> list_of_defparam_assignments() << token(";")
}*/

#[test]
fn test() {
    println!("{:?}", module_or_generate_item_declaration().run("integer a;")); // integer value
    //println!("{:?}", module_or_generate_item_declaration().run("time last_chng;")); // time value
    println!("{:?}", module_or_generate_item_declaration().run("real float ;")); // a variable to store a real value
    println!("{:?}", module_or_generate_item_declaration().run("realtime rtime ;")); // a variable to store time as a real value

    println!("{:?}", module_or_generate_item_declaration().run("reg x[11:0];")); // scalar reg
    println!("{:?}", module_or_generate_item_declaration().run("wire [0:7] y[5:0];")); // 8-bit-wide vector wire indexed from 0 to 7
    println!("{:?}", module_or_generate_item_declaration().run("reg [31:0] x [127:0];")); // 32-bit-wide reg

    println!("{:?}", module_or_generate_item_declaration().run("reg [7:0] mema[0:255];")); // declares a memory mema of 256 8-bit
                                                                                           // registers. The indices are 0 to 255
    println!("{:?}", module_or_generate_item_declaration().run("reg arrayb[7:0][0:255];")); // declare a two-dimensional array of
                                                                                            // one bit registers
    println!("{:?}", module_or_generate_item_declaration().run("wire w_array[7:0][5:0];")); // declare array of wires
    println!("{:?}", module_or_generate_item_declaration().run("integer inta[1:64];")); // an array of 64 integer values
    //println!("{:?}", module_or_generate_item_declaration().run("time chng_hist[1:1000];")); // an array of 1000 time values
    println!("{:?}", module_or_generate_item_declaration().run("integer t_index;"));

    let input = r"assign _zz_Tout_getTAU_SboxOut_1 = _zz_Tout_getTAU_SboxOut[7 : 0];";
    println!("{:?}", continuous_assign().run_with_out(input, Location::new()));
    let input = r"always @(*) begin
    Tout_getTAU_SboxOut[7 : 0] = _zz_Tout_getTAU_Sbox_port0;
    Tout_getTAU_SboxOut[15 : 8] = _zz_Tout_getTAU_Sbox_port1;
    Tout_getTAU_SboxOut[23 : 16] = _zz_Tout_getTAU_Sbox_port2;
    Tout_getTAU_SboxOut[31 : 24] = _zz_Tout_getTAU_Sbox_port3;
  end";
    println!("{:?}", always_construct().run_with_out(input, Location::new()));
    let input = r"always @(posedge clk) begin
    Klist_4 <= (io_K_0_delay_3_1 ^ Tout_ret);
  end";
    println!("{:?}", always_construct().run_with_out(input, Location::new()));
}
