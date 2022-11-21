use parser_rust_simple::prelude::*;
use crate::verilog::expressions::expressions::constant_expression;

use super::white_space::white_space;

/// block_identifier ::= identifier
pub fn block_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// cell_identifier ::= identifier
pub fn cell_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// config_identifier ::= identifier
pub fn config_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// escaped_identifier ::= \ {Any_ASCII_character_except_white_space} white_space
pub fn escaped_identifier() -> impl Parser<Out = String> {
    Token(r"\") >> ParseRegex(r"[^ \t\n]+") << white_space()
}

/// event_identifier ::= identifier
pub fn event_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// function_identifier ::= identifier
pub fn function_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// gate_instance_identifier ::= identifier
pub fn gate_instance_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// generate_block_identifier ::= identifier
pub fn generate_block_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// genvar_identifier ::= identifier
pub fn genvar_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// hierarchical_block_identifier ::= hierarchical_identifier
pub fn hierarchical_block_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// hierarchical_event_identifier ::= hierarchical_identifier
pub fn hierarchical_event_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// hierarchical_function_identifier ::= hierarchical_identifier
pub fn hierarchical_function_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// hierarchical_identifier ::= { identifier [ [ constant_expression ] ] . } identifier
pub fn hierarchical_identifier() -> impl Parser<Out = String> {
    //TODO
    //(Many( identifier().zip(Try( token("[") >> constant_expression() << token("]")  ))
    //    .zip(token(".")).map(|x| format!("{}{}", x.0, x.1)),
    //    None
    //).map(|x| x.into_iter().reduce(|a,b| a+&b).unwrap())//TODO:unwrap()
    //* identifier())
    //.map(|x| format!("{}{}", x.0, x.1))
    identifier()
}

/// hierarchical_net_identifier ::= hierarchical_identifier
pub fn hierarchical_net_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// hierarchical_parameter_identifier ::= hierarchical_identifier
pub fn hierarchical_parameter_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// hierarchical_variable_identifier ::= hierarchical_identifier
pub fn hierarchical_variable_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// hierarchical_task_identifier ::= hierarchical_identifier
pub fn hierarchical_task_identifier() -> impl Parser<Out = String> {
    hierarchical_identifier()
}

/// identifier ::= simple_identifier | escaped_identifier
pub fn identifier() -> impl Parser<Out = String> {
    simple_identifier().or(escaped_identifier())
}

/// inout_port_identifier ::= identifier
pub fn inout_port_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// input_port_identifier ::= identifier
pub fn input_port_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// instance_identifier ::= identifier
pub fn instance_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// library_identifier ::= identifier
pub fn library_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// module_identifier ::= identifier
pub fn module_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// module_instance_identifier ::= identifier
pub fn module_instance_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// net_identifier ::= identifier
pub fn net_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// output_port_identifier ::= identifier
pub fn output_port_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// parameter_identifier ::= identifier
pub fn parameter_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// port_identifier ::= identifier
pub fn port_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// real_identifier ::= identifier
pub fn real_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// simple_identifier ::= [ a-zA-Z_ ] { [ a-zA-Z0-9_$ ] }
pub fn simple_identifier() -> impl Parser<Out = String> {
    (ParseRegex(r"[a-zA-Z_][a-zA-Z0-9_\$]*"))
        .flatmap(move |x| {
            if keywords().contains(&x) {
                Err(Location { line: usize::MAX, col: usize::MAX })//TODO:better err
            }else {
                Ok(x)
            }}) << white_space()
}

/// specparam_identifier ::= identifier
pub fn specparam_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// system_function_identifier ::= $[ a-zA-Z0-9_$ ]{ [ a-zA-Z0-9_$ ] }
pub fn system_function_identifier() -> impl Parser<Out = String> {
    (Token(r"$") * ParseRegex(r"[a-zA-Z0-9_\$]+")).map(|x| format!("{}{}", x.0, x.1))
}

/// system_task_identifier ::= $[ a-zA-Z0-9_$ ]{ [ a-zA-Z0-9_$ ] }
pub fn system_task_identifier() -> impl Parser<Out = String> {
    (Token(r"$") * ParseRegex(r"[a-zA-Z0-9_\$]+")).map(|x| format!("{}{}", x.0, x.1))
}

/// task_identifier ::= identifier
pub fn task_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// terminal_identifier ::= identifier
pub fn terminal_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// text_macro_identifier ::= identifier
pub fn text_macro_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// topmodule_identifier ::= identifier
pub fn topmodule_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// udp_identifier ::= identifier
pub fn udp_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// udp_instance_identifier ::= identifier
pub fn udp_instance_identifier() -> impl Parser<Out = String> {
    identifier()
}

/// variable_identifier ::= identifier
pub fn variable_identifier() -> impl Parser<Out = String> {
    identifier()
}

fn keywords() -> Vec<String> {
    vec!["always".to_string(),
"and".to_string(),
"assign".to_string(),
"automatic".to_string(),
"begin".to_string(),
"buf".to_string(),
"bufif0".to_string(),
"bufif1".to_string(),
"case".to_string(),
"casex".to_string(),
"casez".to_string(),
"cell".to_string(),
"cmos".to_string(),
"config".to_string(),
"deassign".to_string(),
"default".to_string(),
"defparam".to_string(),
"design".to_string(),
"disable".to_string(),
"edge".to_string(),
"else".to_string(),
"end".to_string(),
"endcase".to_string(),
"endconfig".to_string(),
"endfunction".to_string(),
"endgenerate".to_string(),
"endmodule".to_string(),
"endprimitive".to_string(),
"endspecify".to_string(),
"endtable".to_string(),
"endtask".to_string(),
"event".to_string(),
"for".to_string(),
"force".to_string(),
"forever".to_string(),
"fork".to_string(),
"function".to_string(),
"generate".to_string(),
"genvar".to_string(),
"highz0".to_string(),
"highz1".to_string(),
"if".to_string(),
"ifnone".to_string(),
"incdir".to_string(),
"include".to_string(),
"initial".to_string(),
"inout".to_string(),
"input".to_string(),
"instance".to_string(),
"integer".to_string(),
"join".to_string(),
"large".to_string(),
"liblist".to_string(),
"library".to_string(),
"localparam".to_string(),
"macromodule".to_string(),
"medium".to_string(),
"module".to_string(),
"nand".to_string(),
"negedge".to_string(),
"nmos".to_string(),
"nor".to_string(),
"noshowcancelled".to_string(),
"not".to_string(),
"notif0".to_string(),
"notif1".to_string(),
"or".to_string(),
"output".to_string(),
"parameter".to_string(),
"pmos".to_string(),
"posedge".to_string(),
"primitive".to_string(),
"pull0".to_string(),
"pull1".to_string(),
"pulldown".to_string(),
"pullup".to_string(),
"pulsestyle_onevent".to_string(),
"pulsestyle_ondetect".to_string(),
"rcmos".to_string(),
"real".to_string(),
"realtime".to_string(),
"reg".to_string(),
"release".to_string(),
"repeat".to_string(),
"rnmos".to_string(),
"rpmos".to_string(),
"rtran".to_string(),
"rtranif0".to_string(),
"rtranif1".to_string(),
"scalared".to_string(),
"showcancelled".to_string(),
"signed".to_string(),
"small".to_string(),
"specify".to_string(),
"specparam".to_string(),
"strong0".to_string(),
"strong1".to_string(),
"supply0".to_string(),
"supply1".to_string(),
"table".to_string(),
"task".to_string(),
"time".to_string(),
"tran".to_string(),
"tranif0".to_string(),
"tranif1".to_string(),
"tri".to_string(),
"tri0".to_string(),
"tri1".to_string(),
"triand".to_string(),
"trior".to_string(),
"trireg".to_string(),
"unsigned1".to_string(),
"use".to_string(),
"uwire".to_string(),
"vectored".to_string(),
"wait".to_string(),
"wand".to_string(),
"weak0".to_string(),
"weak1".to_string(),
"while".to_string(),
"wire".to_string(),
"wor".to_string(),
"xnor".to_string(),
"xor".to_string()]
}

#[test]
fn bnf() {
    use crate::ebnf_tools::ebnf;
    let input = r"
block_identifier ::= identifier
cell_identifier ::= identifier
config_identifier ::= identifier

event_identifier ::= identifier
function_identifier ::= identifier
gate_instance_identifier ::= identifier
generate_block_identifier ::= identifier
genvar_identifier ::= identifier
hierarchical_block_identifier ::= hierarchical_identifier
hierarchical_event_identifier ::= hierarchical_identifier
hierarchical_function_identifier ::= hierarchical_identifier

hierarchical_net_identifier ::= hierarchical_identifier
hierarchical_parameter_identifier ::= hierarchical_identifier
hierarchical_variable_identifier ::= hierarchical_identifier
hierarchical_task_identifier ::= hierarchical_identifier
identifier ::= simple_identifier | escaped_identifier
inout_port_identifier ::= identifier
input_port_identifier ::= identifier
instance_identifier ::= identifier
library_identifier ::= identifier
module_identifier ::= identifier
module_instance_identifier ::= identifier
net_identifier ::= identifier
output_port_identifier ::= identifier
parameter_identifier ::= identifier
port_identifier ::= identifier
real_identifier ::= identifier

specparam_identifier ::= identifier

task_identifier ::= identifier
terminal_identifier ::= identifier
text_macro_identifier ::= identifier
topmodule_identifier ::= identifier
udp_identifier ::= identifier
udp_instance_identifier ::= identifier
variable_identifier ::= identifier 


escaped_identifier ::= \ {Any_ASCII_character_except_white_space} white_space
hierarchical_identifier ::= { identifier [ [ constant_expression ] ] . } identifier
simple_identifier3 ::= [ a-zA-Z_ ] { [ a-zA-Z0-9_$ ] }
system_function_identifier4 ::= $[ a-zA-Z0-9_$ ]{ [ a-zA-Z0-9_$ ] }
system_task_identifier4 ::= $[ a-zA-Z0-9_$ ]{ [ a-zA-Z0-9_$ ] }
    ";
    let parser = ebnf(input);
    match parser {
        Ok(x) => println!("{}", x),
        Err(e) => println!("error at {:?}:\n{}", e, input.lines().nth(e.line-1).unwrap()),
    }
}

#[test]
fn test() {
    let input = r"$abc123_$x\a others";
    let parser = system_function_identifier();
    println!("{:?}", parser.run(input));
    let input = "\\abc123_*%#@ abc";
    let parser = escaped_identifier();
    println!("{:?}", parser.run(input));
    let input = "\\abc123_*%#@\tabc";
    let parser = escaped_identifier();
    println!("{:?}", parser.run(input));
    let input = "\\abc123_*%#@\nabc";
    let parser = escaped_identifier();
    println!("{:?}", parser.run(input));
    let input = "input_valid";
    let parser = simple_identifier();
    println!("{:?}", parser.run(input));
}
