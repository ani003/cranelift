// use super::registers::*;
// use crate::bitset::BitSet;
// use crate::cursor::{Cursor, FuncCursor};
use crate::flowgraph::ControlFlowGraph;
// use crate::ir::condcodes::{FloatCC, IntCC};
use crate::ir;
// use crate::ir::{Function, Inst, InstBuilder};
// use crate::isa::constraints::*;
// use crate::isa::enc_tables::*;
// use crate::isa::encoding::base_size;
// use crate::isa::encoding::RecipeSizing;
// use crate::isa::RegUnit;
// use crate::isa;
use crate::isa::TargetIsa;
// use crate::predicates;
// use crate::regalloc::RegDiversions;


pub fn expand_control_x86(
    _inst: ir::Inst,
    _func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    _isa: &dyn TargetIsa,
) { 

    println!("x86 expand control!");
    unimplemented!()
}

pub fn expand_restore_x86(
    _inst: ir::Inst,
    _func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    _isa: &dyn TargetIsa,
) { 
    println!("x86 expand restore!");
    unimplemented!()
}