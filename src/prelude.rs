pub use crate::verilog::source_text::ast::*;
pub use crate::verilog::declaration::ast::*;
//pub use crate::verilog::module_instantiation_generate_construct::ast::*;
pub use crate::verilog::behavioral_statements::ast::*;
pub use crate::verilog::expressions::ast::*;
pub use crate::verilog::general::ast::*;

use crate::verilog::source_text::verilog_source::module_declaration;
pub use parser_rust_simple::prelude::*;

pub fn singla_module_parser() -> impl Parser<Out = ModuleDeclaration> {
    whitespace() >> module_declaration()
}
