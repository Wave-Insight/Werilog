use parser_rust_simple::prelude::*;
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
    (Many( identifier()//TODO: * Try( Token("[") * constant_expression() * Token("]")  )
        .zip(Token(".")).map(|x| format!("{}{}", x.0, x.1)),
        None
    ).map(|x| x.into_iter().reduce(|a,b| a+&b).unwrap())//TODO:unwrap()
    * identifier())
    .map(|x| format!("{}{}", x.0, x.1))
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
    (ParseRegex(r"[a-zA-Z_]") *
        Many(ParseRegex(r"[a-zA-Z_0-9\$]"),None)
            .map(|x| x.into_iter()
                                    .reduce(|a,b| a+&b)
                                    .unwrap_or_else(|| "".to_string()))
    ).map(|x| x.0+&x.1)
    
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
}
