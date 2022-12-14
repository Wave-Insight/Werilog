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
        * (Many(tobox!(statement()), None) << token("end")))
        .map(SeqBlock::new)
}

#[test]
fn test() {
    let input = r"begin
    _zz_Tout_getTAU_Sbox_port0 <= Tout_getTAU_Sbox;
    end
    ";
    println!("{:?}", seq_block().run_with_out(input, Location::new()))
}
