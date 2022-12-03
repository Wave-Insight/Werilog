use parser_rust_simple::prelude::*;

use crate::verilog::expressions::{expression_left_side_values::net_lvalue, expressions::expression};

use super::ast::NetAssign;

/// continuous_assign ::= assign [ drive_strength ] [ delay3 ] list_of_net_assignments ;
pub fn continuous_assign() -> impl Parser<Out = Vec<NetAssign>> {
    token("assign") >>
        //Try(drive_strength()) *//TODO
        //Try(delay3()) *//TODO
        list_of_net_assignments() << token(";")
}


/// list_of_net_assignments ::= net_assignment { , net_assignment }
pub fn list_of_net_assignments() -> impl Parser<Out = Vec<NetAssign>> {
    Many(net_assignment(), Some(","))
}

/// net_assignment ::= net_lvalue = expression
pub fn net_assignment() -> impl Parser<Out = NetAssign> {
    (whitespace() >> net_lvalue().left(whitespace())
        .zip(token("=") >> tobox!(expression()).left(whitespace())))
        .map(|(a,b)| NetAssign(a, b))
}

#[test]
fn test() {
    fn print_parse(input: &str) {
        println!("{:?}", net_assignment().run(input));
    }
    print_parse("mema = 0;"); // Illegal syntax- Attempt to write to entire array
    print_parse("arrayb[1] = 0;"); // Illegal Syntax - Attempt to write to elements
                                         // [1][0]..[1][255]
    print_parse("arrayb[1][12:31] = 0;"); // Illegal Syntax - Attempt to write to
                                                // elements [1][12]..[1][31]
    print_parse("mema[1] = 0;"); // Assigns 0 to the second element of mema
    print_parse("arrayb[1][0] = 0;"); // Assigns 0 to the bit referenced by indices
                                            // [1][0]
    print_parse("inta[4] = 33559;"); // Assign decimal number to integer in array
    print_parse("chng_hist[t_index] = $time;"); // Assign current simulation time to
                                                      // element addressed by integer index

    let input = "_zz_Tout_getTAU_SboxOut_5 = 1'b1;";
    println!("{:?}", net_assignment().run_with_out(input, Location::new()));
    let input = "assign _zz_Tout_getTAU_SboxOut_5 = 1'b1;";
    println!("{:?}", continuous_assign().run_with_out(input, Location::new()));
}
