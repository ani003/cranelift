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

pub fn expand_setjmp_x86(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) { 
    // Unpack the instruction.
    let context_addr = match func.dfg[inst] {
        ir::InstructionData::Unary {
            opcode,
            arg,
            // func_ref,
        } => {
            debug_assert_eq!(opcode, ir::Opcode::Setjmp);
            // assert!(args.len(&func.dfg.value_lists) == 1);
            // TODO: also need to verify the type of the arg
            arg
        }
        _ => panic!("Wanted control: {}\n{:?}", func.dfg.display_inst(inst, None), func.dfg[inst]),
    };

    // --------------- Code generation ------------------

    let mut pos = EncCursor::new(func, isa).at_inst(inst);
    pos.use_srcloc(inst);

    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rax, context_addr, 0);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rbx, context_addr, 8);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rcx, context_addr, 16);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rdx, context_addr, 24);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rbp, context_addr, 32);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rsp, context_addr, 40);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rsi, context_addr, 48);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rdi, context_addr, 56);
    pos.ins().ip_to_rax(17);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rax, context_addr, 64);

    func.dfg.replace(inst).iconst(I32, 0);
}

pub fn expand_longjmp_x86(
    inst: ir::Inst,
    func: &mut ir::Function,
    cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) { 
    println!("x86 expand restore!");

    let (context_addr, return_val) = match func.dfg[inst] {
        ir::InstructionData::Binary {
            opcode,
            args: [context_addr, ret_val],
            // func_ref,
        } => {
            debug_assert_eq!(opcode, ir::Opcode::Longjmp);
            // assert!(args.len(&func.dfg.value_lists) == 1);
            // TODO: also need to verify the type of the arg
            (context_addr, ret_val)
        }
        _ => panic!("Wanted longjmp: {}\n{:?}", func.dfg.display_inst(inst, None), func.dfg[inst]),
    };

    let mut pos = EncCursor::new(func, isa).at_inst(inst);
    pos.use_srcloc(inst);

    pos.ins().copy_mem_to_reg(ir::MemFlags::trusted(), RU::rax, context_addr, 0);
    pos.ins().copy_mem_to_reg(ir::MemFlags::trusted(), RU::rbx, context_addr, 8);
    pos.ins().copy_mem_to_reg(ir::MemFlags::trusted(), RU::rcx, context_addr, 16);


    let tmp = pos.ins().iconst(I32, 567);

    func.dfg.replace(inst).return_(&[return_val]);
}