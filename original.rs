#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use {
    super::{
        common::*, u__DecodeA64_BranchExcSys::*, u__DecodeA64_DataProcFPSIMD::*,
        u__DecodeA64_DataProcImm::*, u__DecodeA64_DataProcReg::*, u__DecodeA64_LoadStore::*,
        u__DecodeA64_Reserved::*, u__DecodeA64_SME::*, u__DecodeA64_SVE::*,
        u__DecodeA64_Unallocated1::*, u__DecodeA64_Unallocated2::*,
    },
    crate::dbt::{
        emitter::{BlockResult, Emitter, Flag, Type, TypeKind},
        x86::{
            emitter::{
                BinaryOperationKind, CastOperationKind, ShiftOperationKind, UnaryOperationKind,
                X86BlockRef, X86Emitter, X86NodeRef, X86SymbolRef,
            },
            X86TranslationContext,
        },
        TranslationContext,
    },
};
#[inline(never)]
pub fn u__DecodeA64(
    ctx: &mut X86TranslationContext,
    pc: X86NodeRef,
    opcode: X86NodeRef,
) -> X86NodeRef {
    struct FunctionState {
        v__0: X86SymbolRef,
        return_: X86SymbolRef,
        v__3: X86SymbolRef,
        gs_249876: X86SymbolRef,
        v__21: X86SymbolRef,
        pc: X86SymbolRef,
        opcode: X86SymbolRef,
        borealis_fn_return_value: X86SymbolRef,
        block_refs: [X86BlockRef; 24usize],
        exit_block_ref: X86BlockRef,
    }
    let fn_state = FunctionState {
        v__0: ctx.create_symbol(),
        return_: ctx.create_symbol(),
        v__3: ctx.create_symbol(),
        gs_249876: ctx.create_symbol(),
        v__21: ctx.create_symbol(),
        pc: ctx.create_symbol(),
        opcode: ctx.create_symbol(),
        borealis_fn_return_value: ctx.create_symbol(),
        block_refs: [
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
            ctx.create_block(),
        ],
        exit_block_ref: ctx.create_block(),
    };
    {
        let emitter = ctx.emitter();
        ctx.emitter().write_variable(fn_state.pc.clone(), pc);
        ctx.emitter()
            .write_variable(fn_state.opcode.clone(), opcode);
    }
    const BLOCK_FUNCTIONS: [fn(&mut X86TranslationContext, &FunctionState) -> BlockResult;
        24usize] = [
        block_0, block_1, block_2, block_3, block_4, block_5, block_6, block_7, block_8, block_9,
        block_10, block_11, block_12, block_13, block_14, block_15, block_16, block_17, block_18,
        block_19, block_20, block_21, block_22, block_23,
    ];
    fn lookup_block_idx_by_ref(block_refs: &[X86BlockRef], block: X86BlockRef) -> usize {
        block_refs.iter().position(|r| *r == block).unwrap()
    }
    enum Block {
        Static(usize),
        Dynamic(usize),
    }
    let mut block_queue = alloc::vec![Block::Static(0)];
    while let Some(block) = block_queue.pop() {
        let result = match block {
            Block::Static(i) => {
                log::debug!("static block {i}");
                BLOCK_FUNCTIONS[i](ctx, &fn_state)
            }
            Block::Dynamic(i) => {
                log::debug!("dynamic block {i}");
                ctx.emitter()
                    .set_current_block(fn_state.block_refs[i].clone());
                BLOCK_FUNCTIONS[i](ctx, &fn_state)
            }
        };
        match result {
            BlockResult::Static(block) => {
                let idx = lookup_block_idx_by_ref(&fn_state.block_refs, block);
                log::debug!("block result: static({idx})");
                block_queue.push(Block::Static(idx));
            }
            BlockResult::Dynamic(b0, b1) => {
                let i0 = lookup_block_idx_by_ref(&fn_state.block_refs, b0);
                let i1 = lookup_block_idx_by_ref(&fn_state.block_refs, b1);
                log::debug!("block result: dynamic({i0}, {i1})");
                block_queue.push(Block::Dynamic(i0));
                block_queue.push(Block::Dynamic(i1));
            }
            BlockResult::Return => {
                log::debug!("block result: return");
                ctx.emitter().jump(fn_state.exit_block_ref.clone());
            }
            BlockResult::Panic => {
                log::debug!("block result: panic");
                ctx.emitter().jump(fn_state.exit_block_ref.clone());
            }
        }
    }
    ctx.emitter()
        .set_current_block(fn_state.exit_block_ref.clone());
    return ctx
        .emitter()
        .read_variable(fn_state.borealis_fn_return_value.clone());
    fn block_0(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b0_s0: read-var opcode:u32
        let b0_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b0_s1: write-var v__0:u32 <= b0_s0:u32
        ctx.emitter()
            .write_variable(fn_state.v__0.clone(), b0_s0.clone());
        // b0_s2: const #31s : i6
        let b0_s2 = ctx.emitter().constant(
            31i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b0_s3: const #1s : i6
        let b0_s3 = ctx.emitter().constant(
            1i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b0_s4: bit-extract b0_s0 b0_s2 b0_s3
        let b0_s4 = ctx
            .emitter()
            .bit_extract(b0_s0.clone(), b0_s2.clone(), b0_s3.clone());
        // b0_s5: cast trunc b0_s4 -> u1
        let b0_s5 = ctx.emitter().cast(
            b0_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
            CastOperationKind::Truncate,
        );
        // b0_s6: const #0u : u1
        let b0_s6 = ctx.emitter().constant(
            0u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b0_s7: cmp-eq b0_s5 b0_s6
        let b0_s7 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b0_s5.clone(),
                b0_s6.clone(),
            ));
        // b0_s8: branch b0_s7 blockRef { index: 2, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 4, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b0_s7.clone(),
            fn_state.block_refs[22usize].clone(),
            fn_state.block_refs[1usize].clone(),
        );
    }
    fn block_1(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b1_s0: read-var opcode:u32
        let b1_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b1_s1: write-var v__3:u32 <= b1_s0:u32
        ctx.emitter()
            .write_variable(fn_state.v__3.clone(), b1_s0.clone());
        // b1_s2: const #31s : i6
        let b1_s2 = ctx.emitter().constant(
            31i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b1_s3: const #1s : i6
        let b1_s3 = ctx.emitter().constant(
            1i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b1_s4: bit-extract b1_s0 b1_s2 b1_s3
        let b1_s4 = ctx
            .emitter()
            .bit_extract(b1_s0.clone(), b1_s2.clone(), b1_s3.clone());
        // b1_s5: cast trunc b1_s4 -> u1
        let b1_s5 = ctx.emitter().cast(
            b1_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
            CastOperationKind::Truncate,
        );
        // b1_s6: const #1u : u1
        let b1_s6 = ctx.emitter().constant(
            1u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b1_s7: cmp-eq b1_s5 b1_s6
        let b1_s7 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b1_s5.clone(),
                b1_s6.clone(),
            ));
        // b1_s8: branch b1_s7 blockRef { index: 5, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 7, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b1_s7.clone(),
            fn_state.block_refs[20usize].clone(),
            fn_state.block_refs[2usize].clone(),
        );
    }
    fn block_2(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b2_s0: read-var opcode:u32
        let b2_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b2_s1: const #25s : i6
        let b2_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b2_s2: const #4s : i6
        let b2_s2 = ctx.emitter().constant(
            4i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b2_s3: bit-extract b2_s0 b2_s1 b2_s2
        let b2_s3 = ctx
            .emitter()
            .bit_extract(b2_s0.clone(), b2_s1.clone(), b2_s2.clone());
        // b2_s4: cast trunc b2_s3 -> u4
        let b2_s4 = ctx.emitter().cast(
            b2_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
            CastOperationKind::Truncate,
        );
        // b2_s5: const #1u : u4
        let b2_s5 = ctx.emitter().constant(
            1u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
        );
        // b2_s6: cmp-eq b2_s4 b2_s5
        let b2_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b2_s4.clone(),
                b2_s5.clone(),
            ));
        // b2_s7: not b2_s6
        let b2_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b2_s6.clone()));
        // b2_s8: branch b2_s7 blockRef { index: 8, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 26, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b2_s7.clone(),
            fn_state.block_refs[6usize].clone(),
            fn_state.block_refs[3usize].clone(),
        );
    }
    fn block_3(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b3_s0: read-var pc:i
        let b3_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b3_s1: read-var opcode:u32
        let b3_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b3_s2: call __DecodeA64_Unallocated1(b3_s0, b3_s1)
        let b3_s2 = u__DecodeA64_Unallocated1(ctx, b3_s0, b3_s1);
        // b3_s3: write-var gs#249876:() <= b3_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b3_s2.clone());
        // b3_s4: const #26872u : u32
        let b3_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b3_s5: read-reg b3_s4:u1
        let b3_s5 = ctx.emitter().read_register(
            b3_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b3_s6: branch b3_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b3_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_4(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b4_s0: read-var gs#249876:()
        let b4_s0 = ctx.emitter().read_variable(fn_state.gs_249876.clone());
        // b4_s1: write-var return:() <= b4_s0:()
        ctx.emitter()
            .write_variable(fn_state.return_.clone(), b4_s0.clone());
        // b4_s2: return b4_s0
        ctx.emitter()
            .write_variable(fn_state.borealis_fn_return_value.clone(), b4_s0);
        return BlockResult::Return;
    }
    fn block_5(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b5_s0: const #"undefined terminator" : str
        let b5_s0 = "undefined terminator";
        // b5_s1: panic b5_s0
        ctx.emitter().panic(b5_s0);
        return BlockResult::Panic;
    }
    fn block_6(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b6_s0: read-var opcode:u32
        let b6_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b6_s1: const #25s : i6
        let b6_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b6_s2: const #4s : i6
        let b6_s2 = ctx.emitter().constant(
            4i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b6_s3: bit-extract b6_s0 b6_s1 b6_s2
        let b6_s3 = ctx
            .emitter()
            .bit_extract(b6_s0.clone(), b6_s1.clone(), b6_s2.clone());
        // b6_s4: cast trunc b6_s3 -> u4
        let b6_s4 = ctx.emitter().cast(
            b6_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
            CastOperationKind::Truncate,
        );
        // b6_s5: const #2u : u4
        let b6_s5 = ctx.emitter().constant(
            2u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
        );
        // b6_s6: cmp-eq b6_s4 b6_s5
        let b6_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b6_s4.clone(),
                b6_s5.clone(),
            ));
        // b6_s7: not b6_s6
        let b6_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b6_s6.clone()));
        // b6_s8: branch b6_s7 blockRef { index: 9, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 25, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b6_s7.clone(),
            fn_state.block_refs[8usize].clone(),
            fn_state.block_refs[7usize].clone(),
        );
    }
    fn block_7(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b7_s0: read-var pc:i
        let b7_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b7_s1: read-var opcode:u32
        let b7_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b7_s2: call __DecodeA64_SVE(b7_s0, b7_s1)
        let b7_s2 = u__DecodeA64_SVE(ctx, b7_s0, b7_s1);
        // b7_s3: write-var gs#249876:() <= b7_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b7_s2.clone());
        // b7_s4: const #26872u : u32
        let b7_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b7_s5: read-reg b7_s4:u1
        let b7_s5 = ctx.emitter().read_register(
            b7_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b7_s6: branch b7_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b7_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_8(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b8_s0: read-var opcode:u32
        let b8_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b8_s1: const #25s : i6
        let b8_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b8_s2: const #4s : i6
        let b8_s2 = ctx.emitter().constant(
            4i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b8_s3: bit-extract b8_s0 b8_s1 b8_s2
        let b8_s3 = ctx
            .emitter()
            .bit_extract(b8_s0.clone(), b8_s1.clone(), b8_s2.clone());
        // b8_s4: cast trunc b8_s3 -> u4
        let b8_s4 = ctx.emitter().cast(
            b8_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
            CastOperationKind::Truncate,
        );
        // b8_s5: const #3u : u4
        let b8_s5 = ctx.emitter().constant(
            3u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
        );
        // b8_s6: cmp-eq b8_s4 b8_s5
        let b8_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b8_s4.clone(),
                b8_s5.clone(),
            ));
        // b8_s7: not b8_s6
        let b8_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b8_s6.clone()));
        // b8_s8: branch b8_s7 blockRef { index: 10, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 24, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b8_s7.clone(),
            fn_state.block_refs[10usize].clone(),
            fn_state.block_refs[9usize].clone(),
        );
    }
    fn block_9(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b9_s0: read-var pc:i
        let b9_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b9_s1: read-var opcode:u32
        let b9_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b9_s2: call __DecodeA64_Unallocated2(b9_s0, b9_s1)
        let b9_s2 = u__DecodeA64_Unallocated2(ctx, b9_s0, b9_s1);
        // b9_s3: write-var gs#249876:() <= b9_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b9_s2.clone());
        // b9_s4: const #26872u : u32
        let b9_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b9_s5: read-reg b9_s4:u1
        let b9_s5 = ctx.emitter().read_register(
            b9_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b9_s6: branch b9_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b9_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_10(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b10_s0: read-var opcode:u32
        let b10_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b10_s1: const #26s : i6
        let b10_s1 = ctx.emitter().constant(
            26i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b10_s2: const #3s : i6
        let b10_s2 = ctx.emitter().constant(
            3i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b10_s3: bit-extract b10_s0 b10_s1 b10_s2
        let b10_s3 = ctx
            .emitter()
            .bit_extract(b10_s0.clone(), b10_s1.clone(), b10_s2.clone());
        // b10_s4: cast trunc b10_s3 -> u3
        let b10_s4 = ctx.emitter().cast(
            b10_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 3,
            },
            CastOperationKind::Truncate,
        );
        // b10_s5: const #4u : u3
        let b10_s5 = ctx.emitter().constant(
            4u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 3,
            },
        );
        // b10_s6: cmp-eq b10_s4 b10_s5
        let b10_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b10_s4.clone(),
                b10_s5.clone(),
            ));
        // b10_s7: not b10_s6
        let b10_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b10_s6.clone()));
        // b10_s8: branch b10_s7 blockRef { index: 11, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 23, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b10_s7.clone(),
            fn_state.block_refs[12usize].clone(),
            fn_state.block_refs[11usize].clone(),
        );
    }
    fn block_11(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b11_s0: read-var pc:i
        let b11_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b11_s1: read-var opcode:u32
        let b11_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b11_s2: call __DecodeA64_DataProcImm(b11_s0, b11_s1)
        let b11_s2 = u__DecodeA64_DataProcImm(ctx, b11_s0, b11_s1);
        // b11_s3: write-var gs#249876:() <= b11_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b11_s2.clone());
        // b11_s4: const #26872u : u32
        let b11_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b11_s5: read-reg b11_s4:u1
        let b11_s5 = ctx.emitter().read_register(
            b11_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b11_s6: branch b11_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b11_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_12(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b12_s0: read-var opcode:u32
        let b12_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b12_s1: const #26s : i6
        let b12_s1 = ctx.emitter().constant(
            26i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b12_s2: const #3s : i6
        let b12_s2 = ctx.emitter().constant(
            3i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b12_s3: bit-extract b12_s0 b12_s1 b12_s2
        let b12_s3 = ctx
            .emitter()
            .bit_extract(b12_s0.clone(), b12_s1.clone(), b12_s2.clone());
        // b12_s4: cast trunc b12_s3 -> u3
        let b12_s4 = ctx.emitter().cast(
            b12_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 3,
            },
            CastOperationKind::Truncate,
        );
        // b12_s5: const #5u : u3
        let b12_s5 = ctx.emitter().constant(
            5u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 3,
            },
        );
        // b12_s6: cmp-eq b12_s4 b12_s5
        let b12_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b12_s4.clone(),
                b12_s5.clone(),
            ));
        // b12_s7: not b12_s6
        let b12_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b12_s6.clone()));
        // b12_s8: branch b12_s7 blockRef { index: 12, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 22, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b12_s7.clone(),
            fn_state.block_refs[14usize].clone(),
            fn_state.block_refs[13usize].clone(),
        );
    }
    fn block_13(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b13_s0: read-var pc:i
        let b13_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b13_s1: read-var opcode:u32
        let b13_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b13_s2: call __DecodeA64_BranchExcSys(b13_s0, b13_s1)
        let b13_s2 = u__DecodeA64_BranchExcSys(ctx, b13_s0, b13_s1);
        // b13_s3: write-var gs#249876:() <= b13_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b13_s2.clone());
        // b13_s4: const #26872u : u32
        let b13_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b13_s5: read-reg b13_s4:u1
        let b13_s5 = ctx.emitter().read_register(
            b13_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b13_s6: branch b13_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b13_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_14(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b14_s0: read-var opcode:u32
        let b14_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b14_s1: write-var v__21:u32 <= b14_s0:u32
        ctx.emitter()
            .write_variable(fn_state.v__21.clone(), b14_s0.clone());
        // b14_s2: const #27s : i6
        let b14_s2 = ctx.emitter().constant(
            27i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b14_s3: const #1s : i6
        let b14_s3 = ctx.emitter().constant(
            1i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b14_s4: bit-extract b14_s0 b14_s2 b14_s3
        let b14_s4 = ctx
            .emitter()
            .bit_extract(b14_s0.clone(), b14_s2.clone(), b14_s3.clone());
        // b14_s5: cast trunc b14_s4 -> u1
        let b14_s5 = ctx.emitter().cast(
            b14_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
            CastOperationKind::Truncate,
        );
        // b14_s6: const #1u : u1
        let b14_s6 = ctx.emitter().constant(
            1u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b14_s7: cmp-eq b14_s5 b14_s6
        let b14_s7 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b14_s5.clone(),
                b14_s6.clone(),
            ));
        // b14_s8: branch b14_s7 blockRef { index: 13, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 15, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b14_s7.clone(),
            fn_state.block_refs[18usize].clone(),
            fn_state.block_refs[15usize].clone(),
        );
    }
    fn block_15(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b15_s0: read-var opcode:u32
        let b15_s0 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b15_s1: const #25s : i6
        let b15_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b15_s2: const #3s : i6
        let b15_s2 = ctx.emitter().constant(
            3i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b15_s3: bit-extract b15_s0 b15_s1 b15_s2
        let b15_s3 = ctx
            .emitter()
            .bit_extract(b15_s0.clone(), b15_s1.clone(), b15_s2.clone());
        // b15_s4: cast trunc b15_s3 -> u3
        let b15_s4 = ctx.emitter().cast(
            b15_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 3,
            },
            CastOperationKind::Truncate,
        );
        // b15_s5: const #5u : u3
        let b15_s5 = ctx.emitter().constant(
            5u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 3,
            },
        );
        // b15_s6: cmp-eq b15_s4 b15_s5
        let b15_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b15_s4.clone(),
                b15_s5.clone(),
            ));
        // b15_s7: not b15_s6
        let b15_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b15_s6.clone()));
        // b15_s8: branch b15_s7 blockRef { index: 16, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 19, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b15_s7.clone(),
            fn_state.block_refs[17usize].clone(),
            fn_state.block_refs[16usize].clone(),
        );
    }
    fn block_16(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b16_s0: read-var pc:i
        let b16_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b16_s1: read-var opcode:u32
        let b16_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b16_s2: call __DecodeA64_DataProcReg(b16_s0, b16_s1)
        let b16_s2 = u__DecodeA64_DataProcReg(ctx, b16_s0, b16_s1);
        // b16_s3: write-var gs#249876:() <= b16_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b16_s2.clone());
        // b16_s4: const #26872u : u32
        let b16_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b16_s5: read-reg b16_s4:u1
        let b16_s5 = ctx.emitter().read_register(
            b16_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b16_s6: branch b16_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b16_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_17(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b17_s0: read-var pc:i
        let b17_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b17_s1: read-var opcode:u32
        let b17_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b17_s2: call __DecodeA64_DataProcFPSIMD(b17_s0, b17_s1)
        let b17_s2 = u__DecodeA64_DataProcFPSIMD(ctx, b17_s0, b17_s1);
        // b17_s3: write-var gs#249876:() <= b17_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b17_s2.clone());
        // b17_s4: const #26872u : u32
        let b17_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b17_s5: read-reg b17_s4:u1
        let b17_s5 = ctx.emitter().read_register(
            b17_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b17_s6: branch b17_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b17_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_18(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b18_s0: read-var v__21:u32
        let b18_s0 = ctx.emitter().read_variable(fn_state.v__21.clone());
        // b18_s1: const #25s : i6
        let b18_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b18_s2: const #1s : i6
        let b18_s2 = ctx.emitter().constant(
            1i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b18_s3: bit-extract b18_s0 b18_s1 b18_s2
        let b18_s3 = ctx
            .emitter()
            .bit_extract(b18_s0.clone(), b18_s1.clone(), b18_s2.clone());
        // b18_s4: cast trunc b18_s3 -> u1
        let b18_s4 = ctx.emitter().cast(
            b18_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
            CastOperationKind::Truncate,
        );
        // b18_s5: const #0u : u1
        let b18_s5 = ctx.emitter().constant(
            0u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b18_s6: cmp-eq b18_s4 b18_s5
        let b18_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b18_s4.clone(),
                b18_s5.clone(),
            ));
        // b18_s7: not b18_s6
        let b18_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b18_s6.clone()));
        // b18_s8: branch b18_s7 blockRef { index: 15, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 20, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b18_s7.clone(),
            fn_state.block_refs[15usize].clone(),
            fn_state.block_refs[19usize].clone(),
        );
    }
    fn block_19(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b19_s0: read-var pc:i
        let b19_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b19_s1: read-var opcode:u32
        let b19_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b19_s2: call __DecodeA64_LoadStore(b19_s0, b19_s1)
        let b19_s2 = u__DecodeA64_LoadStore(ctx, b19_s0, b19_s1);
        // b19_s3: write-var gs#249876:() <= b19_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b19_s2.clone());
        // b19_s4: const #26872u : u32
        let b19_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b19_s5: read-reg b19_s4:u1
        let b19_s5 = ctx.emitter().read_register(
            b19_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b19_s6: branch b19_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b19_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_20(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b20_s0: read-var v__3:u32
        let b20_s0 = ctx.emitter().read_variable(fn_state.v__3.clone());
        // b20_s1: const #25s : i6
        let b20_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b20_s2: const #4s : i6
        let b20_s2 = ctx.emitter().constant(
            4i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b20_s3: bit-extract b20_s0 b20_s1 b20_s2
        let b20_s3 = ctx
            .emitter()
            .bit_extract(b20_s0.clone(), b20_s1.clone(), b20_s2.clone());
        // b20_s4: cast trunc b20_s3 -> u4
        let b20_s4 = ctx.emitter().cast(
            b20_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
            CastOperationKind::Truncate,
        );
        // b20_s5: const #0u : u4
        let b20_s5 = ctx.emitter().constant(
            0u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
        );
        // b20_s6: cmp-eq b20_s4 b20_s5
        let b20_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b20_s4.clone(),
                b20_s5.clone(),
            ));
        // b20_s7: not b20_s6
        let b20_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b20_s6.clone()));
        // b20_s8: branch b20_s7 blockRef { index: 7, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 27, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b20_s7.clone(),
            fn_state.block_refs[2usize].clone(),
            fn_state.block_refs[21usize].clone(),
        );
    }
    fn block_21(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b21_s0: read-var pc:i
        let b21_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b21_s1: read-var opcode:u32
        let b21_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b21_s2: call __DecodeA64_SME(b21_s0, b21_s1)
        let b21_s2 = u__DecodeA64_SME(ctx, b21_s0, b21_s1);
        // b21_s3: write-var gs#249876:() <= b21_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b21_s2.clone());
        // b21_s4: const #26872u : u32
        let b21_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b21_s5: read-reg b21_s4:u1
        let b21_s5 = ctx.emitter().read_register(
            b21_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b21_s6: branch b21_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b21_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
    fn block_22(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b22_s0: read-var v__0:u32
        let b22_s0 = ctx.emitter().read_variable(fn_state.v__0.clone());
        // b22_s1: const #25s : i6
        let b22_s1 = ctx.emitter().constant(
            25i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b22_s2: const #4s : i6
        let b22_s2 = ctx.emitter().constant(
            4i64 as u64,
            Type {
                kind: TypeKind::Signed,
                width: 6,
            },
        );
        // b22_s3: bit-extract b22_s0 b22_s1 b22_s2
        let b22_s3 = ctx
            .emitter()
            .bit_extract(b22_s0.clone(), b22_s1.clone(), b22_s2.clone());
        // b22_s4: cast trunc b22_s3 -> u4
        let b22_s4 = ctx.emitter().cast(
            b22_s3.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
            CastOperationKind::Truncate,
        );
        // b22_s5: const #0u : u4
        let b22_s5 = ctx.emitter().constant(
            0u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 4,
            },
        );
        // b22_s6: cmp-eq b22_s4 b22_s5
        let b22_s6 = ctx
            .emitter()
            .binary_operation(BinaryOperationKind::CompareEqual(
                b22_s4.clone(),
                b22_s5.clone(),
            ));
        // b22_s7: not b22_s6
        let b22_s7 = ctx
            .emitter()
            .unary_operation(UnaryOperationKind::Not(b22_s6.clone()));
        // b22_s8: branch b22_s7 blockRef { index: 4, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 29, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b22_s7.clone(),
            fn_state.block_refs[1usize].clone(),
            fn_state.block_refs[23usize].clone(),
        );
    }
    fn block_23(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult {
        // b23_s0: read-var pc:i
        let b23_s0 = ctx.emitter().read_variable(fn_state.pc.clone());
        // b23_s1: read-var opcode:u32
        let b23_s1 = ctx.emitter().read_variable(fn_state.opcode.clone());
        // b23_s2: call __DecodeA64_Reserved(b23_s0, b23_s1)
        let b23_s2 = u__DecodeA64_Reserved(ctx, b23_s0, b23_s1);
        // b23_s3: write-var gs#249876:() <= b23_s2:()
        ctx.emitter()
            .write_variable(fn_state.gs_249876.clone(), b23_s2.clone());
        // b23_s4: const #26872u : u32
        let b23_s4 = ctx.emitter().constant(
            26872u64,
            Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        );
        // b23_s5: read-reg b23_s4:u1
        let b23_s5 = ctx.emitter().read_register(
            b23_s4.clone(),
            Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
        );
        // b23_s6: branch b23_s5 blockRef { index: 17, _phantom:
        // PhantomData<borealis::rudder::Block> } blockRef { index: 18, _phantom:
        // PhantomData<borealis::rudder::Block> }
        return ctx.emitter().branch(
            b23_s5.clone(),
            fn_state.block_refs[5usize].clone(),
            fn_state.block_refs[4usize].clone(),
        );
    }
}
