use parser_rust_simple::prelude::*;

use crate::verilog::{general::identifiers::block_identifier, declaration::block_item::block_item_declaration};

use super::{statements::statement, ast::SeqBlock};

/*
/// par_block ::= fork [ : block_identifier { block_item_declaration } ] { statement } join
// TODO
//pub fn par_block() -> impl Parser<Out = String> {
//    fork()Try(Token(":")block_identifier()Many(block_item_declaration()))Many(statement())join()
//}*/

/// seq_block ::= begin [ : block_identifier { block_item_declaration } ] { statement } end
pub fn seq_block() -> impl Parser<Out = SeqBlock> {
    ((token("begin") >> Try((token(":") >> block_identifier()) * Many(block_item_declaration(), None)))
        * (Many(statement(), None) << token("end")))
        .map(SeqBlock::new)
}
