use super::registers::{FPR, GPR, RU};
// use crate::bitset::BitSet;
use crate::cursor::{Cursor, FuncCursor, EncCursor};
use crate::flowgraph::ControlFlowGraph;
// use crate::ir::condcodes::{FloatCC, IntCC};
use crate::ir;
use crate::ir::{Function, Inst, InstBuilder};
// use crate::isa::constraints::*;
// use crate::isa::enc_tables::*;
// use crate::isa::encoding::base_size;
// use crate::isa::encoding::RecipeSizing;
// use crate::isa::RegUnit;
// use crate::isa;
use crate::isa::TargetIsa;
use crate::ir::types::*;
// use crate::predicates;
// use crate::regalloc::RegDiversions;


pub fn expand_copy_reg_to_mem_x86(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) {
    let mut pos = EncCursor::new(func, isa).at_inst(inst);
    pos.use_srcloc(inst);

    println!("Legalizing copy_reg_to_mem");

    func.dfg.replace(inst).nop();
}
