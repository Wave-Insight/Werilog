use parser_rust_simple::prelude::*;

use crate::verilog::{general::{attributes::attribute_instance, identifiers::port_identifier}, declaration::{types::{inout_declaration, input_declaration, output_declaration, parameter_declaration}, ast::ParameterDeclaration}, expressions::{expressions::constant_range_expression, ast::ConstantRangeExpression}};

use super::ast::{PortDeclaration, Port};


/// module_parameter_port_list ::= # ( parameter_declaration { , parameter_declaration } )
pub fn module_parameter_port_list() -> impl Parser<Out = Vec<ParameterDeclaration>> {
    token("#") >> token("(") >> Many(parameter_declaration(), Some(",")) << token(")")
}

/// list_of_ports ::= ( port { , port } )
pub fn list_of_ports() -> impl Parser<Out = Vec<Port>> {
    token("(") >> Many(port(), Some(",")) << token(")")
}

/// list_of_port_declarations ::=
///     ( port_declaration { , port_declaration } )
///   | ( )
pub fn list_of_port_declarations() -> impl Parser<Out = Vec<PortDeclaration>> {
    token("(") >> Many(port_declaration(), Some(",")) << token(")")
        //TODO:| token("(") >> token(")")
}

/// port ::=
///     [ port_expression ]
///   | . port_identifier ( [ port_expression ] )
pub fn port() -> impl Parser<Out = Port> {
    Try(port_expression()).map(|x| Port(None, x))
        | ((token(".") >> port_identifier().map(Some)) * (token("(") >> Try(port_expression()) << token(")")))
            .map(|x| Port(x.0, x.1))
}

/// port_expression ::=
///     port_reference
///   | { port_reference { , port_reference } }
pub fn port_expression() -> impl Parser<Out = Vec<(String, Option<ConstantRangeExpression>)>> {
    port_reference().map(|x| vec![x])
        .or(token("{") >> Many(port_reference(), Some(",")) << token("}"))
}

/// port_reference ::= port_identifier [ [ constant_range_expression ] ]
pub fn port_reference() -> impl Parser<Out = (String, Option<ConstantRangeExpression>)> {
    port_identifier().zip(token("[") >> Try(constant_range_expression()) << token("]"))
}

/// port_declaration ::=
///     {attribute_instance} inout_declaration
///   | {attribute_instance} input_declaration
///   | {attribute_instance} output_declaration 
pub fn port_declaration() -> impl Parser<Out = PortDeclaration> {
    (Many(attribute_instance(), None) * inout_declaration()).map(|x| PortDeclaration::Inout(x.0, x.1))
        | (Many(attribute_instance(), None) * input_declaration()).map(|x| PortDeclaration::Input(x.0, x.1))
        | (Many(attribute_instance(), None) * output_declaration()).map(|x| PortDeclaration::Output(x.0, x.1))
}

#[test]
fn test() {
    let input = "input               io_input_valid";
    println!("{:?}", port_declaration().run(input));
    let input = "input      [31:0]   io_input_payload_X_0";
    println!("{:?}", port_declaration().run(input));
    let input = "(
  input               io_input_valid,
  input      [31:0]   io_input_payload_X_0,
  input      [31:0]   io_input_payload_X_1,
  input      [31:0]   io_input_payload_X_2,
  input      [31:0]   io_input_payload_X_3,
  input      [31:0]   io_input_payload_MK_0,
  input      [31:0]   io_input_payload_MK_1,
  input      [31:0]   io_input_payload_MK_2,
  input      [31:0]   io_input_payload_MK_3,
  output              io_encoutput_valid,
  output     [31:0]   io_encoutput_payload_0,
  output     [31:0]   io_encoutput_payload_1,
  output     [31:0]   io_encoutput_payload_2,
  output     [31:0]   io_encoutput_payload_3,
  output              io_output_valid,
  output     [31:0]   io_output_payload_0,
  output     [31:0]   io_output_payload_1,
  output     [31:0]   io_output_payload_2,
  output     [31:0]   io_output_payload_3,
  input               clk,
  input               reset
)
    ";
    println!("{:?}", list_of_port_declarations().run(input));
}
