//! Legalization of control.
//!
//! This module exports the `expand_control` function which transforms a `control`
//! instruction into...

use crate::cursor::{Cursor, FuncCursor};
use crate::flowgraph::ControlFlowGraph;
use crate::ir::{self, InstBuilder};
use crate::isa::{TargetIsa};
use crate::ir::types::{I32, I64};

// use crate::isa::StackBase;

// use crate::isa::{CallConv, RegClass, RegUnit, TargetIsa};

/// Expand a `call` instruction. This lowers it to a `call_indirect`, which
/// is only done if the ABI doesn't support direct calls.

fn _expand_control(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) {
    println!("Expanding control!");
    panic!();
    // Unpack the instruction.
    let (func_ref, old_args) = match func.dfg[inst] {
        ir::InstructionData::Call {
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

    let callee = {
        let mut pos = FuncCursor::new(func).at_inst(inst);
        pos.use_srcloc(inst);
        pos.ins().func_addr(ptr_ty, func_ref)
    };

    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);

    let k = pos.ins().iconst(I32, 0); // TODO: generate continuation id

    let newSP = pos.ins().iconst(I64, 321); // TODO: alloc stack

    // pos.

    // pos.ins()
    //     .copy_special(StackBase::SP as RegUnit, RU::rbp as RegUnit);

    // Emit a function call to the given function, with continuation id as first arg, and provided args after.
    let mut new_args = ir::ValueList::default();
    new_args.push(callee, &mut func.dfg.value_lists);
    new_args.push(k, &mut func.dfg.value_lists);
    for i in 0..old_args.len(&func.dfg.value_lists) {
        new_args.push(
            old_args.as_slice(&func.dfg.value_lists)[i],
            &mut func.dfg.value_lists,
        );
    }

    func.dfg
        .replace(inst)
        .CallIndirect(ir::Opcode::CallIndirect, ptr_ty, sig, new_args);
}



/// Expand a `restore` instruction...
fn _expand_restore(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) {
    println!("Expanding restore!");

    // Unpack the instruction.
    let old_args = match func.dfg[inst] {
        ir::InstructionData::MultiAry {
            opcode: ir::Opcode::Restore, ref args
        } => {
            // let args_slice = args.as_slice(&func.dfg.value_lists);
            // debug_assert!(args_slice.len() >= 1); // this should have already been checked by verifier
            args.clone()
        }
        _ => panic!("Wanted restore: {}", func.dfg.display_inst(inst, None)),
    };

    let ptr_ty = isa.pointer_type();

    // let sig = func.dfg.ext_funcs[func_ref].signature;

    // let callee = {
    //     let mut pos = FuncCursor::new(func).at_inst(inst);
    //     pos.use_srcloc(inst);
    //     pos.ins().func_addr(ptr_ty, func_ref)
    // };

    // let mut pos = FuncCursor::new(func).at_inst(inst);
    // pos.use_srcloc(inst);

    // use crate::ir::types::I32;
    // let const0 = pos.ins().iconst(I32, 0);

    let mut k: Option<ir::Value> = None;
    let mut new_args = ir::ValueList::default();

    for i in 0..old_args.len(&func.dfg.value_lists) {
        if i == 0 {
            k = Some(old_args.as_slice(&func.dfg.value_lists)[i])
        } else {
            new_args.push(
                old_args.as_slice(&func.dfg.value_lists)[i],
                &mut func.dfg.value_lists,
            );
        }
    }

    let k = k.unwrap();

    // new_args.push(callee, &mut func.dfg.value_lists);
    // new_args.push(const0, &mut func.dfg.value_lists);
    // // new_args.push(old_arg, &mut func.dfg.value_lists);

    // // let mut new_args = ir::ValueList::default();
    // // new_args.push(callee, &mut func.dfg.value_lists);
    // for i in 0..old_args.len(&func.dfg.value_lists) {
    //     new_args.push(
    //         old_args.as_slice(&func.dfg.value_lists)[i],
    //         &mut func.dfg.value_lists,
    //     );
    // }

    func.dfg
        .replace(inst)
        .MultiAry(ir::Opcode::Return, ptr_ty, new_args);
        // .CallIndirect(ir::Opcode::CallIndirect, ptr_ty, sig, new_args);


    // func.dfg.replace(inst).Return(ir::)
}
