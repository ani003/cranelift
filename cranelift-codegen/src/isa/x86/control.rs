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

pub fn expand_control_x86(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) { 
    // Unpack the instruction.
    let (func_ref, old_args) = match func.dfg[inst] {
        ir::InstructionData::Control {
            opcode,
            ref args,
            func_ref,
        } => {
            debug_assert_eq!(opcode, ir::Opcode::Control);
            // assert!(args.len(&func.dfg.value_lists) == 1);
            // TODO: also need to verify the type of the arg
            (func_ref, args.clone())
        }
        _ => panic!("Wanted control: {}", func.dfg.display_inst(inst, None)),
    };

    let ptr_ty = isa.pointer_type();
    let sig = func.dfg.ext_funcs[func_ref].signature;

    let cont_table_addr = old_args.first(&func.dfg.value_lists).unwrap();

    println!("func_ref: {:?}, args: {:?}", func_ref, old_args.len(&func.dfg.value_lists));


    // let header_pos = FuncCursor::new(func).at_inst(inst);

    // --------------- Code generation ------------------

    let mut pos = EncCursor::new(func, isa).at_inst(inst);
    pos.use_srcloc(inst);

    // Get the current continuation id
    // let k_addr = pos.ins().global_value(I64, ir::GlobalValue::with_number(0).unwrap());
    // let k_addr = pos.ins().iconst(I64, 0xdeadbeef);
    // let k = pos.ins().load(I64, ir::MemFlags::trusted(), k_addr, 0);


    // Save the current context into index k in the table
    // let entry_offset = pos.ins().imul_imm(k, 10 * 8); // TODO: 10 * 8 should be the size in bytes of each context entry.
    // let entry_offset = pos.ins().iconst(I64, 0xdeaddead);
    // let entry_addr = pos.ins().iadd(cont_table_addr, entry_offset);

    // let garbage = pos.ins().iconst(I64, 0);
    let entry_addr = cont_table_addr;

    
    // let rax_val = pos.ins().copy_to_ssa(I64, RU::rax);
    // pos.ins().store(ir::MemFlags::trusted(), rax_val, entry_addr, 0);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rax, entry_addr, 0);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rbx, entry_addr, 8);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rcx, entry_addr, 16);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rdx, entry_addr, 24);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rbp, entry_addr, 32);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rsp, entry_addr, 40);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rsi, entry_addr, 48);
    pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rdi, entry_addr, 56);

    pos.ins().ip_to_rax(0x23);

    // pos.ins().copy_reg_to_mem(ir::MemFlags::trusted(), RU::rip, entry_addr, 64);


    // let rbx_val = pos.ins().copy_to_ssa(I64, RU::rbx);
    // pos.ins().store(ir::MemFlags::trusted(), rbx_val, entry_addr, 8);
    
    // let rcx_val = pos.ins().copy_to_ssa(I64, RU::rcx);
    // pos.ins().store(ir::MemFlags::trusted(), rcx_val, entry_addr, 16);

    func.dfg.replace(inst).iconst(I32, 0xdeadbeef);

    // ...

    // let rax_addr = pos.ins().iadd_imm(entry_addr, )

    // Increment the in-memory continuation id
    // let k_updated = pos.ins().iadd_imm(k, 1);
    // pos.ins().store(ir::MemFlags::trusted(), k_updated, k_addr, 0);


    // alloc the stack, and setup %rsp
    // TODO: alloc stack, so we have something other than i64.const here
    // let newSP = pos.ins().iconst(I64, 321);

    // let stack_loc = func.locations[newSP];
    // println!("newSP loc = {:?}", stack_loc);

    // pos.ins().copy_special(RU::rsp, RU::rsp);

    // args for the function call
    // let mut new_args = ir::ValueList::default();
    // new_args.push(callee, &mut func.dfg.value_lists);
    // new_args.push(k, &mut func.dfg.value_lists);
    // for i in 1..old_args.len(&func.dfg.value_lists) {
    //     new_args.push(
    //         old_args.as_slice(&func.dfg.value_lists)[i],
    //         &mut func.dfg.value_lists,
    //     );
    // }


    // get the function address
    // let callee = pos.ins().func_addr(ptr_ty, func_ref);

    // func.dfg.replace(inst).CallIndirect(ir::Opcode::CallIndirect, ptr_ty, sig, new_args);



    println!("x86 expand control!");
    // unimplemented!()
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