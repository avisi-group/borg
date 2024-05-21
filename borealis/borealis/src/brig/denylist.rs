use sailrs::{
    jib_ast::{self, Definition, Instruction, InstructionAux, Type, Value, Vl},
    sail_ast::Location,
    types::ListVec,
};

pub fn apply_fn_denylist<I: Iterator<Item = jib_ast::Definition>>(
    iter: I,
) -> impl Iterator<Item = jib_ast::Definition> {
    iter.map(|def| {
        if let Definition::Fundef(name, idk, arguments, body) = def {
            let body = if !DENYLIST.contains(&name.as_interned().as_ref()) {
                body
            } else {
                ListVec::from(vec![Instruction {
                    inner: InstructionAux::Throw(Value::Lit(Vl::Unit, Type::Unit)),
                    annot: (0, Location::Unknown),
                }])
            };

            Definition::Fundef(name, idk, arguments, body)
        } else {
            def
        }
    })
}

const DENYLIST: &[&'static str] = &[
    "integer_update_subrange",
    "ExecuteA64", // remove me next
    "ExecuteA32",
    "ExecuteT32__1",
    "ExecuteT16",
    "__FetchInstr",
    "__TryDecodeExecute",
    "__DecodeExecute",
    "__CheckForEmulatorTermination",
    "__InstructionExecute",
    "__TopLevel",
    "__CycleEnd",
    "AccessDescriptor_to_Access_kind",
    "sail_mem_read",
    "read_request",
    "sail_mem_write",
    "write_request",
    "take_exception",
    "gic_readonly",
    "__ReadUART",
    "__WriteUART",
    "IsPhysicalSErrorPending",
    "ClearPendingPhysicalSError",
    "__UpdateSystemCounter",
    "SetInterruptRequestLevel",
    "SpeculationBarrier",
    "SupportedPowerTwoSVL",
    "PhysMemTagWrite",
    "PhysMemTagRead",
    "__EndCycle",
    "__ListConfig",
    "__Reset",
    "ActionRequired",
    "DiscardTransactionalWrites",
    "EnterLowPowerState",
    "LocalTimeoutEvent",
    "MarkExclusiveGlobal",
    "MarkExclusiveLocal",
    "PendSErrorInterrupt",
    "ProfilingSynchronizationBarrier",
    "RemapRegsHaveResetValues",
    "step_model",
    "main",
];
