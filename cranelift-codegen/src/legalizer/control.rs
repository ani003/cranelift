//! Legalization of control.
//!
//! This module exports the `expand_control` function which transforms a `control`
//! instruction into...

use crate::cursor::{Cursor, FuncCursor};
use crate::flowgraph::ControlFlowGraph;
use crate::ir::{self, InstBuilder};
use crate::isa::TargetIsa;

/// Expand a `call` instruction. This lowers it to a `call_indirect`, which
/// is only done if the ABI doesn't support direct calls.
pub fn expand_control(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) {
    println!("Expanding control!");

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

    use crate::ir::types::I32;
    let const0 = pos.ins().iconst(I32, 0);

    let mut new_args = ir::ValueList::default();
    new_args.push(callee, &mut func.dfg.value_lists);
    new_args.push(const0, &mut func.dfg.value_lists);
    // new_args.push(old_arg, &mut func.dfg.value_lists);

    // let mut new_args = ir::ValueList::default();
    // new_args.push(callee, &mut func.dfg.value_lists);
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
